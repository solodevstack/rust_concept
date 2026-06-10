use std::time::Duration;
pub fn spawn_main() {
    trpl::block_on(async {
     let s_j   =   trpl::spawn_task(async {
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
