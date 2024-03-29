use cddl::{cddl_from_str, validate_cbor_from_slice, validate_json_from_str};
use cddl_cat::{parse_cddl as parse_cat, validate_cbor_bytes, validate_json_str, ValidateResult};
use cuddle::parse_cddl as parse_cuddle;
use once_cell::sync::Lazy;
use std::path::Path;

#[non_exhaustive]
#[derive(FromFormField, Clone)]
pub enum ValidationLibrary {
    Cddl,
    CddlCat,
    Cuddle,
}

impl std::fmt::Display for ValidationLibrary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationLibrary::Cddl => write!(f, "cddl"),
            ValidationLibrary::CddlCat => write!(f, "cddl-cat"),
            ValidationLibrary::Cuddle => write!(f, "cuddle"),
        }
    }
}

#[non_exhaustive]
#[derive(Clone)]
pub enum ValidationType {
    Plain(String),
    WithJson(String, String),
    WithCbor(String, Vec<u8>),
}

static FILEPATH: Lazy<&Path> = Lazy::new(|| Path::new("cddl.cddl"));

pub fn validate_all(validation_type: ValidationType) -> Vec<(String, String)> {
    let libraries = if let ValidationType::Plain(..) = validation_type {
        vec![
            ValidationLibrary::Cddl,
            ValidationLibrary::CddlCat,
            ValidationLibrary::Cuddle,
        ]
    } else {
        vec![ValidationLibrary::Cddl, ValidationLibrary::CddlCat]
    };

    libraries
        .iter()
        .filter_map(
            |library| match validate(library.clone(), validation_type.clone()) {
                Err(err) => Some((library.to_string(), err)),
                Ok(_) => None,
            },
        )
        .collect()
}

pub fn validate(library: ValidationLibrary, validation_type: ValidationType) -> Result<(), String> {
    match library {
        ValidationLibrary::Cddl => match validation_type {
            ValidationType::Plain(cddl_str) => cddl_from_str(&cddl_str, false).map(|_| ()),
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
            ValidationType::Plain(cddl_str) => parse_cuddle(&cddl_str, *FILEPATH)
                .map(|_| ())
                .map_err(|e| e.to_string()),
            ValidationType::WithJson(..) => {
                Err("Cuddle does not support JSON validation".to_string())
            }
            ValidationType::WithCbor(..) => {
                Err("Cuddle does not support CBOR validation".to_string())
            }
        },
    }
}

fn cddl_cat_validate_against_data<F>(input: impl AsRef<str>, f: F) -> Result<(), String>
where
    F: Fn(&str) -> ValidateResult,
{
    let cddl = parse_cat(input.as_ref()).map_err(|e| e.to_string())?;

    if cddl.rules.iter().any(|r| f(&r.name).is_ok()) {
        Ok(())
    } else {
        Err("No matching rule found".to_string())
    }
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
