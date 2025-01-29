# CPU Mining Rust Experiment

This project is a simple experiment to demonstrate CPU mining using Rust.  It simulates mining by iterating through nonces to find a hash that meets a specified difficulty.

## How it works

The program utilizes multiple threads to parallelize the mining process, leveraging all available CPU cores. Each thread is assigned a portion of the nonce space to search.  The mining process involves hashing a fixed data block with each nonce using SHA256, and checking if the resulting hash meets the defined difficulty (number of leading zeros).

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

