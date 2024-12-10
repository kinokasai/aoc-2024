use std::{collections::HashSet, fs};
use petgraph::{algo::all_simple_paths, dot::{Config, Dot}, graph::DiGraph, visit::{Bfs, Dfs}};

#[derive(Debug)]
struct Parsed {
  array: Vec<i32>,
  len: isize
}

impl Parsed {
  fn get (&self, i: isize) -> Option<i32> {
    let len = self.array.len() as isize;
    if i < 0 || i >= len { None } else { Some(self.array[i as usize]) }
  }

  fn print(&self) {
    for (i, d) in self.array.iter().enumerate() {
      if i > 0 && i % self.len as usize == 0 {
        print!("\n");
      }
      print!("{}", d);
    }
    print!("\n");
  }

}

fn get_map(filename: &str) -> Parsed {
  let s = fs::read_to_string(filename).unwrap();
  let v = s.split('\n')
    .filter(|s| s.len() > 0)
    .map(|s| {s.split("")
      .filter(|s| s.len() > 0)
        .map(|s| {s.parse::<i32>().unwrap()})
        .collect::<Vec<i32>>()})
    .collect::<Vec<Vec<i32>>>();
    let len = v[0].len() as isize;
    let array = v.into_iter().flatten().collect::<Vec<i32>>();
    Parsed { len, array }
}

fn get_edges(parsed: &Parsed) -> Vec<(usize, usize)> {
  let mut i : isize = 0;
  let mut edges = vec![];
  while i < parsed.array.len() as isize {
    let value = parsed.get(i).unwrap();
    let offsets = if i % parsed.len == 0 {
      vec![i + 1, i - parsed.len, i + parsed.len]
    } else if i % parsed.len == parsed.len - 1 {
      vec![i - 1, i - parsed.len, i + parsed.len]
    } else {
    vec![i - 1, i + 1, i - parsed.len, i + parsed.len]
    };
    let neighbors = offsets.into_iter().map(|ofst| (ofst, parsed.get(ofst)))
      .map(|(ofst, height)| match height { None => None, Some(h) => Some((ofst, h)) })
      .filter_map(|s| s)
      .filter(|(_, h)| *h == value + 1)
      .map(|(idx, _)| (i as usize, idx as usize))
      .collect::<Vec<(usize, usize)>>();
      // println!("{} | {:?}", i, neighbors);
      edges.push(neighbors);
      i += 1;
  };
  edges.into_iter().flatten().collect::<Vec<(usize, usize)>>()
}

fn get_trailheads(parsed: &Parsed) -> Vec<usize> {
  parsed.array.iter()
    .enumerate()
    .filter(|(_, h)| **h == 0)
    .map(|(i, _)| i)
    .collect::<Vec<usize>>()
}

fn get_trailends(parsed: &Parsed) -> Vec<usize> {
  parsed.array.iter()
    .enumerate()
    .filter(|(_, h)| **h == 9)
    .map(|(i, _)| i)
    .collect::<Vec<usize>>()
}

fn trail(g : &DiGraph<(), usize, usize>, trailheads: &Vec<usize>, trailends: &Vec<usize>) {
  let mut count = 0;
  for head in trailheads.iter() {
    let mut reachable_ends = vec![];
    let mut dfs = Dfs::new(&g, (*head).into());
    while let Some(nx) = dfs.next(&g) {
      let nx = nx.index();
      if trailends.contains(&nx) {
        reachable_ends.push(nx);
      }
    }
    for end in reachable_ends.iter() {
      let paths = all_simple_paths::<Vec<_>, _>(&g, (*head).into(), (*end).into(), 0, None)
        .collect::<Vec<_>>();
      count += paths.len();
    }
  }
  println!("{:?}", count);

}

fn main() {
  let filename = match std::env::args().nth(1) {
    None => "./src/full_input".to_string(),
    Some(f) => f
  };
  let parsed = get_map(&filename);
  let edges = get_edges(&parsed);
  // println!("{:?}", edges);
  let g = DiGraph::<(), usize, usize>::from_edges(&edges);
  let trailheads = get_trailheads(&parsed);
  let trailends = get_trailends(&parsed);
  trail(&g, &trailheads, &trailends);
  println!("{:?}", trailheads.len());
  // println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel, Config::NodeIndexLabel]));
}
