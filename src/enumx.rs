enum IpAddKind{
    V4,
    V6
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) ->u8{
    match coin {
        Coin::Dime => 10,
        Coin::Nickel => 5,
        Coin::Penny => 1,
        Coin::Quarter => 25,
    }
}

fn plus_one(x:Option<i32>)-> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}