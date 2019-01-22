pub mod solution {
  pub fn reverse( x: i32 ) -> i32 {
    let mut number : i32 = x;
    let mut result : i32 = 0;
    let mut last_digit : i32 = 0;
    while number > 0 {
      last_digit = number % 10;
      result = (result * 10) + last_digit;
      number = number / 10;
    } 
    result
  }
  pub fn is_palindrome(x : i32 ) -> bool {
    if x < 0 { return false; }
    let reversed : i32 = reverse(x);
    return x == reverse(reversed);
  }
}

#[cfg(test)]
mod test {
  use crate::palindrome_number::solution;
	#[test]
	fn test_reverse_int(){
		assert!(true == solution::is_palindrome(121));
		assert!(true == solution::is_palindrome(22122));
		assert!(false == solution::is_palindrome(-22122));
		assert!(false == solution::is_palindrome(100));
		assert!(true == solution::is_palindrome(1001));
	}
}

