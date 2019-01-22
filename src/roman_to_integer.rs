
pub mod solution {
  #[derive(Debug)]
  enum Numeral {
    I = 1,
    V = 5,
    X = 10,
    L = 50,
    C = 100,
    D = 500,
    M = 1000,
    None = 0,
  }
  #[derive(Debug)]
  pub struct Numerals {
    pub items: Vec<i32>,
  }
  impl Numerals {
    pub fn parse(input: &str) -> Option<Numerals> {
      let mut items = Vec::<i32>::new();
      for c in input.chars() {
        let numeral = match c {
          'I' => Numeral::I as i32,
          'V' => Numeral::V as i32,
          'X' => Numeral::X as i32,
          'L' => Numeral::L as i32,
          'C' => Numeral::C as i32,
          'D' => Numeral::D as i32,
          'M' => Numeral::M as i32, 
           _ => Numeral::None as i32,
        };
        items.push(numeral);
      }
      Some(Numerals{ items: items })
    }
  }
  pub fn roman_to_int(s: String) -> i32 {
    let numerals = Numerals::parse(&s);
    let mut result = 0;
    let mut max = 0;
    for val in numerals.unwrap().items.iter().rev() {
      result += if *val >= max { *val } else { -*val };
      if max < *val {
        max = *val;
      }
    }
    result 
  }
}

#[cfg(test)]
mod test {
  use crate::roman_to_integer::solution;
  #[test]
  pub fn test_roman_to_int() {
    assert!( solution::roman_to_int("II".to_string()) == 2 );
    assert!( solution::roman_to_int("III".to_string()) == 3 );
    assert!( solution::roman_to_int("V".to_string()) == 5 );
    assert!( solution::roman_to_int("VII".to_string()) == 7 );
    assert!( solution::roman_to_int("XII".to_string()) == 12 );
    assert!( solution::roman_to_int("IX".to_string()) == 9 );
    assert!( solution::roman_to_int("XC".to_string()) == 90 );
  }
}
