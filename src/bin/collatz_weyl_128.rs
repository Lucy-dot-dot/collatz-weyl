//! Command-line front-end for [`CollatzWeyl128`].
//!
//! Usage:
//!
//! ```text
//! collatz_weyl_128 <seed> [count]
//! ```
//!
//! `seed` is mandatory and must be an odd `u128` (decimal, or hex with a `0x`
//! prefix). `count` is optional and defaults to 10. Each generated value is
//! printed on its own line.

use std::io::Write;
use collatz_weyl::CollatzWeyl128;
use std::process::ExitCode;

const PROG: &str = "collatz_weyl_128";

fn parse_u128(s: &str) -> Result<u128, String> {
    let s = s.trim();
    let value = if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        u128::from_str_radix(hex, 16)
    } else {
        s.parse::<u128>()
    };
    value.map_err(|_| format!("`{s}` is not a valid u128"))
}

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);

    let seed_arg = match args.next() {
        Some(seed_arg) => seed_arg,
        None => {
            eprintln!("missing seed\nusage: {PROG} <seed>");
            return ExitCode::FAILURE;
        }
    };
    let seed = match parse_u128(&seed_arg) {
        Ok(seed) => seed,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
    };
    if seed & 1 == 0 {
        eprintln!("seed must be odd, got {seed}");
        return ExitCode::FAILURE;
    }


    let mut rng = CollatzWeyl128::new(seed);
    let mut out = std::io::stdout().lock();
    loop {
        if out.write_all(&rng.next().to_le_bytes()).is_err() {
            break;
        }
    }
    ExitCode::SUCCESS
}
