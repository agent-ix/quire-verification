// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Controlled native child used only by NFR-006 acceptance tests.

#![forbid(unsafe_code)]

use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::{self, Command, Stdio};
use std::thread;
use std::time::Duration;

use serde_json::json;
use sha2::{Digest, Sha256};

fn main() {
    let mut arguments = env::args().skip(1);
    let mode = arguments.next().expect("fixture mode");
    match mode.as_str() {
        "parse" => parser_parse(arguments),
        "format" => parser_format(arguments),
        "inspect" => {
            let payload = json!({
                "arguments": arguments.collect::<Vec<_>>(),
                "workingDirectory": env::current_dir().expect("working directory"),
                "declared": env::var("DECLARED").ok(),
                "homePresent": env::var_os("HOME").is_some(),
            });
            print!("{payload}");
        }
        "write-marker" => write_marker(&arguments.next().expect("marker path")),
        "sleep-mark" => {
            let milliseconds = arguments
                .next()
                .expect("milliseconds")
                .parse::<u64>()
                .expect("integer milliseconds");
            let marker = arguments.next().expect("marker path");
            thread::sleep(Duration::from_millis(milliseconds));
            write_marker(&marker);
        }
        "descendant-holds-pipes" => {
            let milliseconds = arguments.next().expect("milliseconds");
            let pid_file = arguments.next().expect("descendant pid file");
            let marker = arguments.next().expect("marker path");
            spawn_unreaped_descendant(&milliseconds, &pid_file, &marker, true);
        }
        "descendant-closes-pipes" => {
            let milliseconds = arguments.next().expect("milliseconds");
            let pid_file = arguments.next().expect("descendant pid file");
            let marker = arguments.next().expect("marker path");
            spawn_unreaped_descendant(&milliseconds, &pid_file, &marker, false);
        }
        "stdout-bytes" => write_bytes(io::stdout(), byte_count(arguments), b'x'),
        "stderr-bytes" => write_bytes(io::stderr(), byte_count(arguments), b'y'),
        "both-bytes" => write_both(byte_count(arguments)),
        "exit" => {
            let code = arguments
                .next()
                .expect("exit code")
                .parse::<i32>()
                .expect("integer exit code");
            process::exit(code);
        }
        "abort" => process::abort(),
        "invalid-stdout" => io::stdout()
            .write_all(&[0xff])
            .expect("write invalid stdout"),
        "invalid-stderr" => io::stderr()
            .write_all(&[0xff])
            .expect("write invalid stderr"),
        "malformed-json" => print!("not-json"),
        "valid-json" => print!("{{\"status\":\"ok\"}}"),
        other => panic!("unknown fixture mode {other}"),
    }
}

fn parser_parse(mut arguments: impl Iterator<Item = String>) {
    let identity = arguments.next().expect("source identity");
    let revision = arguments.next().expect("source revision");
    let path = arguments.next().expect("source path");
    let bytes = fs::read(&path).expect("read parser fixture");
    if revision == "fixture:process-failure" {
        process::exit(7);
    }
    if bytes
        .windows(b"invarianx".len())
        .any(|part| part == b"invarianx")
    {
        eprintln!(
            "{}",
            json!({
                "status": "refused",
                "phase": "parse",
                "code": "invalid_syntax",
                "source": {"identity": identity, "revision": revision},
                "path": path,
                "span": {
                    "start": {"byte": 249, "line": 6, "column": 1},
                    "end": {"byte": 258, "line": 6, "column": 10}
                },
                "message": "expected a clause keyword"
            })
        );
        process::exit(1);
    }
    let digest = if revision == "fixture:protocol-mismatch" {
        format!("sha256:{}", "f".repeat(64))
    } else {
        format!("sha256:{:x}", Sha256::digest(&bytes))
    };
    println!(
        "{}",
        json!({
            "status": "parsed",
            "source": {
                "identity": identity,
                "revision": revision,
                "digest": digest
            },
            "path": path,
            "imports": 1,
            "clauses": 4
        })
    );
}

fn parser_format(mut arguments: impl Iterator<Item = String>) {
    let _identity = arguments.next().expect("source identity");
    let revision = arguments.next().expect("source revision");
    let path = arguments.next().expect("source path");
    let mut bytes = fs::read(path).expect("read formatter fixture");
    if revision == "fixture:formatted-mismatch" {
        bytes.extend_from_slice(b"\n// controlled second-format difference\n");
    }
    io::stdout()
        .write_all(&bytes)
        .expect("write formatted bytes");
}

// The missing wait is the adverse condition under test: the direct fixture
// exits while its descendant retains both inherited output pipes.
#[allow(clippy::zombie_processes)]
fn spawn_unreaped_descendant(
    milliseconds: &str,
    pid_file: &str,
    marker: &str,
    inherit_pipes: bool,
) {
    let mut command = Command::new(env::current_exe().expect("fixture executable"));
    command.args(["sleep-mark", milliseconds, marker]);
    if !inherit_pipes {
        command.stdout(Stdio::null()).stderr(Stdio::null());
    }
    let child = command.spawn().expect("spawn descendant");
    fs::write(pid_file, child.id().to_string()).expect("write descendant pid");
}

fn byte_count(mut arguments: impl Iterator<Item = String>) -> usize {
    arguments
        .next()
        .expect("byte count")
        .parse::<usize>()
        .expect("integer byte count")
}

fn write_marker(path: &str) {
    fs::write(Path::new(path), b"launched").expect("write marker");
}

fn write_bytes(mut output: impl Write, count: usize, byte: u8) {
    let block = [byte; 16 * 1024];
    let mut remaining = count;
    while remaining > 0 {
        let size = remaining.min(block.len());
        output.write_all(&block[..size]).expect("write output");
        output.flush().expect("flush output");
        remaining -= size;
    }
}

fn write_both(count: usize) {
    let block_out = [b'x'; 16 * 1024];
    let block_err = [b'y'; 16 * 1024];
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();
    let mut remaining = count;
    while remaining > 0 {
        let size = remaining.min(block_out.len());
        stdout.write_all(&block_out[..size]).expect("write stdout");
        stdout.flush().expect("flush stdout");
        stderr.write_all(&block_err[..size]).expect("write stderr");
        stderr.flush().expect("flush stderr");
        remaining -= size;
    }
}
