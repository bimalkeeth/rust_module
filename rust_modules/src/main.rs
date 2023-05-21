#![allow(unused)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_variables)]


use std::env;
use std::path::Path;

mod match_expr;
mod filter;


fn main() {
    let image_path = env::args().
        skip(1).next().
        unwrap_or("peakpx.jpg".to_string());

    let path = Path::new(&image_path);
    let img = image::open(path).unwrap();
    let rorated = img.rotate90();
    rorated.save("peakpxq.jpg".to_string()).unwrap()
}

pub fn slow_fibonacci(nth: usize) -> u64 {
    if nth <= 1 {
        return nth as u64;
    } else {
        return slow_fibonacci(nth - 1) + slow_fibonacci(nth - 2);
    }
}

pub fn fast_fibonacci(nth: usize) -> u64 {
    let mut a = 0;
    let mut b = 1;
    let mut c = 0;
    for _ in 1..nth {
        c = a + b;
        a = b;
        b = c;
    }
    c
}
