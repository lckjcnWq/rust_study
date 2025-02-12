trait Duck{
    fn quack(&self);
}

struct MallardDuck;
impl Duck for MallardDuck{
      fn quack(&self){
        println!("v1")
      }
}

// 具体实现2
struct RubberDuck;
impl Duck for RubberDuck {
    fn quack(&self) {
        println!("橡皮鸭叫: 吱吱吱");
    }
}


// 多态应用场景
fn make_duck_sound(duck: &dyn Duck) { // dyn表示动态分发
    duck.quack(); // 统一调用接口
}