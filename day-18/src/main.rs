use std::{env::args, fs, isize};

use ndarray::Array2;
use pathfinding::prelude::*;

fn parse(filename: String) -> Vec<[isize; 2]> {
  let s = fs::read_to_string(filename).unwrap();
  s.split("\n").filter(|s| s.len() > 0)
    .map(|s| s.split(',').map(|i| i.parse::<isize>().unwrap()).collect::<Vec<isize>>())
    .map(|v| [v[0], v[1]])
    .collect::<Vec<[isize; 2]>>()
}

#[derive(Debug)]
struct Map<T>(Array2<T>);

impl Map<bool> {
  fn make(size: usize) -> Self {
    let map = Array2::<bool>::from_elem([size, size], false);
    Map(map)
  }

  fn is_wall(&self, x: isize, y:isize) -> bool {
    match self.get(x, y) {
      | Some(false) => false,
      | _ => true
    }
  }
}
impl<T> Map<T> {
  fn unsafe_get(&self, x: isize, y:isize) -> &T {
    &self.0[[x as usize, y as usize]]
  }

  fn get(&self, x:isize, y:isize) -> Option<&T> {
    self.0.get([x as usize, y as usize])
  }

  fn set(&mut self, [x, y]: [isize; 2], val: T) {
    self.0[[x as usize, y as usize]] = val
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(isize, isize);

impl Pos {
  fn successors(&self, map: &Map<bool>) -> Vec<(Pos, usize)> {
    let &Pos(x,y) = self;
    vec![(x+1, y), (x-1, y), (x, y+1), (x, y-1)]
      .into_iter()
      .filter(|(x,y)| !map.is_wall(*x, *y))
      .map(|(x,y)| (Pos(x,y), 1))
      .collect::<Vec<(Pos, usize)>>()
  }
}


fn main() {
  let mut map = Map::make(71);
  let start = Pos(0, 0);
  let end = Pos(70, 70);
  let parsed = parse(args().nth(1).unwrap());
  let mut it = parsed.into_iter();
  it.by_ref().take(1024).for_each(|crds| 
    map.set(crds, true));
  let mut coords : [isize; 2] = [0, 0];
  while let Some(_) = dijkstra(&start, |p| p.successors(&map), |p| *p == end) {
    coords = it.next().unwrap();
    map.set(coords, true);
  }
  println!("end found: {:?}", coords);
}
