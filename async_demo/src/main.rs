use tokio::time::{sleep, Duration};

#[derive(Debug)]
struct TaskResult {
    name: String,
    duration: u64,
    result: String,
}

async fn execute_task(name: &str, duration: u64) -> TaskResult {
    println!("{} created at {:?}", name, chrono::Local::now());
    
    sleep(Duration::from_millis(duration)).await;
    
    println!("{} completed at {:?}", name, chrono::Local::now());
    
    TaskResult {
        name: name.to_string(),
        duration,
        result: format!("{} result", name),
    }
}

#[tokio::main]
async fn main() {
    println!("Rust Demo Start\n");
    
    // Create multiple async tasks
    let task1 = execute_task("Rust Task 1", 2000);
    let task2 = execute_task("Rust Task 2", 1000);
    
    println!("Tasks created, but not yet started...\n");
    
    sleep(Duration::from_millis(500)).await;
    println!("Starting task execution now...\n");
    
    // Wait for results
    let results = tokio::join!(task1, task2);
    println!("\nAll results: {:?}", results);
}
