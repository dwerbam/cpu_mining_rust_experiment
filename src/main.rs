use std::thread;
use std::thread::available_parallelism;
use std::sync::mpsc::channel;
mod blockbatch; 

fn main() {
    // ~ M1 Macbook Air ~ 30 million nonces per second
    // ~ 0,00003 TH/s
    const MAX_NONCE: u32 = u32::MAX;
    let cpus: usize = available_parallelism().unwrap().get();

    println!("Mining with {} threads", cpus);

    let (tx, rx) = channel();
    for i in 0..cpus {
        let tx = tx.clone();
        thread::spawn(move || {
            let nonce_ini = i as u32 * (MAX_NONCE / cpus as u32);
            let nonce_end = (i as u32 + 1) * (MAX_NONCE / cpus as u32);
            // for this experiment we are using 76 bytes of "twos", but feel free to change it
            let mut block = blockbatch::BlockBatch::new([2; 76], nonce_ini, nonce_end);
            block.set_batch_id(i as u8);
            block.mine();
            tx.send(()).unwrap();
        });
    }

    for _ in 0..cpus {
        rx.recv().unwrap();
    }
}