//所有权
pub fn get_base_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            println!("index i:{}",&s[0..i]);
            return &s[0..i];
        }
    }
    println!("get_base_first_word :{}",&s[..]);
    &s[..]
}