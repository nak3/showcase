use std::thread;
use std::time::Duration;
use futures::executor::ThreadPool;
use std::io::Read;

fn main() {
    let pool = ThreadPool::new().unwrap();
    let task = async {
        let mut id = 0;
        for _ in 1..4 {
            id = id + 10;
            pool.spawn_ok(async move {
                for i in 0..10 {
                    println!("thread {}: count {}", id, i);
                    id = id + 1;
                    thread::sleep(Duration::from_millis(1000));
                }
            });
            thread::sleep(Duration::from_millis(1000));
        }
    };
    println!("Enter some key to stop");
    futures::executor::block_on(task);
    let _ = std::io::stdin().read(&mut [0]);
}
