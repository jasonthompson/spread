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
}

mod tests {
  use std::num;
  use super::*;

  struct TestSpread {
    low: int,
    high: int
  }

  impl TestSpread {
    fn new(low: int, high: int) -> TestSpread {
      TestSpread {
        low: low,
        high: high
      }
    }
  }

  impl super::Spread for TestSpread {
    fn spread(&self) -> int {
      num::abs(self.high - self.low)
    }
  }

  #[test]
  fn test_find_spread() {
    let s: TestSpread = TestSpread::new(10, 4);
    assert_eq!(6i, s.spread());
  }
}
