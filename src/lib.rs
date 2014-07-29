// Part Three: DRY Fusion
//
// Take the two programs written previously and factor
// out as much common code as possible, leaving you with two smaller programs
// and some kind of shared functionality.
//
#![crate_name="spread"]
#![crate_type="rlib"]
#![feature(globs)]

pub trait Spread {
  fn spread(&self) -> int;
  fn new(name: &'static str, lowest: int, highest: int)-> Self;
}

mod tests {
  use std::num;
  use std::string;
  use super::*;

  struct TestSpread {
    name: string::String,
    low: int,
    high: int
  }

  impl super::Spread for TestSpread {
    fn spread(&self) -> int {
      num::abs(self.high - self.low)
    }

    fn new(name: &'static str, low: int, high: int) -> TestSpread {
      TestSpread { name: string::String::from_str(name), low: low, high: high}
    }
  }

  #[test]
  fn test_find_spread() {
    let s: &TestSpread = &Spread::new("Test", 10, 4);
    assert_eq!(6i, s.spread());
  }
}
