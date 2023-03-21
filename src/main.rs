use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::{thread, time};

use rand::Rng;

fn main() {
    let (tx, rx) = mpsc::channel();
    let stop = Arc::new(AtomicBool::new(false));
    println!("initial stop value {:?}", &stop);
    let mut workers = vec![];

    let stop1 = stop.clone();
    let tx1 = tx.clone();
    workers.push(thread::spawn(move || {
        let mut i = 0;
        loop {
            i += 1;
            let r1: u64 = rand::thread_rng().gen_range(50..100);
            let delay = time::Duration::from_millis(r1);
            thread::sleep(delay);
            let load = stop1.load(Ordering::Relaxed);
            let msg = format!("thread 1 load {}: {}", &i, &load);
            tx1.send(msg).unwrap();
            if load {
                // println!("thread 1 iterations = {}", i);
                let msg = format!("thread 1 iterations = {}", i);
                tx1.send(msg).unwrap();
                break;
            }
        }
    }));

    let stop2 = stop.clone();
    // let tx3 = tx.clone();
    workers.push(thread::spawn(move || {
        let delay = time::Duration::from_secs(1);
        thread::sleep(delay);
        stop2.store(true, Ordering::Relaxed);
        println!("thread 2 changed stop value to {:?}", &stop2);
        // let msg = format!("thread 2 changed stop value to {:?}", &stop2);
        // tx3.send(msg).unwrap();
    }));

    let stop3 = stop.clone();
    let tx2 = tx.clone();
    workers.push(thread::spawn(move || {
        let mut j = 0;
        loop {
            j += 1;
            let r2: u64 = rand::thread_rng().gen_range(25..75);
            let delay = time::Duration::from_millis(r2);
            thread::sleep(delay);
            let load = stop3.load(Ordering::Relaxed);
            let msg = format!("thread 3 load {}: {}", &j, &load);
            tx2.send(msg).unwrap();

            if load {
                // println!("thread 3 iterations = {}", j);
                let msg = format!("thread 3 iterations = {}", j);
                tx2.send(msg).unwrap();
                break;
            }
        }
    }));

    workers.push(thread::spawn(move || {
        let mut k = 0;
        loop {
            k += 1;
            let r: u64 = rand::thread_rng().gen_range(0..50);
            let delay = time::Duration::from_millis(r);
            thread::sleep(delay);
            let load = stop.load(Ordering::Relaxed);
            let msg = format!("thread 4 load {}: {}", &k, &load);
            tx.send(msg).unwrap();

            if load {
                let msg = format!("thread 4 iterations = {}", k);
                tx.send(msg).unwrap();
                break;
            }
        }
    }));

    workers.push(thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            println!("Got: {}", msg);
        }
    }));
    for worker in workers {
        // Wait for the thread to finish. Returns a result.
        let _ = worker.join();
    }

    // let handle = thread::spawn(move || loop {
    //     if let Ok(msg) = rx.recv() {
    //         println!("Got: {}", msg);
    //     } else {
    //         break;
    //     }
    // });
    // let _ = handle.join();
    // for received in rx {
    //     println!("Got: {}", received);
    // }
    println!("All done");
}
