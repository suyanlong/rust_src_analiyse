
use std::default;
//可以看到核心的item，也是默认导入的，通过extern crate xxx;
//或者说，我们写的代码都是运行在核心模块里面。所以核心的item，不是pub item 也可以访问，可以理解编译器内置item。

use std::iter::Iterator;
use std::collections::VecDeque;

#[derive(Debug)]
struct Student<T> {
    name: T,
}

// impl<T> Iterator for Student<T> {
//     type Item = Student<T>;
//     fn next(&mut self) -> Option<Self::Student<T>>{
//         self
//     }
// }

fn main() {
    println!("----begin---");
    let i32_value: i32 = Default::default();
    println!("---{}---", i32_value);


         let mut contian = VecDeque::new();
    contian.push_back('a');
    contian.push_back('b');
    contian.push_back('c');
    contian.push_back('d');
    assert_eq!('d', contian.pop_back().unwrap());
    // contian.buffer_read(1);error: method `buffer_read` is private

    let flag = true;
    // flag.hash();

    //有一些核心部件、struct\trict都是编译器内置里面的,或者说item代表什么意思。
    use std::collections::hash_map::DefaultHasher; //这个才是结构体元素。
    use std::hash::Hasher; //这个是trait，
    let mut hasher = DefaultHasher::new();
    let data = [12];
    hasher.write(&data);
    println!("Hash is {:x}!", hasher.finish());

    //看来，不能在与C++、C语言比较了，rust语言的学习，还是要深入进去。
    //看到基本类型，怎么感觉也在标准库里面呢。
    println!("i32 max_values = {}", i32::max_value());
    let v1 = vec![1, 2, 3];
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, [2, 3, 4]);
    println!("----end---");
}


//   impl Hash for bool {
//         fn hash<H: Hasher>(&self, state: &mut H) {
//             state.write_u8(*self as u8)
//         }
//     }
