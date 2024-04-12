// Actually as of 10th april 2024
const SATS_AT_HALVENING_2024: u64 = (19678777.604 * 100000000.0) as u64;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
pub struct Sat(pub u64);

impl Sat {
    pub const LAST: Self = Self(Self::SUPPLY - 1);
    pub const SUPPLY: u64 = 2099999997690000;

    pub fn name(self) -> String {
        let mut x = Self::SUPPLY - self.0;
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
}


fn main() {
 for satoshi in 0..SATS_AT_HALVENING_2024 {
        let sat = Sat(1);
        println!("{},{}", satoshi, sat.name());
    }
}
