use std::fs;

fn get_input() -> Vec<Vec<i32>> {
  let s = fs::read_to_string("./src/small_input");
  let input = match s {
    Ok(string) => string,
    Err(error) => panic!("Couldn't open input: {:?}", error),
  };
  let parsed: Vec<Vec<i32>> = input.split("\n")
    .filter(|s| s.len() > 0)
    .map(|s| s.split_whitespace()
      .map(|s| s.parse::<i32>().unwrap())
      .collect())
    .collect();
    parsed
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Polarity {
  Unk,
  Pos,
  Neg
}

fn is_same(a: &Polarity, b : &Polarity) -> bool {
  if *a == Polarity::Unk || *b == Polarity::Unk { return true }
  return *a == *b;
}

fn get_pol(a: i32, b: i32) -> Polarity {
  if a > b { Polarity::Neg } else { Polarity::Pos }
}

fn level_safeness(level: Vec<i32>) -> bool {
  let mut ranges = vec![];
  let mut level_polarity = Polarity::Unk;
  let is_safe = |polarity, level_polarity: Polarity, delta: i32| {
    // println!("{:?} {:?} {:?}", polarity, level_polarity, delta);
    is_same(&polarity, &level_polarity) && delta <= 3 && delta != 0
  };
  for (i, w) in level.windows(2).enumerate() {
    let a = w[0];
    let b = w[1];
    let delta = i32::abs(a - b);
    let polarity = get_pol(a, b);
    // println!("{:?} {:?} {:?}", polarity, level_polarity, delta);
    if !is_safe(polarity.clone(), level_polarity.clone(), delta) {
      ranges.push((level_polarity.clone(), i, i+1));
    } 
    level_polarity = polarity;
  };

  let is_safe_idx = |a: usize, b: usize, prev_pol: Polarity| {
    if b >= level.len() { return true }
    let a = level[a];
    let b = level[b];
    let pol = get_pol(a, b);
    let delta = i32::abs(a - b);
    // println!("{:?} {:?} {:?}", pol, prev_pol, delta);
    is_safe(pol, prev_pol, delta)
  };

  println!("{:?}", level);
  println!("{:?}", ranges);
  let safe = match ranges[..] {
    [] => true,
    [(pol, lidx, ridx)] => {
      if is_safe_idx(lidx, lidx + 2, pol.clone()) {
        println!("Remove {:?}", level[ridx]);
        true
      } else if lidx == 0 || is_safe_idx(lidx - 1, ridx, Polarity::Unk) {
        println!("Remove {:?}", level[lidx]);
        true
      } else { 
        println!("Couldn't link either left or right");
        false 
      }
    },
    [(a_pol, a_lidx, a_ridx), (_b_pol, b_lidx, b_ridx)] =>
      if a_ridx == b_lidx {
        println!("Remove {:?}", a_ridx);
        is_safe_idx(a_lidx, b_ridx, a_pol)
      } else {
        println!("Couldn't link the bridge");
        false
      }
    _ => false
  };
  println!("{:?}", safe);
  safe
}

fn main() {
  let levels = get_input();
  let count = levels.into_iter()
    .map(level_safeness)
    .filter(|v| *v == true)
    .count();
    println!("count: {:?}", count);
}
