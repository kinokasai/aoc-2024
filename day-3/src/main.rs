use regex::Regex;

enum Instr {
  Do,
  Dont,
  Mul(i32, i32)
}

#[derive(Clone, Copy)]
enum State {
  Active,
  Inactive
}

fn main() {
  let hay = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
  let rgx = Regex::new(r"(?<do>do\(\))|(?<dont>don't\(\))|mul\((?<lhs>\d+),(?<rhs>\d+)\)").unwrap();
  let result = rgx.captures_iter(hay).map(|caps| {
    match (caps.name("do"), caps.name("dont"), caps.name("lhs")) {
      (Some(_), _, _) => Instr::Do,
      (_, Some(_), _) => Instr::Dont,
      _ => {
    let lhs = caps.name("lhs").unwrap().as_str();
    let rhs = caps.name("rhs").unwrap().as_str();
    println!("{:?} x {:?}", lhs, rhs);
    let lhs = lhs.parse::<i32>().unwrap();
    let rhs = rhs.parse::<i32>().unwrap();
    Instr::Mul(lhs, rhs)
      }
    }
  }).fold((State::Active, 0), |(state, acc), instr| {
    match (state, instr) {
      (State::Inactive, Instr::Do) => (State::Active, acc),
      (State::Active, Instr::Dont) => (State::Inactive, acc),
      (State::Active, Instr::Mul(lhs, rhs)) => (state, acc + (lhs * rhs)),
      _ => (state, acc)
    }
  });
  println!("{:?}", result.1); 
}
