/*
 * Copyright 2022 Fluence Labs Limited
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

use marine_rs_sdk::{marine, module_manifest, WasmLoggerBuilder};

use forest_cid::Cid;
use forest_cid::Code;
// use cid::multihash::{Code, MultihashDigest};
//use cid::Cid;
use forest_db;
use forest_ipld::ipld;
use ipld_blockstore::BlockStore;
use ipld_proofs::ProofGenerator;

module_manifest!();

pub fn main() {
    WasmLoggerBuilder::new().build().unwrap();
}

#[marine]
// pub fn data_proof(data: Vec<u8>) {
pub fn data_proof() {
    let bs = forest_db::MemoryDB::default();

    let e = bs.put(&8u8, Code::Blake2b256).unwrap();
    let d = bs.put(&"Some data", Code::Blake2b256).unwrap();
    let c = bs.put(&"Some other value", Code::Blake2b256).unwrap();
    let b = bs.put(&(d, e), Code::Blake2b256).unwrap();
    let a = bs.put(&ipld!([2u8, "3", 4u64]), Code::Blake2b256).unwrap();
    let root = bs.put(&ipld!([a, b, c]), Code::Blake2b256).unwrap();

    // Start using the proof generator here
    let p_gen = ProofGenerator::new(&bs);

    // Retrieve data from a store
    let [_, b, _]: [Cid; 3] = p_gen.get(&root).unwrap().unwrap();
    let (d, _): (Cid, Cid) = p_gen.get(&b).unwrap().unwrap();
    let data: String = p_gen.get(&d).unwrap().unwrap();

    // Generate a proof of the data
    let proof = p_gen.generate_proof(&data).unwrap();

    println!("proof: {:?}", proof);
    //println!("proof validate: ()", proof.validate().unwrap());
}

// To run tests:
// cargo test --release
// Since the unit tests are using the wasm module via the marine_test crate import
// the modules and Config.toml need to exist.
// That is, run ./build.sh before you run cargo test.
// Moreover, the test function(s) need to be prefixed by the wasm module namespace, which
// generally is derived from the project name.
// For example, if you name the project "greeting", e.g., cargo generate -g https:// ... --name greeting,
// the test below can be executed as is. If not, the project needs to replace the "greeting"
// reference in place. Let's say our project name is "greetproject" then you need to update the type to:
// fn test_greeting(greeting: marine_test_env::greetproject::ModuleInterface)

#[cfg(test)]
mod tests {
    use super::*;
    use marine_rs_sdk_test::marine_test;

    // #[marine_test(config_path = "../Config.toml", modules_dir = "../artifacts")]
    #[test]
    fn dta_proof() {
        let proof = data_proof();
        // assert_eq!(proof.nodes().len(), 3);
        // assert_eq!(proof.root(), root);

        //println!("proof validate: ()", proof.unwrap());
    }
}
