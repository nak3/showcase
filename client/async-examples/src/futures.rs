use std::thread;
use std::time::Duration;

async fn foo(id: i32) {
    for i in 0..5 {
        println!("thread #{} count {}.", id, i);
        thread::sleep(Duration::from_millis(1000));
    }
}

fn main() {
    let task = async {
        foo(10).await;
        foo(20).await;
    };
    futures::executor::block_on(task);
    println!("done");
}
