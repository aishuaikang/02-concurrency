use std::{thread, time::Duration};

use concurrency::metrics::Metrics;
use rand::Rng;

const N: usize = 2;
const M: usize = 4;

fn main() -> anyhow::Result<()> {
    let metrics = Metrics::new();

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

    // (0..100).for_each(|index| {
    //     metrics.inc("req.page.1");
    //     metrics.inc("req.page.2");
    //     if index % 2 == 0 {
    //         metrics.inc("req.page.3");
    //     }
    // });

    // (0..27).for_each(|_| {
    //     metrics.inc("call.thread.worker.1");
    // });

    // println!("{:?}", metrics.snapshot());

    // Ok(())
}

#[allow(unreachable_code)]
fn task_worker(index: usize, metrics: Metrics) {
    thread::spawn(move || -> anyhow::Result<()> {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(100..5000)));
            metrics.inc(format!("call.thread.worker.{}", index))?;
        }
        Ok(())
    });
}

fn request_worker(metrics: Metrics) {
    thread::spawn(move || -> anyhow::Result<()> {
        let mut rng = rand::thread_rng();
        thread::sleep(Duration::from_millis(rng.gen_range(50..800)));
        let page = rng.gen_range(1..5);
        metrics.inc(format!("req.page.{}", page))?;
        Ok(())
    });
}
