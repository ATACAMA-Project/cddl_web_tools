use cddl::{cddl_from_str, validate_cbor_from_slice, validate_json_from_str};
use cddl_cat::{parse_cddl as parse_cat, validate_cbor_bytes, validate_json_str, ValidateResult};
use cuddle::{cddl::Cddl, parse_cddl as parse_cuddle};

pub enum ValidationLibrary {
    Cddl,
    CddlCat,
    Cuddle,
}

pub enum ValidationType {
    Plain(String),
    WithJson(String, String),
    WithCbor(String, Vec<u8>),
}

static FILENAME: &str = "cddl.cddl";

pub fn validate(library: ValidationLibrary, validation_type: ValidationType) -> Result<(), String> {
    match library {
        ValidationLibrary::Cddl => match validation_type {
            ValidationType::Plain(cddl_str) => cddl_from_str(&cddl_str, true).map(|_| ()),
            ValidationType::WithJson(cddl_str, json_str) => {
                validate_json_from_str(&cddl_str, &json_str, None).map_err(|e| e.to_string())
            }
            ValidationType::WithCbor(cddl_str, cbor_bytes) => {
                validate_cbor_from_slice(&cddl_str, &cbor_bytes, None).map_err(|e| e.to_string())
            }
        },
        ValidationLibrary::CddlCat => match validation_type {
            ValidationType::Plain(cddl_str) => {
                parse_cat(&cddl_str).map(|_| ()).map_err(|e| e.to_string())
            }
            ValidationType::WithJson(cddl_str, json_str) => {
                cddl_cat_validate_against_data(&cddl_str, |name| {
                    validate_json_str(name, &cddl_str, &json_str)
                })
            }
            ValidationType::WithCbor(cddl_str, cbor_bytes) => {
                cddl_cat_validate_against_data(&cddl_str, |name| {
                    validate_cbor_bytes(name, &cddl_str, &cbor_bytes)
                })
            }
        },
        ValidationLibrary::Cuddle => match validation_type {
            ValidationType::Plain(cddl_str) => parse_cuddle(&cddl_str, FILENAME)
                .map(|_| ())
                .map_err(|e| e.to_string()),
            ValidationType::WithJson(..) => {
                Err("Cuddle does not support JSON validation".to_string())
            }
            ValidationType::WithCbor(cddl_str, cbor_bytes) => {
                let cddl_root = parse_cuddle(&cddl_str, FILENAME).map_err(|e| e.to_string())?;
                let cddl = Cddl::from_cddl_root(&cddl_root).map_err(|e| e.to_string())?;
                cddl.validate_cbor(cbor_bytes).map_err(|e| e.to_string())
            }
        },
    }
}

fn cddl_cat_validate_against_data<F>(input: &str, f: F) -> Result<(), String>
where
    F: Fn(&str) -> ValidateResult,
{
    parse_cat(input)
        .unwrap()
        .rules
        .iter()
        .find(|r| f(&r.name).is_ok())
        .ok_or_else(|| "No matching rule found".to_string())
        .map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;

    const CDDL: &str = include_str!("../tests/cddl.cddl");
    const JSON: &str = include_str!("../tests/cddl.json");

    #[test]
    fn cddl_plain() {
        validate(
            ValidationLibrary::Cddl,
            ValidationType::Plain(CDDL.to_string()),
        )
        .unwrap();
    }

    #[test]
    fn cat_plain() {
        validate(
            ValidationLibrary::CddlCat,
            ValidationType::Plain(CDDL.to_string()),
        )
        .unwrap();
    }

    #[test]
    fn cuddle_plain() {
        validate(
            ValidationLibrary::Cuddle,
            ValidationType::Plain(CDDL.to_string()),
        )
        .unwrap();
    }

    #[test]
    fn cddl_with_json() {
        validate(
            ValidationLibrary::Cddl,
            ValidationType::WithJson(CDDL.to_string(), JSON.to_string()),
        )
        .unwrap();
    }

    #[test]
    fn cddlcat_with_json() {
        validate(
            ValidationLibrary::CddlCat,
            ValidationType::WithJson(CDDL.to_string(), JSON.to_string()),
        )
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn cuddle_with_json() {
        validate(
            ValidationLibrary::Cuddle,
            ValidationType::WithJson(CDDL.to_string(), JSON.to_string()),
        )
        .unwrap();
    }
}
