use cddl::{cddl_from_str, validate_cbor_from_slice, validate_json_from_str};
use cddl_cat::{parse_cddl as parse_cat, validate_cbor_bytes, validate_json_str};
use cuddle::{cddl::Cddl, parse_cddl as parse_cuddle};

pub struct ValidationData {
    cddl: String,
    json: Option<String>,
    cbor: Option<Vec<u8>>,
}

impl ValidationData {
    pub fn new(cddl: String, json: Option<String>, cbor: Option<Vec<u8>>) -> Self {
        Self { cddl, json, cbor }
    }
}

pub enum ValidationLibrary {
    Cddl,
    CddlCat,
    Cuddle,
}

pub enum ValidationType {
    Plain,
    WithJson,
    WithCbor,
}

static FILENAME: &str = "cddl";

pub fn validate(
    data: ValidationData,
    library: ValidationLibrary,
    validation_type: ValidationType,
) -> Result<(), String> {
    let cddl_str = data.cddl.as_ref();
    let json_str = data.json.as_ref();
    let cbor_bytes = data.cbor.as_ref();
    match library {
        ValidationLibrary::Cddl => match validation_type {
            ValidationType::Plain => cddl_from_str(cddl_str, true).map(|_| ()),
            ValidationType::WithJson => {
                validate_json_from_str(cddl_str, json_str.unwrap(), None).map_err(|e| e.to_string())
            }
            ValidationType::WithCbor => {
                validate_cbor_from_slice(cddl_str, cbor_bytes.unwrap(), None)
                    .map_err(|e| e.to_string())
            }
        },
        ValidationLibrary::CddlCat => match validation_type {
            ValidationType::Plain => parse_cat(cddl_str).map(|_| ()).map_err(|e| e.to_string()),
            ValidationType::WithJson => {
                validate_json_str("", cddl_str, json_str.unwrap()).map_err(|e| e.to_string())
            }
            ValidationType::WithCbor => {
                validate_cbor_bytes("", cddl_str, cbor_bytes.unwrap()).map_err(|e| e.to_string())
            }
        },
        ValidationLibrary::Cuddle => match validation_type {
            ValidationType::Plain => parse_cuddle(cddl_str, FILENAME)
                .map(|_| ())
                .map_err(|e| e.to_string()),
            ValidationType::WithJson => Err("Cuddle does not support JSON validation".to_string()),
            ValidationType::WithCbor => {
                let cddl_root = parse_cuddle(cddl_str, FILENAME).map_err(|e| e.to_string())?;
                let cddl = Cddl::from_cddl_root(&cddl_root).map_err(|e| e.to_string())?;
                cddl.validate_cbor(cbor_bytes.unwrap())
                    .map_err(|e| e.to_string())
            }
        },
    }
}
