// TypeScript Example (save as async_demo.ts)
// Run with: ts-node async_demo.ts

// Define return types for clarity
type TaskResult = {
    name: string;
    duration: number;
    result: string;
};

class AsyncTask {
    constructor(
        private name: string,
        private duration: number
    ) {}

    async execute(): Promise<TaskResult> {
        console.log(`${this.name} created at ${new Date().toISOString()}`);
        
        await new Promise<void>((resolve) => 
            setTimeout(resolve, this.duration)
        );
        
        console.log(`${this.name} completed at ${new Date().toISOString()}`);
        
        return {
            name: this.name,
            duration: this.duration,
            result: `${this.name} result`
        };
    }
}

async function runTsDemo() {
    console.log("TypeScript Demo Start\n");
    
    // Create multiple async tasks
    const task1 = new AsyncTask("TS Task 1", 2000);
    const task2 = new AsyncTask("TS Task 2", 1000);
    
    console.log("Tasks instantiated, but not yet executed...\n");
    
    // Small delay to demonstrate that tasks don't start until execute() is called
    await new Promise<void>(resolve => setTimeout(resolve, 500));
    console.log("Starting task execution now...\n");
    
    // Start tasks in parallel
    const tasks = [
        task1.execute(),
        task2.execute()
    ];
    
    // Wait for results
    const results = await Promise.all(tasks);
    console.log("\nAll results:", results);
}

// Execute the demo
runTsDemo().catch(console.error);
