#![feature(proc_macro_hygiene, decl_macro)]
#![allow(unused_attributes)]

#[macro_use]
extern crate rocket;

use blockchain::blockchain::{Block, Blockchain};
use rocket::State;

mod blockchain;

// #[get("/")]
// fn index() -> &'static str {
//     "Hello, world!"
// }

// #[get("/")]
// fn get_last_block(blockchain: &State<Blockchain>) -> String {
//     blockchain.get_last_block()
// }

// #[launch]
// fn rocket() -> _ {
//     rocket::build()
//         .manage(Blockchain::new())
//         .mount("/", routes![index])
//         .mount("/api", routes![get_last_block])
// }

fn main() {
    let block = Block::new(0, 0, 0);
    let hash = Block::get_hash(block);
    println!("Base64-encoded hash: {}", hash);
}
