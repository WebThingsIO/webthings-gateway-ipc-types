/**
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.*
 */
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use os_pipe::{dup_stderr, dup_stdout};

mod extras_generator;

fn main() {
    clone_schema_repo();
    let schema_path = Path::new("gateway-addon-ipc-schema/schema.json");
    code_gen(
        jsonschema_code_generator::generate(&schema_path),
        "src/types.rs",
    );
    code_gen(extras_generator::generate(&schema_path), "src/extras.rs");
}

fn code_gen(code: String, file: &str) {
    let rust_code_types = format(code);
    fs::write(file, rust_code_types).expect("Unable to write file");
}

fn clone_schema_repo() {
    Command::new("rm")
        .arg("-rf")
        .arg("gateway-addon-ipc-schema")
        .stdout(dup_stdout().expect("Could not redirect stdout"))
        .stderr(dup_stderr().expect("Could not redirect stderr"))
        .output()
        .expect("Could not delete old schema repo");

    Command::new("git")
        .arg("clone")
        .arg("https://github.com/WebThingsIO/gateway-addon-ipc-schema.git")
        .stdout(dup_stdout().expect("Could not redirect stdout"))
        .stderr(dup_stderr().expect("Could not redirect stderr"))
        .output()
        .expect("Could not clone schema repo");

    Command::new("git")
        .arg("-C")
        .arg("gateway-addon-ipc-schema")
        .arg("checkout")
        .arg("v1.0.0")
        .stdout(dup_stdout().expect("Could not redirect stdout"))
        .stderr(dup_stderr().expect("Could not redirect stderr"))
        .output()
        .expect("Could not checkout correct schema version");
}

fn format(text: impl std::fmt::Display) -> String {
    let mut rustfmt = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    write!(rustfmt.stdin.take().unwrap(), "{}", text).unwrap();
    let output = rustfmt.wait_with_output().unwrap();
    String::from_utf8(output.stdout).unwrap()
}
