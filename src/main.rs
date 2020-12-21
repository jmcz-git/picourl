#[allow(dead_code)]
fn shorter(s: &str) -> &str {
  s
}

fn main() {
  println!("{}", shorter("http://www.educative.io/courses/grokking-the-system-design-interview/m2ygV4E81AR"));
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        assert_eq!(
          shorter("http://www.educative.io/courses/grokking-the-system-design-interview/m2ygV4E81AR"),
          "https://www.educative.io/courses/grokking-the-system-design-interview/m2ygV4E81AR"
        );
    }
}

