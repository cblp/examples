use marine_rs_sdk::marine;

pub use crate::common::{glaze, Result};

#[marine]
fn stream_commits(stream_id: String, sk: String) -> Result {
    // list commits contained within a stream
    Result {
        stdout: "".to_owned(),
        stderr: "Not implemented".to_owned(),
    }
}

#[marine]
fn stream_state() -> Result {
    // get the state of a Stream

    Result {
        stdout: "".to_owned(),
        stderr: "Not implemented".to_owned(),
    }
}
