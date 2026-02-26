use tokio::time::{sleep, Duration};
    
async fn task_a() {
    sleep(Duration::from_secs(2)).await;
    println!("Task A done");
}

async fn task_b() {
    sleep(Duration::from_secs(1)).await;
    println!("Task B done");
}

pub async fn example() {
    // Wait for both tasks to complete
    let _ = tokio::join!(task_a(), task_b());   
}
