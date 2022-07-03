#![feature(proc_macro_hygiene, decl_macro)]
#![allow(unused_attributes)]

#[macro_use]
extern crate rocket;

mod blockchain;
use blockchain::blockchain::Blockchain;
use rocket::State;

use serde_json;
use std::sync::Mutex;

struct StateBlockchain {
    blockchain: Mutex<Blockchain>,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/add_block")]
fn add_block(state: &State<StateBlockchain>) -> String {
    let lock = state.blockchain.lock();
    let mut blockchain = lock.unwrap();
    let new_block = blockchain.mine_block();
    return match serde_json::to_string(new_block) {
        Ok(new_block) => new_block,
        Err(e) => return e.to_string(),
    };
}

#[get("/last_block")]
fn get_last_block(state: &State<StateBlockchain>) -> String {
    let lock = state.blockchain.lock();
    let blockchain = lock.unwrap();
    let last_block = blockchain.get_last_block();
    return match serde_json::to_string(last_block) {
        Ok(last_block) => last_block,
        Err(e) => return e.to_string(),
    };
}

#[get("/valid_chain")]
fn check_valid(state: &State<StateBlockchain>) -> String {
    let lock = state.blockchain.lock();
    let blockchain = lock.unwrap();
    let is_valid = blockchain.is_chain_valid();

    if is_valid {
        return match serde_json::to_string(&blockchain.chain) {
            Ok(blockchain_json) => blockchain_json,
            Err(e) => return e.to_string(),
        };
    } else {
        return match serde_json::to_string("Chain is invalid.") {
            Ok(error) => error,
            Err(e) => return e.to_string(),
        };
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(StateBlockchain {
            blockchain: Mutex::new(Blockchain::new()),
        })
        .mount("/", routes![index])
        .mount("/api", routes![get_last_block, add_block, check_valid])
}
