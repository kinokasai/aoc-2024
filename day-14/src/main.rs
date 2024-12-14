use std::{collections::HashMap, fs};

use regex::Regex;
fn parse(filename: &str) -> Vec<Robot> {
  let s = fs::read_to_string(filename).unwrap();
  let reg = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
  s.split("\n").filter(|s| s.len() > 0)
    .map(|s| {
      let c = reg.captures(s).unwrap();
			let x = c[1].parse::<isize>().unwrap();	
			let y = c[2].parse::<isize>().unwrap();
			let vx = c[3].parse::<isize>().unwrap();	
			let vy = c[4].parse::<isize>().unwrap();
			Robot { pos: (x,y), vel: (vx, vy) }
    }).collect::<Vec<Robot>>()
}


struct Map {
  height: isize,
  width: isize,
}

impl Map {
  fn print(&self, robs: &Vec<Robot>) {
    let mut robs_pos = HashMap::<(isize, isize), i32>::new();
    robs.iter()
      .for_each(|rob| {
        let prev = robs_pos.get(&rob.pos).unwrap_or_else(|| &0);
        robs_pos.insert(rob.pos, prev + 1);
      });
    for y in 0..self.height {
      for x in 0..self.width {
        match robs_pos.get(&(x, y)) {
          None => print!("."),
          Some(i) => print!("{}", i)
        }
      }
      print!("\n");
    }
  }
}

#[derive(Debug)]
struct Robot {
  pos: (isize, isize),
  vel: (isize, isize)
}

impl Robot {
  fn move_(&mut self, map: &Map) {
    let x = (self.pos.0 + self.vel.0).rem_euclid(map.width);
    let y = (self.pos.1 + self.vel.1).rem_euclid(map.height);
    self.pos = (x, y)
  }
}

fn main() {
  let map = Map { width : 101, height: 103};
  let mut robs = parse("./src/full_input");
  // map.print(&robs_pos);
  for i in 0..10000 {
    let mut counter = 0;
    for rob in robs.iter_mut() {
      rob.move_(&map);
      if rob.pos.0 > map.width / 4 && rob.pos.0 < 3 * map.width / 4
        && rob.pos.1 > map.height / 4 && rob.pos.1 < 3 * map.height / 4 {
          counter += 1
      }
    }
    if counter > 300 {
      println!("iteration {:?}: ", i);
      map.print(&robs);
    }
  }
}
