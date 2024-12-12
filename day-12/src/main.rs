use std::{collections::HashSet, fs, time::Instant};

#[derive(Debug)]
struct Map {
  land: Vec<char>,
  len: isize,
}

impl Map {
  fn get(&self, i: isize) -> Option<char> {
    let len = self.land.len() as isize;
    if i < 0 || i >= len { None } else { Some(self.land[i as usize]) }
  }

  fn get_from(&self, i:isize, ofst: isize) -> Option<char> {
    if i % self.len == self.len - 1 && ofst == 1 {
      None
    } else if i % self.len == 0 && ofst == -1 {
      None
    } else {
      self.get(i + ofst)
    }
  }

  fn neighbors_idx(&self, i: isize) -> Vec<isize> {
    let mut h_neighbors = match i % self.len {
      0 => vec![i+1],
      r if r == self.len - 1 => vec![i - 1],
      _ => vec![i-1, i+1],
    };
    let v_neighbors = match i {
      i if i < self.len => vec![i+self.len],
      i if i > self.land.len() as isize - self.len => vec![i-self.len],
      _ => vec![i+self.len, i - self.len]
    };
    h_neighbors.extend(v_neighbors);
    h_neighbors
  }

  fn neighbors_idx_opt(&self, i: isize) -> Vec<Option<isize>> {
    let mut nidxs = self.neighbors_idx(i).into_iter().map(|c| Some(c))
      .collect::<Vec<Option<isize>>>();
      while nidxs.len() < 4 {
        nidxs.push(None);
      }
      nidxs
  }
}

fn get_field_starts(map: &Map) -> Vec<isize> {
  let mut field_starts = vec![];
  for (i, c) in map.land.iter().enumerate() {
    let i = i as isize;
    match (map.get_from(i, -1), map.get_from(i, -map.len)) {
      (Some(cl), Some(cu)) => if (cu != *c) && (cl != *c) {
        field_starts.push(i)
      },
      (None, Some(cp)) |
      (Some(cp), None) => if cp != *c { field_starts.push(i) },
      (None, None) => field_starts.push(i)
    }
  }
  field_starts
}

fn get_map(filename: &str) -> Map {
  let s = fs::read_to_string(filename).unwrap();
  let v = s.split("\n").filter(|s| s.len() > 0)
    .map(|s| s.chars().collect::<Vec<char>>())
    .collect::<Vec<Vec<char>>>();
    let len = v[0].len() as isize;
    let land = v.into_iter().flatten().collect::<Vec<char>>();
    Map { land, len }
}

#[derive(Debug)]
struct FieldProps {
  kind: char,
  area: usize,
  perimeter: usize,
  corners: usize,
  parcels: Vec<isize>,
  cost: usize,
  bulk_cost: usize,
}

fn get_props(map: &Map) -> Vec<FieldProps> {
  let mut field_props = vec![];
  let mut visited = HashSet::<isize>::new();
  for idx in 0..map.land.len() {
    let idx = idx as isize;
    if visited.contains(&idx) {
      continue;
    }
    let mut to_visit : Vec<isize> = vec![idx];
    let current_char = map.get(idx).unwrap();
    let mut i = 0;
    let mut perimeter = 0;
    let mut area = 0;
    while i < to_visit.len() {
      let idx = to_visit[i];
      if visited.contains(&idx) {
        i+=1;
        continue;
      }
      visited.insert(idx);
      let char = map.get(idx).unwrap();
      let neighbors = map.neighbors_idx(idx);
      let count = neighbors.into_iter().filter_map(|idx| map.get(idx))
        .filter(|c| *c == char)
        .count();
        perimeter += 4 - count;
        area += 1;
        // new neighbors to visit
        [-1, 1, map.len, -map.len].into_iter()
          .map(|ofst| (ofst, map.get_from(idx, ofst)))
            .filter_map(|(ofst, copt)| match copt {
              Some(c) if c == char => Some(ofst),
              _ => None
            })
        .for_each(|ofst| to_visit.push(idx + ofst));
        i += 1;
    };
    let cost = area * perimeter;
    to_visit.sort(); to_visit.dedup();
    let mut prop = FieldProps {area, perimeter, cost,  kind: current_char, parcels: to_visit,
    corners: 0, bulk_cost: 0};
    prop.corners = corners(&map, &prop);
    prop.bulk_cost = area * prop.corners;
    field_props.push(prop);
  }
  field_props
}

fn ignore<T>(_: T) {
}

fn corners(map: &Map, prop: &FieldProps) -> usize {
  // println!("{:?}", prop);
  let mut ids = HashSet::<isize>::new();
  let mut corners = 0;
  prop.parcels.iter().map(|i| vec![i+0, i+1, i+map.len, i+map.len + 1])
    .flatten()
    .for_each(|i| ignore(ids.insert(i)));
  // println!("{:?}", ids);
  let mut ids = ids.into_iter().collect::<Vec<isize>>();
  ids.sort();
  for idx in ids.into_iter() {
    let window = [-1-map.len, -1, -map.len, 0];
    let window = window.into_iter().map(|i| (i, map.get_from(idx, i)))
      .map(|(i, c)| match c { Some(c) if prop.parcels.contains(&(idx + i)) => Some(c), _ => None})
      .collect::<Vec<Option<char>>>();
    let hits = window.iter().filter(|c| Option::is_some(*c)).count();
    let window : [Option<char>; 4] = window.try_into().unwrap();
    let mut corner_diff = 0;
    match (hits, window) {
      (1,_) | (3, _) => corner_diff = 1,
      (4,_) => (),
      (_, [Some(_), None, None, Some(_)]) |
      (_, [None, Some(_), Some(_), None]) => 
      corner_diff = 2,
      _ => ()
    }
    corners += corner_diff;
    // println!("{} -> {:?}", idx, window);
    // println!("{} -> +{}", idx, corner_diff);
  }
  corners
}

fn main() {
  let map = get_map("./src/full_input");
  let time = Instant::now();
  // let map = get_map("./src/enclosed_map");
  let mut props = get_props(&map).into_iter().filter(|p| p.kind != '.').collect::<Vec<FieldProps>>();
  // props.iter().for_each(|c| println!("{:?}", c));
  corners(&map, &mut props[0]);
  let total : usize  = props.iter().map(|p| p.cost).sum();
  let total_bulk: usize  = props.into_iter().map(|p| p.bulk_cost).sum();
  println!("elpased: {:?}", time.elapsed());
  println!("total cost: {:?}", total);
  println!("total bulk cost: {:?}", total_bulk);
}
