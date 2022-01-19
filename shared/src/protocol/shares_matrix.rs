use curv_kzen::arithmetic::{Converter, Samplable};
use curv_kzen::BigInt;
use std::fmt;
use std::ops::{Index, IndexMut};

/// Matrix used by the collectors for (n,n) Secret Sharing
pub struct SharesMatrix {
  num_voters: usize,
  modulus: BigInt,
  matrix: Vec<BigInt>,
}

impl SharesMatrix {
  pub fn new(collector: usize, num_collectors: usize, num_voters: usize, modulus: BigInt) -> Self {
    let mut matrix = SharesMatrix {
      num_voters,
      modulus,
      matrix: vec![BigInt::from(0); num_voters * num_voters],
    };

    // Fill in the matrix quadrants
    matrix.fill_collector(collector, num_collectors);
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

  // Fill the quadrants associated with each collector
  //
  // [ 1 ][ 2 ][ 3 ] ... [ n ]
  // [ n ][ 1 ][ 2 ] ... [n-1]
  // [n-1][ n ][ 1 ] ... [n-2]
  //   |    |    |    \    |
  // [ 2 ][ 3 ][ 4 ] ... [ 1 ]
  //
  // Areas outside are filled with 0's so the sum is unchaged
  fn fill_collector(&mut self, collector: usize, num_collectors: usize) {
    // Fill the diagonal "cells", wrapping around
    //  Each cell is approx. (num_voters / num_collectors)^2 entries in size
    //  For a special case, the last row and column may be different
    for cell_index in 0..num_collectors {
      let row_cell_index = cell_index;
      let col_cell_index = (collector + cell_index) % num_collectors;

      // Top left of the cell
      let row_start = (row_cell_index * self.num_voters) / num_collectors;
      let col_start = (col_cell_index * self.num_voters) / num_collectors;

      // Last row may be a different size to fill remaining rows
      let row_end = if row_cell_index == (num_collectors - 1) {
        self.num_voters
      } else {
        ((row_cell_index + 1) * self.num_voters) / num_collectors
      };

      // Last column may be a different size to fill remaining columns
      let col_end = if col_cell_index == (num_collectors - 1) {
        self.num_voters
      } else {
        ((col_cell_index + 1) * self.num_voters) / num_collectors
      };

      // Fill in all rows and columns in the individual cell
      for row in row_start..row_end {
        for col in col_start..col_end {
          // Don't fill the main diagonal
          if row != col {
            self[(row, col)] = BigInt::sample_below(&self.modulus);
          }
        }
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
