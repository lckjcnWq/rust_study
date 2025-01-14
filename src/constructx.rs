//结构体
#[derive(Debug)]
pub struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}


pub fn get_base_user()-> User{
    let user1 = User {
        email: String::from("tx1"),
        username: String::from("tx2"),
        active: true,
        sign_in_count: 1,
    };
    println!("user1:{:?}", &user1);

    let user2 = User {
        email: String::from("tx3"),
        ..user1
    };
    println!("user2:{:?}", &user2);

    return user2;
 }

impl Rectangle  {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}