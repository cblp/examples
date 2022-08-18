use marine_rs_sdk::{marine, MountedBinaryResult};

#[marine]
pub struct Result {
    pub stdout: String,
    pub stderr: String,
    // err_code: i64,
}

#[marine]
#[link(wasm_import_module = "host")]
extern "C" {
    pub fn glaze(cmd: Vec<String>) -> MountedBinaryResult;
}
