use ark_circom::read_zkey;
use criterion::{criterion_group, criterion_main, Criterion};
use std::io::Cursor;

pub fn zkey_load_benchmark(c: &mut Criterion) {
    let zkey = rln::circuit::ZKEY_BYTES.to_vec();
    let size = zkey.len() as f32;
    println!("Size of zkey: {:.2?} MB", size / 1024.0 / 1024.0);

    c.bench_function("zkey::zkey_from_raw", |b| {
        b.iter(|| {
            let mut reader = Cursor::new(zkey.clone());
            let r = read_zkey(&mut reader);
            assert_eq!(r.is_ok(), true);
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(std::time::Duration::from_secs(250));
    targets = zkey_load_benchmark
}
criterion_main!(benches);
