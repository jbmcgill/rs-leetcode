
mod solution {
  pub fn remove_duplicates( nums: &mut Vec<i32> ) -> i32 {
    nums.dedup();
    return nums.len() as i32;
  }
}

#[cfg(test)]
mod test {
  use crate::remove_duplicates_from_sorted_array::solution; 
  #[test]
  pub fn test_remove_duplicates_from_sorted_array(){
    assert!( 3 == solution::remove_duplicates(&mut vec![1,2,3]) );
    assert!( 3 == solution::remove_duplicates(&mut vec![1,1,2,3]) );
    assert!( 3 == solution::remove_duplicates(&mut vec![1,1,1,2,3]) );
    assert!( 3 == solution::remove_duplicates(&mut vec![1,1,1,2,2,2,2,2,2,3,3,3,3,3]) );
  }
}
