use crate::constructx::{get_base_user, Rectangle};
use crate::controller::get_base_first_word;
use crate::migrate::*;

mod migrate;
mod controller;
mod constructx;
mod enumx;
mod packagex;
mod arraylistx;
mod Typex;
mod testx;
mod iterator;

fn main() {
    /*get_base_tuple();
    get_base_array();
    let success = get_else_if(6);
    println!("success: {}", success);
    get_loop();*/

    // let s = String::from("hello");
    // get_base_first_word(&s);

    // get_base_user();
    let rect1 = Rectangle {
        width: 10,
        height: 20,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
