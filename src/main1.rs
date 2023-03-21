use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread, time};

use rand::Rng;

fn main() {
    let stop = Arc::new(AtomicBool::new(false));
    println!("initial stop value {:?}", &stop);
    let stop1 = stop.clone();

    let mut workers = vec![];
    workers.push(thread::spawn(move || {
        let mut i = 0;
        loop {
            i += 1;
            let r1: u64 = rand::thread_rng().gen_range(0..50);
            let delay = time::Duration::from_millis(r1);
            thread::sleep(delay);
            let load = stop1.load(Ordering::Relaxed);
            println!("thread 1 load {}: {}", &i, &load);
            if load {
                println!("thread 1 iterations = {}", i);
                break;
            }
        }
    }));
    let stop2 = stop.clone();
    workers.push(thread::spawn(move || {
        let delay = time::Duration::from_secs(2);
        thread::sleep(delay);
        stop2.store(true, Ordering::Relaxed);
        println!("thread 2 changed stop value to {:?}", &stop2);
    }));
    workers.push(thread::spawn(move || {
        let mut j = 0;
        loop {
            j += 1;
            let r2: u64 = rand::thread_rng().gen_range(0..20);
            let delay = time::Duration::from_millis(r2);
            thread::sleep(delay);
            let load = stop.load(Ordering::Relaxed);
            println!("thread 3 load {}: {}", &j, &load);
            if load {
                println!("thread 3 iterations = {}", j);
                break;
            }
        }
    }));

    for worker in workers {
        // Wait for the thread to finish. Returns a result.
        let _ = worker.join();
    }

    println!("All done");
}
