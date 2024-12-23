use std::sync::{Arc, Mutex};
use tokio::sync::{mpsc, oneshot};
use tokio::time::{sleep, Duration, Instant};
use std::collections::HashMap;

// Helper function to print timing info
async fn log_operation(start: Instant, operation: &str, details: &str) {
    let elapsed = start.elapsed().as_millis();
    println!("[{:>4}ms] {} - {}", elapsed, operation, details);
}

// Example 1: Basic Mutex - Quick operations
struct BasicBank {
    accounts: Mutex<HashMap<String, i32>>,
}

impl BasicBank {
    fn new() -> Self {
        let mut accounts = HashMap::new();
        accounts.insert("Alice".to_string(), 100);
        BasicBank {
            accounts: Mutex::new(accounts)
        }
    }

    fn deposit(&self, account: &str, amount: i32) -> Result<i32, &'static str> {
        // Basic mutex locks immediately block other threads
        let mut accounts = self.accounts.lock().unwrap();
        if let Some(balance) = accounts.get_mut(account) {
            *balance += amount;
            Ok(*balance)
        } else {
            Err("Account not found")
        }
    }
}

// Example 2: Async Mutex - Complex operations
struct AsyncBank {
    accounts: tokio::sync::Mutex<HashMap<String, i32>>,
}

impl AsyncBank {
    fn new() -> Self {
        let mut accounts = HashMap::new();
        accounts.insert("Alice".to_string(), 100);
        AsyncBank {
            accounts: tokio::sync::Mutex::new(accounts)
        }
    }

    async fn process_deposit(&self, account: &str, amount: i32) -> Result<i32, &'static str> {
        let mut accounts = self.accounts.lock().await;
        // Simulate some async processing while holding the lock
        sleep(Duration::from_millis(200)).await;
        
        if let Some(balance) = accounts.get_mut(account) {
            *balance += amount;
            Ok(*balance)
        } else {
            Err("Account not found")
        }
    }
}

// Example 3: Message Passing - Independent manager
#[derive(Debug)]
enum BankMessage {
    Deposit { 
        account: String, 
        amount: i32, 
        respond_to: oneshot::Sender<Result<i32, String>> 
    }
}

async fn run_basic_mutex_example() {
    println!("\n=== Basic Mutex Example (Blocking Operations) ===");
    let bank = Arc::new(BasicBank::new());
    let start = Instant::now();
    let mut handles = vec![];

    // Launch three concurrent operations
    for i in 0..3 {
        let bank = Arc::clone(&bank);
        let start = start.clone();
        handles.push(tokio::spawn(async move {
            log_operation(start, "Task", &format!("{} starting", i)).await;
            
            match bank.deposit("Alice", 50) {
                Ok(balance) => {
                    log_operation(start, "Task", 
                        &format!("{} completed - Balance: {}", i, balance)).await;
                },
                Err(e) => log_operation(start, "Task", 
                    &format!("{} failed - {}", i, e)).await,
            }
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }
}

async fn run_async_mutex_example() {
    println!("\n=== Async Mutex Example (Non-blocking Operations) ===");
    let bank = Arc::new(AsyncBank::new());
    let start = Instant::now();
    let mut handles = vec![];

    // Launch three concurrent operations
    for i in 0..3 {
        let bank = Arc::clone(&bank);
        let start = start.clone();
        handles.push(tokio::spawn(async move {
            log_operation(start, "Task", &format!("{} starting", i)).await;
            
            match bank.process_deposit("Alice", 50).await {
                Ok(balance) => {
                    log_operation(start, "Task", 
                        &format!("{} completed - Balance: {}", i, balance)).await;
                },
                Err(e) => log_operation(start, "Task", 
                    &format!("{} failed - {}", i, e)).await,
            }
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }
}

async fn run_message_passing_example() {
    println!("\n=== Message Passing Example (Independent Manager) ===");
    let (tx, mut rx) = mpsc::channel(32);
    let start = Instant::now();

    // Spawn the bank manager task
    let manager = tokio::spawn(async move {
        let mut accounts = HashMap::new();
        accounts.insert("Alice".to_string(), 100);
        
        while let Some(msg) = rx.recv().await {
            match msg {
                BankMessage::Deposit { account, amount, respond_to } => {
                    // Manager processes each request sequentially
                    sleep(Duration::from_millis(200)).await;
                    
                    let result = match accounts.get_mut(&account) {
                        Some(balance) => {
                            *balance += amount;
                            Ok(*balance)
                        },
                        None => Err("Account not found".to_string())
                    };
                    let _ = respond_to.send(result);
                }
            }
        }
    });

    // Launch three concurrent client requests
    let mut client_handles = vec![];
    for i in 0..3 {
        let tx = tx.clone();
        let start = start.clone();
        client_handles.push(tokio::spawn(async move {
            log_operation(start, "Client", &format!("{} sending request", i)).await;
            
            let (resp_tx, resp_rx) = oneshot::channel();
            tx.send(BankMessage::Deposit {
                account: "Alice".to_string(),
                amount: 50,
                respond_to: resp_tx,
            }).await.unwrap();

            match resp_rx.await.unwrap() {
                Ok(balance) => {
                    log_operation(start, "Client", 
                        &format!("{} got response - Balance: {}", i, balance)).await;
                },
                Err(e) => log_operation(start, "Client", 
                    &format!("{} got error - {}", i, e)).await,
            }
        }));
    }

    // Wait for all clients and cleanup
    for handle in client_handles {
        handle.await.unwrap();
    }
    drop(tx);
    manager.await.unwrap();
}

#[tokio::main]
async fn main() {
    run_basic_mutex_example().await;
    sleep(Duration::from_secs(1)).await;
    run_async_mutex_example().await;
    sleep(Duration::from_secs(1)).await;
    run_message_passing_example().await;
}
