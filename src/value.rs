// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Fallible JSON shape access shared by the public boundaries (NFR-005).

use serde_json::{Map, Value};

use crate::contracts::{VerificationError, VerificationErrorCode};

pub(crate) fn object<'a>(
    value: &'a Value,
    label: &str,
) -> Result<&'a Map<String, Value>, VerificationError> {
    value
        .as_object()
        .ok_or_else(|| invalid(label, "must be an object"))
}

pub(crate) fn array<'a>(value: &'a Value, label: &str) -> Result<&'a [Value], VerificationError> {
    value
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| invalid(label, "must be an array"))
}

pub(crate) fn field<'a>(
    value: &'a Value,
    name: &str,
    label: &str,
) -> Result<&'a Value, VerificationError> {
    object(value, label)?
        .get(name)
        .ok_or_else(|| invalid(&format!("{label}.{name}"), "is required"))
}

pub(crate) fn string<'a>(value: &'a Value, label: &str) -> Result<&'a str, VerificationError> {
    value
        .as_str()
        .filter(|item| !item.is_empty())
        .ok_or_else(|| invalid(label, "must be a non-empty string"))
}

pub(crate) fn string_field<'a>(
    value: &'a Value,
    name: &str,
    label: &str,
) -> Result<&'a str, VerificationError> {
    string(field(value, name, label)?, &format!("{label}.{name}"))
}

pub(crate) fn arrays_field<'a>(
    value: &'a Value,
    name: &str,
    label: &str,
) -> Result<&'a [Value], VerificationError> {
    array(field(value, name, label)?, &format!("{label}.{name}"))
}

pub(crate) fn invalid(label: &str, detail: impl Into<String>) -> VerificationError {
    VerificationError::new(VerificationErrorCode::InvalidInput, detail.into())
        .with_context("field", label)
}

pub(crate) fn optional_array<'a>(
    value: &'a Value,
    name: &str,
    label: &str,
) -> Result<&'a [Value], VerificationError> {
    match object(value, label)?.get(name) {
        Some(item) => array(item, &format!("{label}.{name}")),
        None => Ok(&[]),
    }
}
