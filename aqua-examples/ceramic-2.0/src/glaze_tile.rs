use marine_rs_sdk::marine;

pub use crate::common::{glaze, Result};

#[marine]
pub fn tile_content(sk: String, stream_id: String) -> Result {
    // show the contents of a Tile stream
    let args: Vec<String> = vec!["tile:content".to_owned(), "--key".to_owned(), sk];

    let res = glaze(args);
    let stdout = String::from_utf8(res.stderr).unwrap();
    let stderr = String::from_utf8(res.stdout).unwrap();

    Result {
        stdout: stdout,
        stderr: stderr,
    }
}

// #[marine]
pub fn tile_create(sk: String, payload: String) -> Result {
    // create a new Tile stream
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
pub fn tile_update(sk: String, stream_id: String, payload: String) -> Result {
    // Update a stream
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
pub fn tile_deterministic() -> Result {
    // load a deterministic Tile stream
    Result {
        stdout: "".to_owned(),
        stderr: "Not implemented".to_owned(),
    }
}

#[marine]
pub fn tile_show(stream_id: String) -> Result {
    // show the contents of a Tile stream
    let args = vec!["tile:show".to_owned()];

    let res = glaze(args);
    let stdout = String::from_utf8(res.stderr).unwrap();
    let stderr = String::from_utf8(res.stdout).unwrap();

    Result {
        stdout: stdout,
        stderr: stderr,
    }
}
