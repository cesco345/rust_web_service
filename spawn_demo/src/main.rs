use tokio::time::{sleep, Duration};
use std::sync::Arc;
use tokio::sync::Mutex;  // Changed to tokio::sync::Mutex instead of std::sync::Mutex

async fn basic_spawn_example() {
    println!("\n=== Basic Spawn Example ===");
    
    let handle = tokio::spawn(async {
        for i in 0..3 {
            println!("Task 1: Number {}", i);
            sleep(Duration::from_millis(100)).await;
        }
        "Task 1 Complete"
    });
    
    for i in 0..3 {
        println!("Main task: Number {}", i);
        sleep(Duration::from_millis(100)).await;
    }
    
    let result = handle.await.unwrap();
    println!("Spawned task result: {}", result);
}

async fn multiple_tasks_example() {
    println!("\n=== Multiple Tasks Example ===");
    
    let mut handles = vec![];
    
    for i in 0..3 {
        let handle = tokio::spawn(async move {
            println!("Task {} starting", i);
            sleep(Duration::from_millis(100 * (i + 1) as u64)).await;
            println!("Task {} completed", i);
            i
        });
        handles.push(handle);
    }
    
    for handle in handles {
        let result = handle.await.unwrap();
        println!("Task returned: {}", result);
    }
}

async fn shared_state_example() {
    println!("\n=== Shared State Example ===");
    
    // Create shared counter using tokio::sync::Mutex instead of std::sync::Mutex
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for i in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = tokio::spawn(async move {
            // Lock the mutex
            let mut lock = counter.lock().await;  // Note: .await here instead of .unwrap()
            *lock += 1;
            println!("Task {} incremented counter to {}", i, *lock);
            // Lock is automatically dropped at the end of scope
            drop(lock);  // Optional but explicit
            
            // Now we can safely await since we've dropped the lock
            sleep(Duration::from_millis(100)).await;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.await.unwrap();
    }
    
    let final_count = counter.lock().await;
    println!("Final counter value: {}", *final_count);
}

async fn channel_example() {
    println!("\n=== Channel Communication Example ===");
    
    let (tx, mut rx) = tokio::sync::mpsc::channel(32);
    
    let producer = tokio::spawn(async move {
        for i in 0..5 {
            tx.send(i).await.unwrap();
            println!("Produced: {}", i);
            sleep(Duration::from_millis(100)).await;
        }
    });
    
    let consumer = tokio::spawn(async move {
        while let Some(value) = rx.recv().await {
            println!("Consumed: {}", value);
            sleep(Duration::from_millis(200)).await;
        }
    });
    
    producer.await.unwrap();
    consumer.await.unwrap();
}

#[tokio::main]
async fn main() {
    basic_spawn_example().await;
    multiple_tasks_example().await;
    shared_state_example().await;
    channel_example().await;
}
