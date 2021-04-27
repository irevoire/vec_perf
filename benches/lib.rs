#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench_push(b: &mut Bencher) {
    let mut vec = Vec::new();

    b.iter(|| {
        for _ in 0..1000 {
            vec.push(black_box(0));
        }
    });
}

#[bench]
fn bench_insert(b: &mut Bencher) {
    let mut vec = Vec::new();

    b.iter(|| {
        for _ in 0..1000 {
            vec.insert(vec.len(), black_box(0));
        }
    });
}
