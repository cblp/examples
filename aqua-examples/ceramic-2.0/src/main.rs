/*
 * Copyright 2021 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use marine_rs_sdk::{module_manifest, WasmLoggerBuilder};

mod common;
mod glaze_config;
mod glaze_did;
mod glaze_model;
mod glaze_pin;
mod glaze_stream;
mod glaze_tile;

module_manifest!();

pub fn main() {
    WasmLoggerBuilder::new().build().unwrap();
}

/*
For some reason, glaze writes to stderr instead of stdout. so we'll have to switch things up.
*/

/*

#[marine]
fn create_stream(sk: String, payload: String) -> Result {
    let args = vec![
        "tile:create".to_owned(),
        "--key".to_owned(),
        sk,
        "--content".to_owned(),
        payload,
    ];

    let res = glaze(args);
    let stdout = String::from_utf8(res.stderr).unwrap();
    let stderr = String::from_utf8(res.stdout).unwrap();

    Result {
        stdout: stdout,
        stderr: stderr,
    }
}

#[marine]
fn query_stream(stream_id: String) -> Result {
    let args = vec!["tile:show".to_owned()];

    let res = glaze(args);
    let stdout = String::from_utf8(res.stderr).unwrap();
    let stderr = String::from_utf8(res.stdout).unwrap();

    Result {
        stdout: stdout,
        stderr: stderr,
    }
}

#[marine]
fn update_stream(sk: String, stream_id: String, payload: String) -> Result {
    let args = vec![
        "tile:update".to_owned(),
        "--key".to_owned(),
        sk,
        "--content".to_owned(),
        payload,
    ];

    let res = glaze(args);
    let stdout = String::from_utf8(res.stderr).unwrap();
    let stderr = String::from_utf8(res.stdout).unwrap();

    Result {
        stdout: stdout,
        stderr: stderr,
    }
}

#[marine]
fn stream_commits() {
    // list commits contained within a stream
}


#[marine]
#[link(wasm_import_module = "host")]
extern "C" {
    pub fn glaze(cmd: Vec<String>) -> MountedBinaryResult;
}
*/
