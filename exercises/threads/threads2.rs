// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.



use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::ops::AddAssign;

struct JobStatus {
    jobs_completed: u32,
}

impl AddAssign<u32> for JobStatus {
    fn add_assign(&mut self, other: u32) {
        self.jobs_completed += other;
    }
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            let mut num = status_shared.lock().unwrap();

            thread::sleep(Duration::from_millis(250));
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    let status = status.lock().unwrap();
    println!("jobs completed: {}", status.jobs_completed);
}
