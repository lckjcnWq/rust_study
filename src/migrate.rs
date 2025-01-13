//创建元组tuple
pub fn get_base_tuple()-> (i32, f64, u8){
    let x:(i32,f64,u8) = (500,6.4,1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("five_hundred :{}",five_hundred);
    println!("six_point_four:{}",six_point_four);
    println!("one :{}",one);
    x
}

pub fn get_base_array() -> [i32;5]{
    let a:[i32;5] = [1,2,3,4,5];
    println!("a:{:?}",a);
    a
}