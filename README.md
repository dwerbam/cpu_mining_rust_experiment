# CPU Mining Rust Experiment

This project is a simple experiment to demonstrate CPU mining using Rust.  It simulates mining by iterating through nonces to find a hash that meets a specified difficulty.

## How it works

The program utilizes multiple threads to parallelize the mining process, leveraging all available CPU cores. Each thread is assigned a portion of the nonce space to search.  The mining process involves hashing a fixed data block with each nonce using SHA256, and checking if the resulting hash meets the defined difficulty (number of leading zeros).

## Example
```sh
Mining with 8 threads
Mining block with batch_id: 0 0 536870911
Mining block with batch_id: 2 1073741822 1610612733
Mining block with batch_id: 1 536870911 1073741822
Mining block with batch_id: 3 1610612733 2147483644
Mining block with batch_id: 4 2147483644 2684354555
Mining block with batch_id: 5 2684354555 3221225466
Mining block with batch_id: 7 3758096377 4294967288
Mining block with batch_id: 6 3221225466 3758096377

▒▒▄▀▀▀▀▀▄▒▒▒▒▒▄▄▄▄▄▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒ 
▒▐░▄░░░▄░▌▒▒▄█▄█▄█▄█▄▒░█▀▀▄░█░░▄▀▀▄░█▀▄░█░▄░░░▒█▀▄▀█░▀█▀░▒█▄░▒█░▒█▀▀▀░▒█▀▀▄▒▒
▒▐░▀▀░▀▀░▌▒▒▒▒▒░░░▒▒▒▒░█▀▀▄░█░░█░░█░█░░░█▀▄░░░▒█▒█▒█░▒█░░▒█▒█▒█░▒█▀▀▀░▒█░▒█▒▒
▒▒▀▄░═░▄▀▒▒▒▒▒▒░░░▒▒▒▒░▀▀▀▀░▀▀░░▀▀░░▀▀▀░▀░▀░░░▒█░░▒█░▄█▄░▒█░░▀█░▒█▄▄▄░▒█▄▄█▒▒
▒▒▐░▀▄▀░▌▒▒▒▒▒▒░░░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
                
Block mined! Nonce: 2823679306
BatchID:5 00000000b010b1dcac70839a04a193c8aca116f802355255f31a5694cfa1cb77
Elapsed time: 38s
3666440.8157894737 H/s x *processor*
```

## Usage

1. **Clone the repository:** `git clone <repository_url>`
2. **Navigate to the project directory:** `cd cpu_mining_rust_experiment`
3. **Build the project:** `cargo build`
4. **Run the project:** `cargo run`

The output will show the mining progress and report the nonce and hash when a block is "mined" (i.e., the hash meets the difficulty requirement).  The program will then terminate.

## Configuration

The difficulty is currently set to a low value for testing purposes (8 leading zeroes).  This can be adjusted by changing the `DIFFICULTY_HEX` constant in `src/blockbatch.rs`.

## Dependencies

- `hex`: For hex encoding/decoding.
- `sha2`: For SHA256 hashing.

