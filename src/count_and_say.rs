
pub mod solution {

  fn count_and_say_alg( s: String ) -> String{
    let mut cur_char : char = '-';
    let mut cur_count : i32 = 0;
    let mut result : String = String::from("");
    for c in s.chars() {
      if c == cur_char {
        cur_count = cur_count+1;
      } else {
        if cur_char != '-' {
          result = format!("{}{}{}", result, cur_count, cur_char);
        }
        cur_char = c;
        cur_count = 1;
      }
    }
		result = format!("{}{}{}", result, cur_count, cur_char);
    return result;
  }

  pub fn count_and_say(n: i32) -> String {
    let mut buf = String::from("1");
    for _i in 1..n {
        buf = count_and_say_alg(buf);
    }
    buf 
  }
}

#[cfg(test)]
mod test {
  use crate::count_and_say::solution;
  #[test]
  pub fn test_count_and_say(){
    assert!( "1211".to_string() == solution::count_and_say(4) );
    assert!( "111221".to_string() == solution::count_and_say(5) );
  }
}
