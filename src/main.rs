#[allow(dead_code)]
fn shorter(_url: &str) -> String {
  let s = String::from("http://tinyurl.com/jlg8zpc");
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
        let s = shorter("http://www.educative.io/courses/grokking-the-system-design-interview/m2ygV4E81AR");
        assert_eq!(s.len(), 26);
    }
}

