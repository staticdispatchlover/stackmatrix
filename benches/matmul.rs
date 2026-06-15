use criterion::{black_box, criterion_group, criterion_main, Criterion};
use stackmatrix::{Matrix, MatrixOps};

fn matmul_benchmark(c: &mut Criterion) {
    let a: Matrix<10, 10> = Matrix {
        data: [[1.0; 10]; 10],
    };

    let b: Matrix<10, 10> = Matrix {
        data: [[2.0; 10]; 10],
    };

    c.bench_function("matmul 10x10", |bencher| {
        bencher.iter(|| {
            let result = black_box(a.matmul(black_box(b)));
            black_box(result);
        });
    });
}

criterion_group!(benches, matmul_benchmark);
criterion_main!(benches);
