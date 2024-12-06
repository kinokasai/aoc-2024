use std::collections::HashSet;
use std::f32::consts::PI;
use std::panic;
use std::fs;

#[derive(Debug)]
struct Parsed {
  len: usize,
  str: String,
}
#[derive(Debug, Clone)]
struct Board {
  len: usize,
  tiles: Vec<Tile>,
}

#[derive(Debug, Clone, Copy)]
enum Tile {
  Void,
  Obstacle,
  Player,
  OOB
}

impl Board {
  fn make(parsed: &Parsed) -> Self {
    let bytes = parsed.str.as_bytes();
    let tiles = bytes.iter().map(|c| {
      match *c as char {
        '#' => Tile::Obstacle,
        '.' => Tile::Void,
        '^' => Tile::Player,
        'x' => Tile::OOB,
        c => panic!("we don't know that char: {}", c)
      }
    })
    .collect();
    Board {len : parsed.len, tiles}
  }
  fn get(&self, (x, y) :(usize, usize)) -> Option<Tile> {
    let pos = x + y * self.len;
    match self.tiles[pos] {
      Tile::OOB => None,
      tile => Some(tile),
    }
  }

  fn set(&mut self, (x,y) : (usize, usize), tile: Tile) {
    let pos = x + y * self.len;
    self.tiles[pos] = tile;
  }
}

#[derive(Debug, Clone)]
struct Guard {
  pos: (usize, usize),
  dir: (i32, i32),
  t: f32,
}

impl Guard {
  fn move_by(&self, dir: (i32, i32)) -> (usize, usize) {
    let x = self.pos.0 as i32;
    let y = self.pos.1 as i32;
    let new_x = x + dir.0;
    let new_y = y + dir.1;
    (new_x.try_into().unwrap(), new_y.try_into().unwrap())
  }

  fn turn_right(&mut self) -> (i32, i32) {
    self.t += 1.0;
    let dir_x = f32::sin(self.t/2.0 * PI + PI).round()as i32;
    let dir_y = f32::cos(self.t/2.0 * PI).round() as i32;
    let dir = (dir_x, dir_y);
    dir
  }

  fn get_record(&self) -> GuardRecord {
    GuardRecord { pos: self.pos, dir: self.dir}
  }
}

#[derive(Hash, PartialEq, Eq)]
struct GuardRecord {
  pos: (usize, usize),
  dir: (i32, i32)
}

fn get_input() -> Parsed {
  let s = fs::read_to_string("./src/full_input");
  let input = match s {
    Ok(string) => string,
    Err(error) => panic!("couldn't open input {:?}", error),
  };
  let parsed: Vec<&str> = input.split_whitespace().collect();
  let len = parsed[0].len();
  let str = parsed.concat();
  Parsed {len, str}
}

struct SimResult {
  looped: bool,
  visited: HashSet<(usize, usize)>,
}

fn sim_guard(board: &Board, guard: &mut Guard) -> SimResult {
  let mut visited = HashSet::new();
  let mut guard_record = HashSet::new();
  while let Some(tile) = board.get(guard.pos) {
    match tile {
      Tile::Void | Tile::Player => {
        visited.insert(guard.pos);
        if !guard_record.insert(guard.get_record()) {
          return SimResult {looped: true, visited}
        }
        guard.pos = guard.move_by(guard.dir)
      }
      Tile::Obstacle => {
        guard.pos = guard.move_by((-guard.dir.0, -guard.dir.1));
        guard.dir = guard.turn_right();
        guard.pos = guard.move_by(guard.dir);
      },
      Tile::OOB => panic!("Should not get it here")
    }
  }
  SimResult {looped: false, visited}
}

fn main() {
  let parsed = get_input();
  let starting_guard = Guard { pos: (67, 91), dir: (0, -1), t : 2.0 };
  let mut board = Board::make(&parsed);
  let mut guard = starting_guard.clone();
  let sim_result = sim_guard(&board, &mut guard);
  let to_check = sim_result.visited.into_iter().collect::<Vec<(usize, usize)>>();
  let mut count = 0;
  for loc in to_check.iter() {
    guard = starting_guard.clone();
    board.set(*loc, Tile::Obstacle);
    let sim_result = sim_guard(&board, &mut guard);
    if sim_result.looped {
      count += 1;
    }
    board.set(*loc, Tile::Void);
  }
  println!("{}", count);
}
