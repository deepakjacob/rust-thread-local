use std::cell::RefCell;

use tokio::task;
const ORIG_VALUE: i32 = 42;

thread_local! {
    static T_LOCAL: RefCell<i32> = RefCell::new(ORIG_VALUE);
}

async fn task() {
    T_LOCAL.with(|val| {
        *val.borrow_mut() += 1;
        println!("value inside async_task: {}", *val.borrow());
    });
}

#[tokio::main]
async fn main() {
    let tasks = vec![
        task::spawn(task()),
        task::spawn(task()),
        task::spawn(task()),
    ];

    for task in tasks {
        let _ = task.await;
    }

    T_LOCAL.with(|val| {
        println!(
            "original value didn't change -> {}",
            *val.borrow() == ORIG_VALUE
        );
    });
}
