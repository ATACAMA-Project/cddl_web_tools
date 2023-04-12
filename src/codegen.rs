use cddl_codegen::cli::Cli;
use cddl_codegen::generation::GenerationScope;
use cddl_codegen::intermediate::{CDDLIdent, IntermediateTypes, RustIdent};
use cddl_codegen::parsing::{parse_rule, rule_ident, rule_is_scope_marker};
use cddl_codegen::{dep_graph, parsing};
use std::ffi::OsString;
use std::fs::File;
use std::io::{Read, Seek, Write};
use std::path::Path;
use walkdir::{DirEntry, WalkDir};
use zip::result::{ZipError, ZipResult};
use zip::write::FileOptions;

pub const GEN_ZIP_FILE: &str = "gen.zip";

pub fn generate_code(root: &Path, cddl_str: &str, args: &mut Cli) -> Result<OsString, ZipError> {
    let gen_path = root.join("gen");
    args.output = gen_path.clone();

    let mut input_files_content = format!(
        "\n{}{} = \"{}\"\n{}\n",
        parsing::SCOPE_MARKER,
        0,
        "lib",
        cddl_str
    );
    // we also need to mark the extern marker to a placeholder struct that won't get codegened
    input_files_content.push_str(&format!("{} = [0]", parsing::EXTERN_MARKER));
    // and a raw bytes one too
    input_files_content.push_str(&format!("{} = [1]", parsing::RAW_BYTES_MARKER));

    // Plain group / scope marking
    let cddl = cddl::parser::cddl_from_str(&input_files_content, true).unwrap();
    //panic!("cddl: {:#.unwrap()}", cddl);
    let pv = cddl::ast::parent::ParentVisitor::new(&cddl).unwrap();
    let mut types = IntermediateTypes::new();
    // mark scope and filter scope markers
    let mut scope = "lib".to_owned();
    let cddl_rules = cddl
        .rules
        .iter()
        .filter(|cddl_rule| {
            // We inserted string constants with specific prefixes earlier to mark scope
            if let Some(new_scope) = rule_is_scope_marker(cddl_rule) {
                println!("Switching from scope '{scope}' to '{new_scope}'");
                scope = new_scope;
                false
            } else {
                let ident = rule_ident(cddl_rule);
                types.mark_scope(ident, scope.clone());
                true
            }
        })
        .collect::<Vec<_>>();
    // We need to know beforehand which are plain groups so we can serialize them properly
    // e.g. x = (3, 4), y = [1, x, 2] should be [1, 3, 4, 2] instead of [1, [3, 4], 2]
    for cddl_rule in cddl_rules.iter() {
        if let cddl::ast::Rule::Group { rule, .. } = cddl_rule {
            // Freely defined group - no need to generate anything outside of group module
            match &rule.entry {
                cddl::ast::GroupEntry::InlineGroup { group, .. } => {
                    types.mark_plain_group(
                        RustIdent::new(CDDLIdent::new(rule.name.to_string())),
                        Some(group.clone()),
                    );
                }
                x => panic!("Group rule with non-inline group.unwrap() {:?}", x),
            }
        }
    }

    // Creating intermediate form from the CDDL
    for cddl_rule in dep_graph::topological_rule_order(&cddl_rules) {
        println!("\n\n------------------------------------------\n- Handling rule: {}:{}\n------------------------------------", scope, cddl_rule.name());
        parse_rule(&mut types, &pv, cddl_rule, args);
    }
    types.finalize(&pv, args);

    // Generating code from intermediate form
    println!("\n-----------------------------------------\n- Generating code...\n------------------------------------");
    let mut gen_scope = GenerationScope::new();
    gen_scope.generate(&types, args);
    gen_scope.export(&types, args).unwrap();
    types.print_info();

    gen_scope.print_structs_without_deserialize();

    let gen_zip = root.join(GEN_ZIP_FILE);
    let _ = doit(
        gen_path.to_str().unwrap(),
        gen_zip.to_str().unwrap(),
        zip::CompressionMethod::Deflated,
    )?;

    Ok(gen_zip.into_os_string())
}

// Copied from https://github.com/zip-rs/zip/blob/master/examples/write_dir.rs

fn zip_dir<T>(
    it: &mut dyn Iterator<Item = DirEntry>,
    prefix: &str,
    writer: T,
    method: zip::CompressionMethod,
) -> ZipResult<T>
where
    T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            println!("adding file {path:?} as {name:?} ...");
            #[allow(deprecated)]
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            println!("adding dir {path:?} as {name:?} ...");
            #[allow(deprecated)]
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()
}

fn doit(src_dir: &str, dst_file: &str, method: zip::CompressionMethod) -> ZipResult<File> {
    if !Path::new(src_dir).is_dir() {
        return Err(ZipError::FileNotFound);
    }

    let path = Path::new(dst_file);
    let file = File::create(path).unwrap();

    let walkdir = WalkDir::new(src_dir);
    let it = walkdir.into_iter();

    zip_dir(&mut it.filter_map(|e| e.ok()), src_dir, file, method)
}
