extern crate crossbeam;

use std::thread;
use crossbeam::crossbeam_channel::{Receiver, Sender, unbounded};
use crossbeam::sync::{WaitGroup};

const THREADS: usize = 4;

struct Info {
    n: i32, 
    s: String,
}

fn get_data()-> Vec<String> {
    let mut v: Vec<String> = Vec::new();
    for i in 0..10000 {
        let s = format!("s{}", i);
        v.push(s);
    }
    return v;
}

fn create_receiver<T: Clone>(vec: Vec<T>) -> Receiver<T> {
    let (s, r) = unbounded();
    for e in vec.iter() {
        s.send(e.clone()).unwrap();
    }
    return r;
}

fn process_data(i: i32, ss_r: Receiver<String>, info_s: Sender<Info>) {
    for s in ss_r.iter() {
        info_s.send(Info{n: i, s: s}).unwrap();
    }
    println!("process_data {} done.", i);
}

fn main() {
    let vec = get_data();
    let ss_r = create_receiver(vec);
    let (info_s, info_r) = unbounded();
    let wg = WaitGroup::new();

    for i in 0..THREADS {
        let wg0 = wg.clone();
        let ss_r0 = ss_r.clone();
        let info_s0 = info_s.clone();

        thread::spawn(move || {
            process_data(i as i32, ss_r0, info_s0);
            drop(wg0);
        });
    }
    wg.wait();

    println!(">> start info printing.");
    for info in info_r.iter() {
        println!("n: {}, s: {}", info.n, info.s);
        if info_r.is_empty() {
            break;
        }
    }
    println!("done.");
}
