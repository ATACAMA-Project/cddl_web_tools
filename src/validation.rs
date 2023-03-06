use cddl::{cddl_from_str, validate_cbor_from_slice, validate_json_from_str};
use cddl_cat::{parse_cddl as parse_cat, validate_cbor_bytes, validate_json_str, ValidateResult};
use cuddle::{cddl::Cddl, parse_cddl as parse_cuddle};
use std::any::Any;

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
            ValidationType::WithJson => cddl_cat_validate_against_data(cddl_str, |name| {
                validate_json_str(name, cddl_str, json_str.unwrap())
            }),
            ValidationType::WithCbor => cddl_cat_validate_against_data(cddl_str, |name| {
                validate_cbor_bytes(name, cddl_str, cbor_bytes.unwrap())
            }),
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

fn cddl_cat_validate_against_data<F>(input: &str, f: F) -> Result<(), String>
where
    F: Fn(&str) -> ValidateResult,
{
    parse_cat(input)
        .unwrap()
        .rules
        .iter()
        .find(|r| f(&*r.name).is_ok())
        .ok_or_else(|| "No matching rule found".to_string())
        .map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;

    const CDDL: &str = "reputation-object = {
  application: text
  reputons: [* reputon]
}

reputon = {
  rater: text
  assertion: text
  rated: text
  rating: float16
  ? confidence: float16
  ? normal-rating: float16
  ? sample-size: uint
  ? generated: uint
  ? expires: uint
  * text => any
}";
    const JSON: &str = r#"{
  "application": "conchometry",
  "reputons": [
    {
      "rater": "Ephthianura",
      "assertion": "codding",
      "rated": "sphaerolitic",
      "rating": 0.34133473256800795,
      "confidence": 0.9481983064298332,
      "expires": 1568,
      "unplaster": "grassy"
    },
    {
      "rater": "nonchargeable",
      "assertion": "raglan",
      "rated": "alienage",
      "rating": 0.5724646875815566,
      "sample-size": 3514,
      "Aldebaran": "unchurched",
      "puruloid": "impersonable",
      "uninfracted": "pericarpoidal",
      "schorl": "Caro"
    },
    {
      "rater": "precollectable",
      "assertion": "Merat",
      "rated": "thermonatrite",
      "rating": 0.19164006323936977,
      "confidence": 0.6065252103391268,
      "normal-rating": 0.5187773690879303,
      "generated": 899,
      "speedy": "solidungular",
      "noviceship": "medicine",
      "checkrow": "epidictic"
    }
  ]
}"#;

    #[test]
    fn cddl_plain() {
        validate(
            ValidationData::new(CDDL.to_string(), None, None),
            ValidationLibrary::Cddl,
            ValidationType::Plain,
        )
        .unwrap();
    }

    #[test]
    fn cat_plain() {
        validate(
            ValidationData::new(CDDL.to_string(), None, None),
            ValidationLibrary::CddlCat,
            ValidationType::Plain,
        )
        .unwrap();
    }

    #[test]
    fn cuddle_plain() {
        validate(
            ValidationData::new(CDDL.to_string(), None, None),
            ValidationLibrary::Cuddle,
            ValidationType::Plain,
        )
        .unwrap();
    }

    #[test]
    fn cddl_with_json() {
        validate(
            ValidationData::new(CDDL.to_string(), Some(JSON.to_string()), None),
            ValidationLibrary::Cddl,
            ValidationType::WithJson,
        )
        .unwrap();
    }

    #[test]
    fn cddlcat_with_json() {
        validate(
            ValidationData::new(CDDL.to_string(), Some(JSON.to_string()), None),
            ValidationLibrary::CddlCat,
            ValidationType::WithJson,
        )
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn cuddle_with_json() {
        validate(
            ValidationData::new(CDDL.to_string(), Some(JSON.to_string()), None),
            ValidationLibrary::Cuddle,
            ValidationType::WithJson,
        )
        .unwrap();
    }
}
