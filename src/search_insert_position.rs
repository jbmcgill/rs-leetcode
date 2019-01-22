pub mod solution {
  pub fn search_insert( nums: Vec<i32>, target:i32 ) -> i32 {
    let mut pos : i32 = -1;
    for (idx, val) in nums.iter().enumerate() {
      if *val == target || *val > target {
        pos = idx as i32 - 1;
      }
    }
    pos
  }
}

#[cfg(test)]
mod test {
  use crate::search_insert_position::solution;
  #[test]
  pub fn test_search_insert() {
    assert!( 2 == solution::search_insert( vec![1,3,5,6], 5) ); 
  }
}
