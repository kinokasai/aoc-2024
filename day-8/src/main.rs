use std::{collections::HashMap, collections::HashSet, fs};
use itertools::Itertools;

#[derive(Debug)]
struct Parsed {
  len: usize,
  str: Vec<char>
}

fn parse_input(filename: &str) -> Parsed {
  let str = fs::read_to_string(filename).unwrap();    
  let v : Vec<Vec<char>> = str.split('\n').filter(|s| s.len() > 0)
    .map(|s| s.as_bytes().into_iter().map(|u8| *u8 as char).collect())
    .collect();
  let len = v[0].len();
  let str = v.concat();
  Parsed {len, str}
}

type Coords = (i32, i32);

impl Parsed {
  fn to_2d(&self, i : usize) -> Coords {
    let x = i % self.len;
    let y = i / self.len;
    (x as i32,y as i32)
  }
  fn is_oob(&self, (x,y) : Coords) -> bool {
    let len = self.len as i32;
    let strlen = self.str.len() as i32;
    x < 0 || y < 0 || x >= len || x + y *  len > strlen
  }

  fn get(&self, (x,y) : Coords) -> Option<char> {
    if self.is_oob((x, y)) {
      return None
    }
    let x = x as usize;
    let y = y as usize;
    let pos = x + y * self.len;
    match self.str.get(pos) {
      None => None,
      Some(v) => Some(*v)
    }
  }

  fn set(&mut self, (x,y): Coords, val : char) -> bool {
    if self.is_oob((x, y)) {
      return false
    }
    let x = x as usize;
    let y = y as usize;
    let pos = x + y * self.len;
    self.str[pos] = val;
    true
  }
}

#[derive(Debug)]
struct Antenna {
  pos: Coords,
  freq: char
}

fn ignore<T>(a: T) {
  let _ = a;
  ()
}

fn main() {
  let parsed = parse_input("./src/small_input");
  let mut antenna_map = HashMap::new();
  let antennas = parsed.str.iter().enumerate().map(|(i, c)| {
    match c {
      '.' => None,
      c => {
        let pos = parsed.to_2d(i);
        match antenna_map.get_mut(c) {
          None => ignore(antenna_map.insert(*c, vec![pos])),
          Some(vec) => vec.push(pos)
        };
        Some(Antenna { pos, freq : *c })
      }
    }
  }).filter(Option::is_some)
  .map(|c| c.unwrap())
  .collect::<Vec<Antenna>>();
  // let t = antennas.iter().tuple_combinations().collect::<Vec<(&Antenna, &Antenna)>>();
  let c = antenna_map.into_values().map(|pos| pos.into_iter().tuple_combinations().collect::<Vec<(Coords, Coords)>>())
    .collect::<Vec<Vec<(Coords, Coords)>>>();
  let mut antinodes = HashSet::new();
  for antenna_type in c.iter() {
    for ((lx, ly), (rx, ry)) in antenna_type.into_iter() {
      antinodes.insert((*lx, *ly));
      antinodes.insert((*rx, *ry));
      let diff_x = lx - rx;
      let diff_y = ly - ry;
      let mut x = *lx + diff_x;
      let mut y = *ly + diff_y;
      while !parsed.is_oob((x, y)) {
        antinodes.insert((x, y));
        x += diff_x;
        y += diff_y;
      }
      let mut x = *rx - diff_x;
      let mut y = *ry - diff_y;
      while !parsed.is_oob((x, y)) {
        antinodes.insert((x,y));
        x -= diff_x;
        y -= diff_y;
      }
    }
  }
  println!("{:?}", antinodes);
  let mut count = 0;
  for (i, c) in parsed.str.iter().enumerate() {
    let pos = parsed.to_2d(i);
    match antinodes.get(&pos) {
      None => print!("{}", c),
      Some(_) => {print!("#");
        count += 1}
    }
    if i % parsed.len == 0 && i != 0 {
      print!("\n");
    }
  }
  print!("\n");
  println!("{:?}", count);
  let count = antinodes.iter().count();
  println!("{:?}", count);
  // println!("{:?}", t);
  // println!("{:?}", c);
}
