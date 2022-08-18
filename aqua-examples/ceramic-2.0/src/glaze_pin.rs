use marine_rs_sdk::marine;

pub use crate::common::{glaze, Result};

#[marine]
fn pin_add() -> Result {
    // pin a stream
    Result {
        stdout: "".to_owned(),
        stderr: "Not implemented".to_owned(),
    }
}

#[marine]
fn pin_ls() -> Result {
    // list pinned streams
    Result {
        stdout: "".to_owned(),
        stderr: "Not implemented".to_owned(),
    }
}

#[marine]
fn pin_rm() -> Result {
    //  unpin a
    Result {
        stdout: "".to_owned(),
        stderr: "Not implemented".to_owned(),
    }
}
