extern crate tokio;
use tokio::sync::mpsc;
extern crate async_std;
use async_std::channel;
use criterion::{criterion_group, criterion_main, Criterion, async_executor::AsyncStdExecutor};

pub async fn channel_tokio(num_threads: u64, size_buf: usize) {
	let (tx, mut rx) = mpsc::channel::<u64>(size_buf);

    let task = tokio::spawn(async move {
        for _ in 0u64..num_threads {
            let _ = rx.recv().await.unwrap();
        }
    });

	for i in 0u64..num_threads {

		let tx = tx.clone();

		tokio::spawn(async move {
			let _ = tx.send(i).await;
		});
	}
    let _ = task.await;
}

fn benchmark_tokio(criterion: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    {
        criterion.bench_function("tokio test 10 threads 1 buf size", |bench| {
            bench.to_async(&runtime).iter(|| async { channel_tokio(10, 1).await });
        });
    }

    let buf_sizes: Vec<usize> = vec![1, 10, 100, 1000, 10000];
    for buf_size in buf_sizes.into_iter() {
        criterion.bench_function(&format!("tokio test 10000 threads {} buf size", buf_size), |bench| {
            bench.to_async(&runtime).iter(|| async { channel_tokio(10000, buf_size).await });
        });
    }
}

pub async fn channel_async_std(num_threads: u64, size_buf: usize) {
	let (tx, rx) = channel::bounded::<u64>(size_buf);

	for i in 0u64..num_threads {

		let tx = tx.clone();

		async_std::task::spawn(async move {
			let _ = tx.send(i).await;
		});
	}

    for _ in 0u64..num_threads {
        let _ = rx.recv().await.unwrap();
    }
}

fn benchmark_async_std(criterion: &mut Criterion) {
    {
        criterion.bench_function("async-std test 10", |bench| {
            bench.to_async(AsyncStdExecutor).iter(|| async { channel_async_std(10, 1).await });
        });
    }

	let buf_sizes: Vec<usize> = vec![1, 10, 100, 1000, 10000];
    for buf_size in buf_sizes.into_iter() {
        criterion.bench_function(&format!("async-std test 10000 threads {} buf size", buf_size), |bench| {
            bench.to_async(AsyncStdExecutor).iter(|| async { channel_async_std(10000, buf_size).await });
        });
    }
}

criterion_group!(benches, benchmark_tokio, benchmark_async_std);
criterion_main!(benches);
