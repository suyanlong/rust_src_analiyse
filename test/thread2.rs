fn main() {
    use std::sync::{Arc, Mutex, Condvar};
    use std::thread;
    let mut vec_contain: Vec<i32> = vec![];
    let pair = Arc::new((Mutex::new(vec_contain), Condvar::new()));
    let pair2 = pair.clone();
    thread::spawn(move || {
        let &(ref lock, ref cvar) = &*pair2;
        let mut count = 0;
        while true {
            {
                thread::sleep_ms(10);

                let mut vec_tmp = lock.lock().unwrap();                
                vec_tmp.push(count);
                count += 1;
                println!("-spawn--{:?}", *vec_tmp);

                if count ==100{
                    break;
                }
            }
            // We notify the condvar that the value has changed.
            cvar.notify_all();
        }

    });

    // Wait for the thread to start up.
    let &(ref lock, ref cvar) = &*pair;
    
    // As long as the value inside the `Mutex` is false, we wait.
    while true {
        thread::sleep_ms(100);
        let mut vec_tmp = lock.lock().unwrap(); //循环时，会遇到锁不释放的问题。
        if vec_tmp.len() > 0 {
            println!("--123123-{:?}", vec_tmp.pop());
        } else {

            vec_tmp = cvar.wait(vec_tmp).unwrap();
             println!("============");

        }
    }
}


// use std::sync::{Arc, Mutex, Condvar};
// use std::thread;

// fn main() {

//     let pair = Arc::new((Mutex::new(false), Condvar::new()));
//     let pair2 = pair.clone();

//     // 创建一个新线程
//     thread::spawn(move|| {
//         let &(ref lock, ref cvar) = &*pair2;
//         let mut started = lock.lock().unwrap();
//         *started = true;
//         cvar.notify_one();
//         println!("notify main thread");
//     });

//     // 等待新线程先运行
//     let &(ref lock, ref cvar) = &*pair;
//     let mut started = lock.lock().unwrap();
//     while !*started {
//         println!("before wait");
//         started = cvar.wait(started).unwrap();
//         println!("after wait");
//     }
// }