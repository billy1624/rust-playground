use lazy_static::lazy_static;
use std::sync::mpsc;
use std::thread;

lazy_static! {
    // Compute the md5 hash of the target number
    static ref TARGET_HASH: md5::Digest = md5::compute(100_000_000.to_string());
}

fn main() {
    time_it(|| naive_hashing());
    time_it(|| parallel_hashing(1));
    time_it(|| parallel_hashing(4));
    time_it(|| parallel_hashing(8));
    time_it(|| parallel_hashing(10));
    time_it(|| parallel_hashing(25));
    time_it(|| parallel_hashing(50));
    time_it(|| parallel_hashing(100));
}

fn naive_hashing() {
    print!("Naive hashing with single thread: ");

    // Iterate from 0 util we found the target number
    let mut i = 0_u32;
    loop {
        // Hash our guessing number and compare with the target hash
        // Stop the loop if we found the target number
        if md5::compute(i.to_string()).eq(&TARGET_HASH) {
            break;
        }
        // What's our next guess?
        i += 1;
    }
}

fn parallel_hashing(num_workers: usize) {
    // A MPSC channel to inform the main thread some worker found the solution
    let (tx, rx) = mpsc::channel();
    print!("Parallel hashing with {} threads: ", num_workers);

    // Spawn workers
    for worker in 0..num_workers {
        // Clone the transmission end and move it into the spawned worker
        let tx1 = tx.clone();
        thread::spawn(move || {
            // The guess number start from the worker number
            let mut i = worker;
            // Iterate from 0 util we found the target number
            loop {
                // Hash our guessing number and compare with the target hash
                let hash = md5::compute(i.to_string());
                // Stop the loop if we found the target number
                // And inform the main thread that we found the target number
                if hash.eq(&TARGET_HASH) {
                    tx1.send("Found!").unwrap();
                    break;
                }
                // What's our next guess?
                i += num_workers;
            }
        });
    }

    while let Ok(_) = rx.recv() {
        // Exit main thread, this will kill all spawn threads
        break;
    }
}

fn time_it<F, T>(f: F) -> T
where
    F: Fn() -> T,
{
    use std::time;
    // Mark the start time
    let start = time::SystemTime::now();
    // Perform the operation we want to time
    let result = f();
    // Record the end time and calculate the elapsed time in seconds
    let end = time::SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("took {} seconds", duration.as_secs_f64());
    result
}
