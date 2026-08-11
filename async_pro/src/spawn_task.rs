use std::time::Duration;

//A task is similar to a thread, but instead of being managed by the operating system, it’s managed by library-level code: the runtime.
//In that regard, tasks are similar to lightweight, runtime-managed threads with added capabilities that come from being managed by a runtime instead of by the operating system.

pub fn spawn_main() {
    trpl::block_on(async {
        let s_j = trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
        s_j.await.unwrap();
    });
}

pub fn double_fut() {
    trpl::block_on(async {
        
        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };
        let fut2 = async {
            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };
        trpl::join(fut1, fut2).await;
    })
}
