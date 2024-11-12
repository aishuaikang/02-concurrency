use std::{sync::mpsc, thread};

const NUM_THREADS: u32 = 4;

#[derive(Debug)]
struct Msg {
    idx: u32,
    val: usize,
}

impl Msg {
    fn new(idx: u32, val: usize) -> Self {
        Msg { idx, val }
    }
}

fn main() -> anyhow::Result<()> {
    let (tx, rx) = mpsc::channel();

    for i in 0..NUM_THREADS {
        let tx = tx.clone();
        thread::spawn(move || producer(tx, i));
    }

    drop(tx);

    let receiver = thread::spawn(move || {
        let mut last_msg = Msg::new(0, 0);
        for msg in rx {
            println!("idx:{} val:{}", msg.idx, msg.val);
            last_msg = msg;
        }
        println!("Receiver is done");
        last_msg
    });

    println!("Waiting for the message...");
    let last_msg = receiver.join().map_err(|e| anyhow::anyhow!("{:?}", e))?;
    println!("Last message: idx:{} val:{}", last_msg.idx, last_msg.val);
    Ok(())
}

fn producer(tx: mpsc::Sender<Msg>, idx: u32) -> anyhow::Result<()> {
    loop {
        let val = rand::random::<usize>();
        let msg = Msg::new(idx, val);
        tx.send(msg)?;

        let r = rand::random::<u8>();
        let sleep_time = r as u64 * 10;
        thread::sleep(std::time::Duration::from_millis(sleep_time));

        if r % 10 == 0 {
            break;
        }
    }

    println!("Thread {} is done", idx);

    Ok(())
}
