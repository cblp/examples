use marine_rs_sdk::marine;

pub use crate::common::{glaze, Result};

#[marine]
fn config_get() -> Result {
    // get a config value
    let cmd = "config:show";
    let args = vec![cmd.to_owned()];

    let res = glaze(args);

    let stdout = String::from_utf8(res.stderr).unwrap();
    let stderr = String::from_utf8(res.stdout).unwrap();

    Result {
        stdout: stdout,
        stderr: stderr,
    }
}

#[marine]
fn config_show() -> Result {
    // show the full config
    Result {
        stdout: "".to_owned(),
        stderr: "Not implemented".to_owned(),
    }
}

#[marine]
fn config_set() -> Result {
    // set a config value
    Result {
        stdout: "".to_owned(),
        stderr: "Not implemented".to_owned(),
    }
}

#[marine]
fn config_reset() -> Result {
    // reset a config value
    Result {
        stdout: "".to_owned(),
        stderr: "Not implemented".to_owned(),
    }
}
