use crate::controller::get_base_first_word;
use crate::migrate::*;

mod migrate;
mod controller;

fn main() {
    /*get_base_tuple();
    get_base_array();
    let success = get_else_if(6);
    println!("success: {}", success);
    get_loop();*/

    let s = String::from("hello");
    get_base_first_word(&s);
}
