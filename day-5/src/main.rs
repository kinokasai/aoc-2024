use std::collections::HashMap;
use std::fs;
use petgraph::algo::toposort;
use petgraph::graph::DiGraph;
// use petgraph::dot::{Dot, Config};
use petgraph::visit::Dfs;

fn read_input(filename: &str) -> String {
  let s = fs::read_to_string(filename);
  let input = match s {
    Ok(string) => string,
    Err(error) => panic!("Couldn't open input: {:?}", error),
  };
  input
}

fn get_graph_input(filename: &str) -> Vec<(u32, u32)> {
  let input = read_input(filename);
  let parsed: Vec<(u32, u32)> = input.split("\n")
    .filter(|s| s.len() > 0)
    .map(|s| s.split('|')
      .map(|s| s.parse::<u32>().unwrap())
      .take(2)
      .collect::<Vec<u32>>())
    .map(|v| (v[0], v[1]))
    .collect();
    parsed
}

fn get_updates_input(filename: &str) -> Vec<Vec<u32>> {
  let input = read_input(filename);
  let parsed = input.split("\n")
    .filter(|s| s.len() > 0)
    .map(|s| s.split(",")
      .map(|i| i.parse::<u32>().unwrap())
      .collect::<Vec<u32>>())
    .collect::<Vec<Vec<u32>>>();
  parsed
}

fn find_child(g : &DiGraph<u32, ()>, elt: u32, child: u32) -> bool {
  let mut dfs = Dfs::new(&g, elt.into());
  let found = false;
  while let Some (nx) = dfs.next(&g) {
    if nx == child.into() {
      return true;
    }
  }
  found
}

type Map = HashMap::<u32, Vec<(u32, u32)>>;
type Graph = DiGraph::<u32, ()>;

fn check_update(update: &Vec<u32>, map: &Map) -> Option<u32> {
  let mut g = Graph::new();
  let edges : Vec<&(u32, u32)> = update.iter()
    .map(|idx| {
      println!("{}", idx);
      map.get(idx)
    })
    .filter(Option::is_some)
    .map(Option::unwrap)
    .flatten()
    .collect();
  g.extend_with_edges(edges);
  let mut iter = update.windows(2);
  let mut ok = true;
  while let Some(pages) = iter.next_back() {
    ok = ok && find_child(&g, pages[0], pages[1]);
  };
  //part2 match
  match ok {
    true => None,
    false => {
      let a = toposort(&g, None).unwrap().into_iter()
        .map(|i| i.index() as u32)
        .filter(|i| update.contains(i))
        .collect::<Vec<u32>>();
      Some(a[a.len() /2])
    }
  }
  // part1 match
  // match ok {
  //   false => {println!("fail: {:?}", update); None},
  //   true => Some(update[update.len() / 2])
  // }
}

fn main() {

  let edges = get_graph_input("./src/full_input");
  let updates = get_updates_input("./src/full_pages_input");
  // let edges = get_graph_input("./src/graph_input");
  // let updates = get_updates_input("./src/update_input");
  let mut map = Map::new();
  for (from, to) in edges.iter() {
    match map.get_mut(from) {
      None => {let _ = map.insert(*from, vec![(*from, *to)]); ()},
      Some(t) => t.push((*from, *to))
    };
  };
  println!("{:?}", map);
  let a = updates.iter()
    .map(|u| check_update(u, &map))
    .filter(Option::is_some)
    .map(|c| c.unwrap())
    .fold(0, |acc, e| acc + e);
  println!("{}", a);
  // let dot = Dot::with_config(&g, &[Config::EdgeNoLabel, Config::NodeIndexLabel]);
  // println!("{:?}", dot);
}
