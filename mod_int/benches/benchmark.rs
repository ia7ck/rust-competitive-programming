use criterion::measurement::WallTime;
use criterion::BatchSize::SmallInput;
use criterion::{criterion_group, criterion_main, BenchmarkGroup, BenchmarkId, Criterion};
use mod_int::ModInt1000000007;
use rand::prelude::*;

const SIZES: [usize; 2] = [1_000, 5_000];
type Mint = ModInt1000000007;

fn generate_random_mints(size: usize) -> Vec<(Mint, Mint)> {
    let mut rng = thread_rng();
    let dist = rand::distributions::Uniform::new_inclusive(std::i64::MIN, std::i64::MAX);
    (0..size)
        .map(|_| {
            let x = Mint::new(rng.sample(dist));
            let y = loop {
                let y = rng.sample(dist);
                if y % Mint::mo() != 0 {
                    break Mint::new(y);
                }
            };
            (x, y)
        })
        .collect()
}

fn bench_f(
    group: &mut BenchmarkGroup<WallTime>,
    size: usize,
    name: &str,
    f: &dyn Fn(Mint, Mint) -> Mint,
) {
    group.bench_with_input(
        BenchmarkId::new(name, format!("{}", size)),
        &size,
        |b, &size| {
            b.iter_batched(
                || generate_random_mints(size),
                |mints| {
                    for (x, y) in mints {
                        let _ = f(x, y);
                    }
                },
                SmallInput,
            );
        },
    );
}

pub fn mod_int_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("ModInt");
    for &size in SIZES.iter() {
        bench_f(&mut group, size, "Addition", &|x, y| x + y);
        bench_f(&mut group, size, "Subtraction", &|x, y| x - y);
        bench_f(&mut group, size, "Multiplication", &|x, y| x * y);
        bench_f(&mut group, size, "Division", &|x, y| x / y);

        // exp
        let mut rng = thread_rng();
        let dist = rand::distributions::Uniform::new_inclusive(std::i64::MIN, std::i64::MAX);
        let dist_u64 = rand::distributions::Uniform::new_inclusive(std::u64::MIN, std::u64::MAX);
        group.bench_with_input(
            BenchmarkId::new("Exponentiation", format!("{}", size)),
            &size,
            |b, &size| {
                b.iter_batched(
                    || {
                        (0..size)
                            .map(|_| {
                                let x = Mint::new(rng.sample(dist));
                                let y = rng.sample(dist_u64);
                                (x, y)
                            })
                            .collect()
                    },
                    |mint_u64: Vec<(Mint, u64)>| {
                        for (x, y) in mint_u64 {
                            let _ = x.pow(y);
                        }
                    },
                    SmallInput,
                );
            },
        );
    }
    group.finish();
}

criterion_group!(benches, mod_int_benchmark);
criterion_main!(benches);
