use marine_rs_sdk::marine;

pub use crate::common::{glaze, Result};

#[marine]
fn model_state() -> Result {
    // get the state of a Stream
    Result {
        stdout: "".to_owned(),
        stderr: "Not implemented".to_owned(),
    }
}
// #[marine]
fn model_create() -> Result {
    // create a local model
    Result {
        stdout: "".to_owned(),
        stderr: "Not implemented".to_owned(),
    }
}
#[marine]
fn model_delete() -> Result {
    // delete a local model
    Result {
        stdout: "".to_owned(),
        stderr: "Not implemented".to_owned(),
    }
}
#[marine]
fn model_add() -> Result {
    // add a stream to a model
    Result {
        stdout: "".to_owned(),
        stderr: "Not implemented".to_owned(),
    }
}
#[marine]
fn model_deploy() -> Result {
    // deploy a model
    Result {
        stdout: "".to_owned(),
        stderr: "Not implemented".to_owned(),
    }
}
#[marine]
fn model_inspect() -> Result {
    // inspect a model
    Result {
        stdout: "".to_owned(),
        stderr: "Not implemented".to_owned(),
    }
}
#[marine]
fn model_list() -> Result {
    // list local models
    Result {
        stdout: "".to_owned(),
        stderr: "Not implemented".to_owned(),
    }
}
#[marine]
fn model_export() -> Result {
    // export a model
    Result {
        stdout: "".to_owned(),
        stderr: "Not implemented".to_owned(),
    }
}
#[marine]
fn model_import() -> Result {
    // import a model into another one
    Result {
        stdout: "".to_owned(),
        stderr: "Not implemented".to_owned(),
    }
}
