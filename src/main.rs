mod migrate;

fn main() {
    migrate::get_base_tuple();
    migrate::get_base_array();
    let success:bool = migrate::get_else_if(6);
    println!("success: {}", success);
    migrate::get_loop();
}
