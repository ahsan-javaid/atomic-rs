use core::num;
use std::thread;

fn main() {
    // simple threads
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);
    t1.join().unwrap();
    t2.join().unwrap();

    // threads closure
    let numbers = Vec::from_iter(0..=1000);
    let t = thread::spawn(move || {
        let n = numbers.len();

        let sum = numbers.iter().sum::<usize>();

        sum / n
    });
    let avg = t.join().unwrap();
    println!("average: {avg}");

    // scoped threads
    


}


fn f() {
    println!("Hello from another thread!");

    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}