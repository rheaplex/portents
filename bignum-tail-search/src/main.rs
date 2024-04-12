use std::fs::File;
use std::io::{BufRead, BufReader};
use compare::{Compare, natural};
use num_bigint::BigUint;
use ordinals::Sat;

// Actually as of 10th april 2024
const SATS_AT_HALVENING_2024: u64 = (19678777.604 * 100000000.0) as u64;

fn main() {
    //let halvening_bignum: BigUint = SATS_AT_HALVENING_2024.into();
    let file = File::open("../blocks.csv").expect("Couldn't open source file");
    let lines = BufReader::new(file).lines();
    let cmp = natural();
    println!("block_index,block_hash,block_lsbs_hex,block_lsbs_for_integer_sat,sat_name,sat_block_heigh,sat_block_offset,block_index_gle_sat_block_index");
    for line in lines.flatten() {
        let mut fields = line.split(',');
        let index = fields.next().unwrap();
        let hash = fields.next().unwrap();
        let x = BigUint::parse_bytes(hash.as_bytes(), 16).unwrap();
        let lsu64 = x.to_u64_digits()[0];
        if lsu64 <= SATS_AT_HALVENING_2024 {
            let sat = Sat(lsu64);
            let decimal = sat.decimal();
            println!(
                "{},{},{:x},{},{},{},{},{:?}",
                index,
                hash,
                lsu64,
                lsu64,
                sat.name(),
                decimal.height,
                decimal.offset,
                cmp.compare(&index.parse::<u32>().unwrap(), &decimal.height.n())
            );
        }
    }
}
