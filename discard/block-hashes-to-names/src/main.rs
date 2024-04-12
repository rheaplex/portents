use num_bigint::BigUint;
use std::fs::File;
use std::io::{self, BufRead};

const SUPPLY: u64 = 2099999997690000;

 fn name_u64(value: u64) -> String {
    let mut x = SUPPLY - value;
    let mut name = String::new();
    while x > 0 {
      name.push(
        "abcdefghijklmnopqrstuvwxyz"
          .chars()
          .nth(((x - 1) % 26) as usize)
          .unwrap(),
      );
      x = (x - 1) / 26;
    }
    name.chars().rev().collect()
  }

fn name(hash: &[u8]) -> String {
    let digits = BigUint::parse_bytes(hash, 16).unwrap().to_u64_digits();
    format!("{}{}{}{}", name_u64(digits[3]), name_u64(digits[2]), name_u64(digits[1]), name_u64(digits[0]))
}

fn main() {
    let file = File::open("../blocks.csv").expect("Couldn't open source file");
    let lines = io::BufReader::new(file).lines();
    for line in lines.flatten() {
        let mut fields = line.split(',');
        let index = fields.next().unwrap();
        let hash = fields.next().unwrap();
        println!("{},{},{}", index, hash, name(hash.as_bytes()));
        break;
    }
}
