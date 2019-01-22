
pub mod solution {
  pub fn reverse(x: i32) -> i32 {
    let mut number : i32 = x;
    let mut result : i32 = 0;
    while number > 0 {
      result = (result * 10) + (number % 10);
      number = number / 10
    }
    result
  }
}
#[cfg(test)]
mod test {
  use crate::reverse_int::solution;
	#[test]
	fn test_reverse_int(){
		assert!(21 == solution::reverse(12));
		assert!(34567 == solution::reverse(76543));
		assert!(212121 == solution::reverse(121212));
		assert!(12345678 == solution::reverse(87654321));
	}
}
