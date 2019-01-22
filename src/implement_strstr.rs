pub mod solution {

  pub fn str_str( haystack: String, needle: String ) -> i32 {
    let nlen = needle.len();
    for (hidx, _hval) in haystack.chars().enumerate() {
      if haystack[hidx..(hidx+nlen)] == needle {
        return hidx as i32;
      }
    }
    0 
  }
}

#[cfg(test)]
mod test {
  use crate::implement_strstr::solution;
  #[test]
  pub fn test_str_str() {
    assert!(3 == solution::str_str(String::from("foobar"), String::from("bar")));
    assert!(10 == solution::str_str(String::from("the quick brown fox"), String::from("brown")));
  }
}
