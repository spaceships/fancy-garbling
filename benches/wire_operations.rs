#[macro_use]
extern crate criterion;
extern crate fancy_garbling;
extern crate rand;

use criterion::Criterion;
use std::time::Duration;

use fancy_garbling::wire::Wire;
use fancy_garbling::util::RngExt;

fn bench_unpack(c: &mut Criterion, p: u16) {
    c.bench_function(&format!("wire::unpack{}", p), move |b| {
        let rng = &mut rand::thread_rng();
        let x = rng.gen_u128();
        b.iter(|| {
            let w = Wire::from_u128(x, p);
            criterion::black_box(w);
        });
    });
}

fn bench_pack(c: &mut Criterion, p: u16) {
    c.bench_function(&format!("wire::pack{}", p), move |b| {
        let rng = &mut rand::thread_rng();
        let w = Wire::rand(rng,p);
        b.iter(|| {
            let x = w.as_u128();
            criterion::black_box(x);
        });
    });
}

fn bench_plus(c: &mut Criterion, p: u16) {
    c.bench_function(&format!("wire::plus{}", p), move |b| {
        let rng = &mut rand::thread_rng();
        let x = Wire::rand(rng,p);
        let y = Wire::rand(rng,p);
        b.iter(|| {
            let z = x.plus(&y);
            criterion::black_box(z);
        });
    });
}

fn bench_plus_eq(c: &mut Criterion, p: u16) {
    c.bench_function(&format!("wire::plus_eq{}", p), move |b| {
        let rng = &mut rand::thread_rng();
        let mut x = Wire::rand(rng,p);
        let y = Wire::rand(rng,p);
        b.iter(|| {
            x.plus_eq(&y);
        });
    });
}

fn bench_cmul(c: &mut Criterion, p: u16) {
    c.bench_function(&format!("wire::cmul{}", p), move |b| {
        let rng = &mut rand::thread_rng();
        let x = Wire::rand(rng,p);
        let c = rng.gen_u16();
        b.iter(|| {
            let z = x.cmul(c);
            criterion::black_box(z);
        });
    });
}

fn bench_cmul_eq(c: &mut Criterion, p: u16) {
    c.bench_function(&format!("wire::cmul_eq{}", p), move |b| {
        let rng = &mut rand::thread_rng();
        let mut x = Wire::rand(rng,p);
        let c = rng.gen_u16();
        b.iter(|| {
            x.cmul_eq(c);
            // criterion::black_box(x);
        });
    });
}

fn bench_negate(c: &mut Criterion, p: u16) {
    c.bench_function(&format!("wire::negate{}", p), move |b| {
        let rng = &mut rand::thread_rng();
        let x = Wire::rand(rng,p);
        b.iter(|| {
            let z = x.negate();
            criterion::black_box(z);
        });
    });
}

fn bench_hash(c: &mut Criterion, p: u16) {
    c.bench_function(&format!("wire::hash{}", p), move |b| {
        let rng = &mut rand::thread_rng();
        let x = Wire::rand(rng,p);
        b.iter(|| {
            let z = x.hash(42);
            criterion::black_box(z);
        });
    });
}

fn bench_zero(c: &mut Criterion, p: u16) {
    c.bench_function(&format!("wire::zero{}", p), move |b| {
        b.iter(|| {
            let z = Wire::zero(p);
            criterion::black_box(z);
        });
    });
}


fn unpack17(c: &mut Criterion) { bench_unpack(c,17) }
fn pack17(c: &mut Criterion) { bench_pack(c,17) }
fn plus17(c: &mut Criterion) { bench_plus(c,17) }
fn pluseq17(c: &mut Criterion) { bench_plus_eq(c,17) }
fn cmul17(c: &mut Criterion) { bench_cmul(c,17) }
fn cmuleq17(c: &mut Criterion) { bench_cmul_eq(c,17) }
fn negate17(c: &mut Criterion) { bench_negate(c,17) }
fn hash17(c: &mut Criterion) { bench_hash(c,17) }
fn zero17(c: &mut Criterion) { bench_zero(c,17) }

criterion_group!{
    name = wire_conversion;
    config = Criterion::default().warm_up_time(Duration::from_millis(100));
    targets = unpack17, pack17, plus17, pluseq17, cmul17, cmuleq17, negate17, hash17, zero17
}

criterion_main!(wire_conversion);
