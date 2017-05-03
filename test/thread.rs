#![feature(thread_id)]
#![feature(type_ascription)]
use std::thread;
use std::collections::VecDeque;
use std::sync::{Mutex, Arc, Condvar};
struct Logger(Arc<Mutex<Vec<String>>>);
impl Logger {
    fn new() -> Logger {
        Logger(Arc::new(Mutex::new(Vec::new())))
    }

    fn add(&mut self, msg: String) {
        self.0.lock().unwrap().push(msg);
    }

    fn print(&mut self) {
        let mut vec = self.0.lock().unwrap(); //.pop().unwrap();
        // vec.next();
        while !vec.is_empty() {
            println!("---msg = {:?}====", vec.pop().unwrap());//连续打印会出现线程中断，所以这个宏不适用！！！！！
        }
    }
}

#[derive(Debug)]
struct ThreadPool {
    PoolName: String,
    cur_count: u32,
    max_count: u32,
    min_count: u32,
    vec_tds: Vec<thread::JoinHandle<()>>,
    vec_Item: Arc<Mutex<VecDeque<i32>>>,
    con: Arc<Condvar>,
}

impl ThreadPool {
    fn new(ct: u32, name: String) -> ThreadPool {
        ThreadPool {
            PoolName: name,
            cur_count: ct,
            max_count: 10,
            min_count: 1,
            vec_tds: Vec::new(),
            vec_Item: Arc::new(Mutex::new(VecDeque::<i32>::new())),
            con: Arc::new(Condvar::new()),
        }
    }

    fn init(&mut self) {
        let mut count: u32 = 1;
        loop {
            //引用、所有者针对 “.”运算符都是一样的。不能和C++等语言相比较的。
            let mut vec = self.vec_Item.clone();
            let mut con = self.con.clone();

            self.vec_tds
                .push(thread::spawn(move || loop {
                                        {
                                            let mut vec_tmp = vec.lock().unwrap();
                                            if vec_tmp.len() == 0 {
                                                vec_tmp = con.wait(vec_tmp).unwrap();
                                            } else {
                                                // println!("--vec_vlues = {:?}", vec_tmp.pop_front());
                                            };
                                        }
                                        // println!("---spawn thread = {:?}--",thread::current().id());
                                        thread::sleep_ms(1);

                                    }));
            if count == self.cur_count {
                break;
            } else {
                count += 1;
            }
        }
    }

    fn wait(mut self) {
        //看到这个明白所有权的转移，转移过后，作用域、生命周期变了。
        for pat in self.vec_tds {
            pat.join();
        }
    }

    fn current_thread(&self) -> u32 {
        let count: u32 = self.vec_tds.len() as u32;
        count
    }

    fn set_threads_count(&mut self, count: u32) {
        self.cur_count = count;
    }

    fn post_item(&mut self, arg: i32) {
        let mut vec = self.vec_Item.lock().unwrap();
        vec.push_back(arg);
        if vec.len() > 0 {
            self.con.notify_all();
        }
    }
}


fn main() {
    // let mut log = Logger::new();
    // log.add(format!("---main thrend id = {:?}----", thread::current().id()).to_string());
    // log.add(format!("---main thrend id = {:?}----", thread::current().id()).to_string());
    // // log.print();


    // let mut pool = ThreadPool::new(2, "syl".to_string());
    // pool.init();
    // let mut kkk = 0;
    // for i in 1..1000 {
    //     // log.add();
    //     //怎么类型转换？？？？？ 或者说怎么怎实现可变类型。
    //     // log(thread::current().id());
    //     log.add(format!("---main thrend id = {:?}--{:?}-",thread::current().id(),i).to_string());
    //     // thread::sleep_ms(1);
    //     pool.post_item(i);
    //     kkk += 1;
    //     // println!("---main thrend id = {:?}--{:?}-",thread::current().id(),i)
    // }
    // println!("-----{}----",kkk);
    // log.print();

    // thread::sleep_ms(1000);
    // println!("---------");
    // pool.wait();

    let mut ii = 0;
    while true {
        ii += 1;
        if ii != 1000000 {
            //看来时输出内容越长，中断越快！！！！！
            println!("---------------------------------------------------------------------------------------------{}", ii);
        } else {
            break;
        }
    }



}
