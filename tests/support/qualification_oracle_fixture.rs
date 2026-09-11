// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Rust producer used only by the real Engineering Assurance consumer test.

#![forbid(unsafe_code)]

use std::env;
use std::ffi::OsString;
use std::fs;
use std::process::{self, Command};
use std::thread;
use std::time::Duration;

fn fail(message: &str) -> ! {
    eprintln!("{message}");
    process::exit(125);
}

fn argument(arguments: &mut impl Iterator<Item = OsString>) -> OsString {
    let Some(argument) = arguments.next() else {
        fail("missing fixture argument");
    };
    argument
}

fn text(value: OsString) -> String {
    match value.into_string() {
        Ok(value) => value,
        Err(_) => fail("fixture argument must be UTF-8"),
    }
}

fn write_response(arguments: &mut impl Iterator<Item = OsString>, malformed: bool) {
    let output = argument(arguments);
    let payload = if malformed {
        OsString::from("{")
    } else {
        argument(arguments)
    };
    if fs::write(output, payload.as_encoded_bytes()).is_err() {
        fail("cannot write fixture response");
    }
}

fn wait(arguments: &mut impl Iterator<Item = OsString>) {
    let Ok(duration) = text(argument(arguments)).parse::<u64>() else {
        fail("invalid wait duration");
    };
    thread::sleep(Duration::from_millis(duration));
}

fn exit_with(arguments: &mut impl Iterator<Item = OsString>) -> ! {
    let Ok(code) = text(argument(arguments)).parse::<i32>() else {
        fail("invalid exit code");
    };
    process::exit(code);
}

fn write_response_then_exit(arguments: &mut impl Iterator<Item = OsString>) -> ! {
    write_response(arguments, false);
    exit_with(arguments)
}

fn fixture_executable() -> OsString {
    match env::var_os("QV_FIXTURE_EXECUTABLE") {
        Some(path) => path,
        None => fail("missing fixture executable binding"),
    }
}

fn escape(arguments: &mut impl Iterator<Item = OsString>) {
    let pid_path = argument(arguments);
    let child = Command::new(fixture_executable())
        .arg("escape-child")
        .arg(pid_path)
        .spawn();
    let Ok(mut child) = child else {
        fail("cannot spawn escaping fixture");
    };
    if child.wait().is_err() {
        fail("cannot wait for escaping fixture");
    }
}

fn escape_child(arguments: &mut impl Iterator<Item = OsString>) {
    if rustix::process::setsid().is_err() {
        fail("cannot create fixture session");
    }
    if fs::write(argument(arguments), process::id().to_string()).is_err() {
        fail("cannot write escaped pid");
    }
    thread::sleep(Duration::from_secs(10));
}

fn main() {
    let mut arguments = env::args_os().skip(1);
    match text(argument(&mut arguments)).as_str() {
        "emit" => write_response(&mut arguments, false),
        "emit-exit" => write_response_then_exit(&mut arguments),
        "malformed" => write_response(&mut arguments, true),
        "wait" => wait(&mut arguments),
        "exit" => exit_with(&mut arguments),
        "escape" => escape(&mut arguments),
        "escape-child" => escape_child(&mut arguments),
        mode => fail(&format!("unknown fixture mode: {mode}")),
    }
}
