
use std::collections::HashMap;
use std::iter::Map;
use std::vec;
use std::iter;

#[derive(Debug)]
struct student {
    name: &'static str,
}

impl student {
    fn printinfo(&self) {
        println!("----{}---", self.name);
    }
}

fn bar(stu: &student) {
    println!("-bar---{}---", stu.name);
}


fn main() {
    println!("--------begin------");
    // let map_contian:M
    let mut arry = [1, 2, 3, 4, 5, 6];
    let mut iter = (&arry).iter();
    println!("----{:?}---", iter.next());
    let mut iter = arry.iter();
    iter.next();
    iter.next();
    iter.next();
    println!("----{:?}---", iter.next());
    for pat in &arry {
        println!("--bar--{}---", pat);
    }

    let vec: Vec<i32> = vec![1, 2, 3, 4, 5];
    // let vec_ref = vec.as_ref();//as_ref 是一个私有的成员，不允许访问。
    vec.is_empty();


    let mut stu = student { name: "suyanlong" };
    stu.printinfo();//成员方法可以这样调用，进行引用强制多态，
    bar(&stu);//而函数却不可以。
    (&mut stu).printinfo();


    let into_iter = vec.into_iter();
    let iter = into_iter.into_iter();
    // iter.
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];
    let answer = foo(&v1, &v2);



    println!("-----------end---");
}


fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    // do stuff with v1 and v2

    // return the answer
    42
}
