// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Generates Rust wire types from the authoritative E02 JSON Schema (FR-002).

use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

use schemars::schema::RootSchema;
use serde_json::Value;
use typify::{TypeSpace, TypeSpaceSettings};

const E02_SCHEMA_PATH: &str = "contracts/e02-draft-2.schema.json";
const SHARED_V1_SCHEMA_PATH: &str = "contracts/shared-reference-1-draft.schema.json";
const SHARED_V2_SCHEMA_PATH: &str = "contracts/shared-reference-2-draft/schema.json";

fn main() -> Result<(), Box<dyn Error>> {
    for path in [
        E02_SCHEMA_PATH,
        SHARED_V1_SCHEMA_PATH,
        SHARED_V2_SCHEMA_PATH,
    ] {
        println!("cargo:rerun-if-changed={path}");
    }

    let authored_schema: Value = serde_json::from_str(&fs::read_to_string(E02_SCHEMA_PATH)?)?;
    let contract_version = authored_schema
        .pointer("/$defs/TechniqueDefinition/properties/contractVersion/const")
        .and_then(Value::as_str)
        .ok_or("schema contract version is missing")?
        .to_owned();
    let input_kinds = authored_schema
        .pointer("/$defs/Input/oneOf")
        .and_then(Value::as_array)
        .ok_or("schema input kinds are missing")?
        .iter()
        .map(|entry| {
            entry
                .pointer("/properties/kind/const")
                .and_then(Value::as_str)
                .ok_or("schema input kind is missing")
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let constants = format!(
        "pub const CONTRACT_VERSION: &str = {contract_version:?}; pub const INPUT_KINDS: &[&str] = &{input_kinds:?};"
    );
    generate_types(E02_SCHEMA_PATH, "e02_contracts.rs", &constants)?;
    generate_types(SHARED_V1_SCHEMA_PATH, "shared_reference_v1.rs", "")?;
    generate_types(SHARED_V2_SCHEMA_PATH, "shared_reference_v2.rs", "")?;
    Ok(())
}

fn generate_types(
    schema_path: &str,
    output_name: &str,
    prefix: &str,
) -> Result<(), Box<dyn Error>> {
    let mut schema: Value = serde_json::from_str(&fs::read_to_string(schema_path)?)?;
    strip_non_constructive_keywords(&mut schema);
    if schema_path == SHARED_V2_SCHEMA_PATH {
        relax_alternative_digest_codegen(&mut schema)?;
    } else if schema_path == E02_SCHEMA_PATH {
        localize_shared_references_for_codegen(&mut schema);
    }
    let root: RootSchema = serde_json::from_value(schema)?;
    let mut settings = TypeSpaceSettings::default();
    settings.with_map_type("::std::collections::BTreeMap".to_owned());
    let mut type_space = TypeSpace::new(&settings);
    if schema_path == E02_SCHEMA_PATH {
        let mut shared: Value = serde_json::from_str(&fs::read_to_string(SHARED_V2_SCHEMA_PATH)?)?;
        strip_non_constructive_keywords(&mut shared);
        relax_alternative_digest_codegen(&mut shared)?;
        let shared: RootSchema = serde_json::from_value(shared)?;
        type_space.add_ref_types(shared.definitions)?;
    }
    type_space.add_root_schema(root)?;

    let output =
        PathBuf::from(env::var_os("OUT_DIR").ok_or("OUT_DIR is not set")?).join(output_name);
    fs::write(output, format!("{prefix} {}", type_space.to_stream()))?;
    Ok(())
}

// Typify 0.7 does not retrieve external schemas. For code generation only,
// resolve the authored URN to definitions loaded below from the exact retained
// schema. Runtime validation keeps the external reference and registry.
fn localize_shared_references_for_codegen(value: &mut Value) {
    match value {
        Value::Object(fields) => {
            const PREFIX: &str = "urn:ix:shared-reference:2-draft#/$defs/";
            let localized = fields
                .get("$ref")
                .and_then(Value::as_str)
                .and_then(|reference| reference.strip_prefix(PREFIX))
                .map(|definition| Value::String(format!("#/$defs/{definition}")));
            if let Some(localized) = localized {
                fields.insert("$ref".to_owned(), localized);
            }
            for child in fields.values_mut() {
                localize_shared_references_for_codegen(child);
            }
        }
        Value::Array(items) => {
            for child in items {
                localize_shared_references_for_codegen(child);
            }
        }
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
    }
}

// Typify unifies the digest property shared by the two CanonicalIdentity
// alternatives. Their selected wire syntaxes have different exact lengths, so
// retaining either branch's length/pattern here would make the other valid
// branch impossible to deserialize. The complete runtime schema remains the
// validator; generated construction retains the branch shape and domain enums.
fn relax_alternative_digest_codegen(schema: &mut Value) -> Result<(), Box<dyn Error>> {
    let alternatives = schema
        .pointer_mut("/$defs/CanonicalIdentity/oneOf")
        .and_then(Value::as_array_mut)
        .ok_or("canonical identity alternatives are missing")?;
    for alternative in alternatives {
        let digest = alternative
            .pointer_mut("/properties/digest")
            .and_then(Value::as_object_mut)
            .ok_or("canonical identity digest is missing")?;
        digest.remove("minLength");
        digest.remove("maxLength");
        digest.remove("pattern");
    }
    Ok(())
}

// Conditional JSON Schema assertions constrain valid values but cannot be
// represented directly as constructive Rust types. Runtime validation retains
// the complete authoritative schema, including these removed codegen keywords.
fn strip_non_constructive_keywords(value: &mut Value) {
    match value {
        Value::Object(fields) => {
            fields.remove("if");
            fields.remove("then");
            fields.remove("else");
            for child in fields.values_mut() {
                strip_non_constructive_keywords(child);
            }
        }
        Value::Array(items) => {
            for child in items {
                strip_non_constructive_keywords(child);
            }
        }
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
    }
}
