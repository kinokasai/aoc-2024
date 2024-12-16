use std::{collections::HashSet, env::args, f32::consts::PI, fs};

use pathfinding::prelude::*;

struct Map(Vec<Vec<char>>);

fn parse(filename: String) -> Map {
  let file = fs::read_to_string(filename).unwrap();
  let lines = file.split("\n").filter(|s| s.len() > 0).collect::<Vec<&str>>();
  let map = lines.into_iter().map(|line| {
    line.chars().collect::<Vec<char>>()
  }).collect::<Vec<Vec<char>>>();
  Map(map)
}

impl Map {
  fn get(&self, x: isize, y:isize) -> char {
    self.0[y as usize][x as usize]
  }

  fn is_wall(&self, x: isize, y:isize) -> bool {
    match self.get(x, y) {
      '#' => true,
      _ => false
    }
  }

  fn get_start(&self) -> (isize, isize) {
    for y in 0..self.0.len() {
      let line = &self.0[y];
      for x in 0..line.len() {
        match line[x] {
          'S' => return (x as isize,y as isize),
          _ => ()
        }
      }
    }
    panic!("Couldn't find the end!");
  }

  fn get_end(&self) -> (isize, isize) {
    for y in 0..self.0.len() {
      let line = &self.0[y];
      for x in 0..line.len() {
        match line[x] {
          'E' => return (x as isize,y as isize),
          _ => ()
        }
      }
    }
    panic!("Couldn't find the end!");
  }

}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Pos {
  x: isize,
  y: isize,
  dir_x: isize,
  dir_y: isize,
  t: i32
}

impl Pos {
  fn distance(&self, other: &Pos) -> usize {
    other.x.abs_diff(self.x) + other.y.abs_diff(self.y)
  }

  fn turn_by(&self, amount: f32) -> (isize, isize) {
    let t = self.t as f32 + amount;
    let dir_x = f32::sin(t/2.0 * PI + PI).round()as isize;
    let dir_y = f32::cos(t/2.0 * PI).round() as isize;
    let dir = (dir_x, dir_y);
    dir
  }


  fn successors(&self, map: &Map) -> Vec<(Pos, usize)> {
    //This is where you check for walls
    let &Pos{x,y,dir_x,dir_y, t} = self;
    let straight = (Pos { x: x + dir_x, y: y + dir_y, dir_x, dir_y, t}, 1);
    let (dir_x, dir_y) = self.turn_by(1.0);
    let turn_right = (Pos {x, y, dir_x, dir_y, t: self.t + 1}, 1000);
    let (dir_x, dir_y) = self.turn_by(-1.0);
    let turn_left = (Pos {x, y, dir_x, dir_y, t: self.t - 1}, 1000);
    if map.is_wall(straight.0.x, straight.0.y) {
      return vec![turn_left, turn_right]
    }
    vec![straight, turn_left, turn_right]
  }
}

fn main() {
  let map = parse(args().nth(1).unwrap());
  let (x,y) = map.get_start();
  let start = Pos { x, y, dir_x: 1, dir_y: 0, t:1 };
  let (x,y) = map.get_end();
  let goal = Pos { x, y, dir_x: 0, dir_y: 0, t:0 };
  let result = astar_bag_collect(&start, |p| p.successors(&map),
                    |p| p.distance(&goal), |p| p.x == goal.x && p.y == goal.y);
  let vecs = result.unwrap().0;
  let mut set = HashSet::<(isize, isize)>::new();
  vecs.into_iter().flatten().map(|Pos{x,y,..}| (x, y))
    .for_each(|pair| {let _ = set.insert(pair); ()});
  println!("{:?}", set.len());
}
