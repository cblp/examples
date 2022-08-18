use marine_rs_sdk::marine;

pub use crate::common::{glaze, Result};

#[marine]
fn did_create() -> Result {
    // create a new DID
    let cmd = "did:create";
    let args = vec![cmd.to_owned()];

    let res = glaze(args);

    let stdout = String::from_utf8(res.stderr).unwrap();
    let stderr = String::from_utf8(res.stdout).unwrap();

    let s = "- Creating DID...\n✔ Created DID did:key:z6Mkm1tS3ezscF1pm5MuJJtVLvvL1MjtTxJAJBa5reKGHMWA with seed 61896cd65e4c254405ef1f8f898cb0279e84ecaa714c12843b17499bfe20a084\n";
    let s: Vec<&str> = s.split("DID ").collect();
    let s = s[1].replace("\n", "");
    let s: Vec<&str> = s.split(" with seed ").collect();

    Result {
        stdout: stdout,
        stderr: stderr,
    }
}

#[marine]
pub fn did_get() -> Result {
    // get the contents of a record in a DID DataStore
    Result {
        stdout: "".to_owned(),
        stderr: "Not implemented".to_owned(),
    }
}

#[marine]
pub fn did_inspect(sk: String, did: String) -> Result {
    // inspect the contents of a DID DataStore
    let cmd = "did:inspect";

    let mut args = vec![cmd.to_owned()];

    if sk.len() > 0 {
        args.push("-k");
    }

    Result {
        stdout: "".to_owned(),
        stderr: "Not implemented".to_owned(),
    }
}

#[marine]
pub fn did_merge() -> Result {
    // merge the contents of a record in a DID DataStore
    Result {
        stdout: "".to_owned(),
        stderr: "Not implemented".to_owned(),
    }
}

#[marine]
pub fn did_set() -> Result {
    // set the contents of a record in a DID DataStore
    Result {
        stdout: "".to_owned(),
        stderr: "Not implemented".to_owned(),
    }
}

#[marine]
pub fn did_sign() -> Result {
    // create a JSON Web Signature
    Result {
        stdout: "".to_owned(),
        stderr: "Not implemented".to_owned(),
    }
}

#[marine]
pub fn did_verify() -> Result {
    // verify a JSON Web Signature
    Result {
        stdout: "".to_owned(),
        stderr: "Not implemented".to_owned(),
    }
}
