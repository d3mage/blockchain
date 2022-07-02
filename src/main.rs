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
    let mut blockchain = Blockchain::new();
    blockchain.mine_block();
    blockchain.mine_block();
    blockchain.mine_block();
    let valid = blockchain.is_chain_valid();
    println!("Chain validity: {}", valid);
}
