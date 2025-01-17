use std::collections::HashMap;

fn get_base_vector(){
    let mut vec = Vec::new();
    vec.push(1);
    vec.insert(0,2);

    *vec.get_mut(0).unwrap() = 6;
}

fn get_base_map(){
    let mut map = HashMap::new();
    map.insert("one",1);
    map.insert("two",2);

    //更新元素
    map.entry("one").or_insert(100);

    map.remove("one");
}
