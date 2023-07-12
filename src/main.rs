use std::ops::Sub;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::thread::{self, JoinHandle};
use std::time::Instant;
const N_TIMES: u64 = 1_000_000;
const N_THREADS: usize = 4;
static R: AtomicU64 = AtomicU64::new(0);
static m: Mutex<u64> = Mutex::new(0);
fn add_n_times(n: u64) -> JoinHandle<()> {
    thread::spawn(move || {
        for _ in 0..n {
            R.fetch_add(1, Ordering::SeqCst);
        }
    })
}

fn add_n_times_mutex(n: u64) -> JoinHandle<()> {
    thread::spawn(move || {
        for _ in 0..n {
            let mut r = m.lock().unwrap();
            *r += 1;
        }
    })
}

// 每个线程对全局变量加一
fn main() {
    // Atomic, cost 81.6156ms
    let s = Instant::now();
    let mut threads = Vec::with_capacity(N_THREADS);
    for _ in 0..N_THREADS {
        threads.push(add_n_times(N_TIMES));
    }
    for thread in threads {
        thread.join().unwrap();
    }
    assert_eq!(N_TIMES * N_THREADS as u64, R.load(Ordering::Relaxed));
    println!("Atomic 实现: {:?}", Instant::now().sub(s));

    // Mutex, cost 302.1519ms
    let s = Instant::now();
    let mut threads = Vec::with_capacity(N_THREADS);
    for _ in 0..N_THREADS {
        threads.push(add_n_times_mutex(N_TIMES));
    }
    for thread in threads {
        thread.join().unwrap();
    }
    assert_eq!(N_TIMES * N_THREADS as u64, *m.lock().unwrap());
    println!("Mutex 实现: {:?}", Instant::now().sub(s));

    println!("Hello, world!");
}