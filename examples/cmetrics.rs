use std::{thread, time::Duration};

use concurrency::metrics::cmap::CmapMetrics;
use rand::Rng;

const N: usize = 2;
const M: usize = 4;

fn main() -> anyhow::Result<()> {
    let metrics = CmapMetrics::new();

    println!("{}", metrics);

    for index in 0..N {
        task_worker(index, metrics.clone());
    }

    for _ in 0..M {
        request_worker(metrics.clone());
    }

    loop {
        thread::sleep(Duration::from_secs(2));
        println!("{}", metrics);
    }
}

#[allow(unreachable_code)]
fn task_worker(index: usize, metrics: CmapMetrics) {
    thread::spawn(move || loop {
        let mut rng = rand::thread_rng();
        thread::sleep(Duration::from_millis(rng.gen_range(100..5000)));
        metrics.inc(format!("call.thread.worker.{}", index));
    });
}

fn request_worker(metrics: CmapMetrics) {
    thread::spawn(move || loop {
        let mut rng = rand::thread_rng();
        thread::sleep(Duration::from_millis(rng.gen_range(50..800)));
        let page = rng.gen_range(1..5);
        metrics.inc(format!("req.page.{}", page));
    });
}
