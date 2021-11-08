use curv_kzen::arithmetic::{Converter, Samplable};
use curv_kzen::BigInt;
use std::fmt;
use std::ops::{Index, IndexMut};

use crate::Collector;

/// Matrix used by the collectors for (n,n) Secret Sharing
pub struct SharesMatrix {
  num_voters: usize,
  modulus: BigInt,
  matrix: Vec<BigInt>,
}

impl SharesMatrix {
  pub fn new(collector: Collector, num_voters: usize, modulus: BigInt) -> Self {
    let mut matrix = SharesMatrix {
      num_voters,
      modulus,
      matrix: vec![BigInt::from(0); num_voters * num_voters],
    };

    // Fill in the matrix halves
    match collector {
      Collector::One => matrix.fill_collector_one(),
      Collector::Two => matrix.fill_collector_two(),
    }

    matrix
  }

  pub fn get_num_voters(&self) -> usize {
    self.num_voters
  }

  pub fn get_modulus(&self) -> &BigInt {
    &self.modulus
  }

  /// Get the sum of shares for a given voter to use for the ballot.
  /// For our protocol, this is the sum of the column.
  ///
  /// Returns nothing if the voter is outside the range of the number of voters.
  pub fn get_ballot_shares(&self, voter: usize) -> Option<BigInt> {
    if voter >= self.num_voters {
      return None;
    }

    let mut sum = BigInt::from(0);
    for row in 0..self.num_voters {
      sum += &self[(row, voter)];
    }

    Some(sum % &self.modulus)
  }

  /// Get the sum of shares for a given voter to use for voter verification.
  /// For our protocol, this is the sum of the row.
  ///
  /// Returns nothing if the voter is outside the range of the number of voters.
  pub fn get_verification_shares(&self, voter: usize) -> Option<BigInt> {
    if voter >= self.num_voters {
      return None;
    }

    let mut sum = BigInt::from(0);
    for col in 0..self.num_voters {
      sum += &self[(voter, col)];
    }

    Some(sum % &self.modulus)
  }

  // Quadrants Filled:
  //   [X ]
  //   [ x]
  //
  // Areas outside are filled with 0's so the sum is unchaged
  fn fill_collector_one(&mut self) {
    let midpoint = self.num_voters / 2;

    // Left half
    for row in 0..midpoint {
      for col in 0..midpoint {
        if row != col {
          self[(row, col)] = BigInt::sample_below(&self.modulus);
        }
      }
    }

    // Right half
    for row in midpoint..self.num_voters {
      for col in midpoint..self.num_voters {
        if row != col {
          self[(row, col)] = BigInt::sample_below(&self.modulus);
        }
      }
    }
  }

  // Quadrants Filled:
  //   [ X]
  //   [X ]
  //
  // Areas outside are filled with 0's so the sum is unchaged
  fn fill_collector_two(&mut self) {
    let midpoint = self.num_voters / 2;

    // Left half
    for row in midpoint..self.num_voters {
      for col in 0..midpoint {
        self[(row, col)] = BigInt::sample_below(&self.modulus);
      }
    }

    // Right half
    for row in 0..midpoint {
      for col in midpoint..self.num_voters {
        self[(row, col)] = BigInt::sample_below(&self.modulus);
      }
    }
  }
}

impl Index<(usize, usize)> for SharesMatrix {
  type Output = BigInt;

  fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
    &self.matrix[row * self.num_voters + col]
  }
}

impl IndexMut<(usize, usize)> for SharesMatrix {
  fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
    &mut self.matrix[row * self.num_voters + col]
  }
}

impl fmt::Display for SharesMatrix {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let num_digits = self.matrix.iter().max().map(|x| x.to_str_radix(10).len()).unwrap_or(0);

    writeln!(
      f,
      "SharesMatrix {{ num_voters: {}, modulus: {}, matrix: ",
      self.num_voters, self.modulus
    )?;

    for row in 0..self.num_voters {
      write!(f, "  |")?;
      for col in 0..self.num_voters {
        if col > 0 {
          write!(f, ", ")?;
        }
        write!(f, "{:>width$}", self[(row, col)].to_string(), width = num_digits)?;
      }
      writeln!(f, "|")?;
    }
    writeln!(f, "}}")
  }
}
