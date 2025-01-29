use sha2::{Sha256, Digest};
use hex;

// WARN: Currently bitcoin is using 19 Zeroes (76 bits) as difficulty;
// E.g.:
// 00000000000000000000f987ca9742b46bbf4cc6512935b7bb8487bcf3c25c4e
// 0000000000000000000140110cbbba9a8a37dc2af0b35bb943dbe2083a50576f
// 00000000000000000000793cf3283928189f60b785e34c507f51f9a83f05f5fe
// 00000000000000000000ff9e515a568e3b6bb2be35f32634bc5d13107fc2a0e9
// 00000000000000000000f987ca9742b46bbf4cc6512935b7bb8487bcf3c25c4e
// 00000000000000000002255b086bba79f3a74455b0115fb0e635e3799b43cf3b
// 12345678901234567890

const DIFFICULTY_HEX: usize = 8;  //testing with 8 zeroes

const DIFFICULTY_BYTES: usize = DIFFICULTY_HEX / 2;
const ZEROES: [u8; DIFFICULTY_BYTES] = [0;DIFFICULTY_BYTES];

const SUCCESS_MESSAGE:&str = r#"
▒▒▄▀▀▀▀▀▄▒▒▒▒▒▄▄▄▄▄▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ 
▒▐░▄░░░▄░▌▒▒▄█▄█▄█▄█▄▒░█▀▀▄░█░░▄▀▀▄░█▀▄░█░▄░░░▒█▀▄▀█░▀█▀░▒█▄░▒█░▒█▀▀▀░▒█▀▀▄▒▒
▒▐░▀▀░▀▀░▌▒▒▒▒▒░░░▒▒▒▒░█▀▀▄░█░░█░░█░█░░░█▀▄░░░▒█▒█▒█░▒█░░▒█▒█▒█░▒█▀▀▀░▒█░▒█▒▒
▒▒▀▄░═░▄▀▒▒▒▒▒▒░░░▒▒▒▒░▀▀▀▀░▀▀░░▀▀░░▀▀▀░▀░▀░░░▒█░░▒█░▄█▄░▒█░░▀█░▒█▄▄▄░▒█▄▄█▒▒
▒▒▐░▀▄▀░▌▒▒▒▒▒▒░░░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
                "#;

pub struct BlockBatch {
    batch_id: u8,
    data: [u8; 76],
    nonce_ini: u32,
    nonce_end: u32,
}

impl BlockBatch {
    pub fn new(data: [u8; 76], nonce_ini: u32, nonce_end: u32) -> Self {
        BlockBatch {
            batch_id: 0,
            data,
            nonce_ini,
            nonce_end,
        }
    }

    pub fn set_batch_id(&mut self, batch_id: u8) {
        self.batch_id = batch_id;
    }

    pub fn mine(&self) {
        let mut hasher = Sha256::new();
        hasher.update(&self.data);
        println!("Mining block with batch_id: {} {} {}", self.batch_id, self.nonce_ini, self.nonce_end);
        //save timestamp to measure time
        let start = std::time::Instant::now();
        for nonce in self.nonce_ini..self.nonce_end {
            let mut hasher_clone = hasher.clone();
            hasher_clone.update(&nonce.to_le_bytes());
            let result = hasher_clone.finalize();


            // TODO: everything on this loop should be very lightway 
            //       (I'm talking about the verification method). 
            //       Check if this is the most efficient way to do this.
            //       TODO: there is a bug here when DIFFICULTY_HEX is odd
            if result[..DIFFICULTY_BYTES] == ZEROES {
                println!("{}",SUCCESS_MESSAGE);
                println!("Block mined! Nonce: {}", nonce);
                println!("BatchID:{} {}", self.batch_id, hex::encode(result));

                // TODO: send mined block to mining pool?

                //estimate the hashrate based on the elapsed time and nonce-nonce_ini
                println!("Elapsed time: {}s", start.elapsed().as_secs());
                
                // show hashes per second
                let hashes_per_sec = (nonce - self.nonce_ini) as f64 / start.elapsed().as_secs() as f64;
                println!("{} H/s x *processor*", hashes_per_sec);

                //quit program (to stop all threads)
                std::process::exit(0);

            }
        }
    }
}
