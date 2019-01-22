
pub mod solution {
  pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32{
    nums.retain(|n| *n != val);
    nums.len() as i32 
  }
}

#[cfg(test)]
mod test {
  use crate::remove_element::solution;
  #[test]
  pub fn test_remove_element(){
    assert!(3 == solution::remove_element(&mut vec![1,2,3], 9) );
    assert!(3 == solution::remove_element(&mut vec![1,2,3,9], 9) );
    assert!(3 == solution::remove_element(&mut vec![1,2,3,9,9,9,9], 9) );
  }
}
