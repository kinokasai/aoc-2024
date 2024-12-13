use std::fs;
use regex::Regex;

use pathfinding::prelude::astar;

type unum = i64;

fn parse(filename: &str) -> Vec<Machine> {
  let s = fs::read_to_string(filename).unwrap();
  let rega = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
  let regb = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
  let regg = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
  s.split("\n").filter(|s| s.len() > 0)
    .collect::<Vec<&str>>()
    .chunks(3)
    .map(|arr| {
      let a = rega.captures(arr[0]).unwrap();
      let b = regb.captures(arr[1]).unwrap();
      let c = regg.captures(arr[2]).unwrap();
      let a_delta = (a[1].parse::<unum>().unwrap(), a[2].parse::<unum>().unwrap());
      let b_delta = (b[1].parse::<unum>().unwrap(), b[2].parse::<unum>().unwrap());
      let goal = (c[1].parse::<unum>().unwrap(), c[2].parse::<unum>().unwrap());
      let goal = (goal.0 + 10000000000000, goal.1 +10000000000000);
       Machine { a_delta, b_delta, goal }
    }).collect::<Vec<Machine>>()
}

#[derive(Clone,Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos{
  pos: (unum, unum),
  a_counter: unum,
  b_counter: unum,
}

#[derive(Debug)]
struct Machine {
  a_delta: (unum, unum),
  b_delta: (unum, unum),
  goal: (unum, unum)
}

impl Pos {
  fn make(x: unum, y: unum) -> Self {
    Pos { pos:(x,y), a_counter: 0, b_counter: 0 }
  }

  fn distance(&self, other: (unum, unum)) -> unum {
    (other.0.abs_diff(self.pos.0)
      + other.1.abs_diff(self.pos.1)) as i64
  }

  fn update_a(&self, machine: &Machine) -> Self {
    Pos { pos: (self.pos.0 + machine.a_delta.0,
                self.pos.1 + machine.a_delta.1),
          a_counter: self.a_counter + 1,
          b_counter: self.b_counter }
  }

  fn update_b(&self, machine: &Machine) -> Self {
    Pos { pos: (self.pos.0 + machine.b_delta.0,
                self.pos.1 + machine.b_delta.1),
          b_counter: self.b_counter + 1,
          a_counter: self.a_counter }
  }


  fn successors(&self, machine: &Machine) -> Vec<(Pos, unum)> {
    let a = self.update_a(&machine);
    let b = self.update_b(&machine);
    match (self.a_counter, self.b_counter) {
      (100, 100) => vec![],
      (100, _) => vec![(b, 1)],
      (_, 100) => vec![(a, 3)],
      _ => vec![(a, 3), (b, 1)]
    }
  }
}

fn legacy_part1(machines: &Vec<Machine>) -> unum {
  machines.into_iter()
    .map(|machine| {
      println!("new machine");
      let start = Pos::make(0, 0);
      let result = astar(&start,
        |p| p.successors(&machine),
        |p| p.distance(machine.goal),
        |p| p.pos == machine.goal);
      match result {
        None => 0,
        Some(tokens) => tokens.1
      }})
  .sum()
}

fn algebra(machine: &Machine) -> unum {
  println!("{:?}", machine);
  let (gx, gy) = machine.goal;
  let (ax, ay) = machine.a_delta;
  let (bx, by) = machine.b_delta;
  let b = (gy * ax - gx * ay) / (by * ax - bx * ay);
  let a = (gx - bx * b) / ax;
  let check_a = (gx - bx * b) % ax;
  let check_b = (gy * ax - gx * ay) % (by * ax - bx * ay);
  println!("a: {:?} | b: {:?}", a, b);
  // check that the result is an integer by checking the remainder
  match (check_a, check_b) {
    (0, 0) => a * 3 + b,
    _ => 0
  }
}

fn main() {
  let machines = parse("./src/full_input");
  let tok_tot:unum = machines.into_iter()
    .map(|m| algebra(&m))
    .sum();
  println!("total tokens: {}", tok_tot);
}
