use std::collections::HashSet;
use std::f32::consts::PI;
use std::panic;
use std::fs;

#[derive(Debug)]
struct Parsed {
  len: usize,
  str: String,
}

#[derive(Debug)]
enum Tile {
  Void,
  Obstacle,
  Player,
}

impl Parsed {
  fn get(&self, (x, y) :(usize, usize)) -> Option<Tile> {
    let pos = x + y * self.len;
    let bytes = self.str.as_bytes();
    match bytes[pos] as char {
      '#' => Some(Tile::Obstacle),
      '.' => Some(Tile::Void),
      '^' => Some(Tile::Player),
      'x' => None,
      c => panic!("we don't know that char: {}", c)
    }
  }
}

#[derive(Debug)]
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

fn main() {
  let board = get_input();
  let mut visited = HashSet::new();
  let mut guard = Guard { pos: (67, 91), dir: (0, -1), t : 2.0 };
  println!("{:?}", board.get((67, 91)));
  while let Some(tile) = board.get(guard.pos) {
    match tile {
      Tile::Void | Tile::Player => {
        visited.insert(guard.pos);
        guard.pos = guard.move_by(guard.dir)
      }
      Tile::Obstacle => {
        // println!("Obstacle at {:?}!", guard.pos);
        guard.pos = guard.move_by((-guard.dir.0, -guard.dir.1));
        guard.dir = guard.turn_right();
        guard.pos = guard.move_by(guard.dir);
        // println!("Moving towards {:?}!", guard.pos);
      }
    }
  }
  println!("{:?}", visited.len());
}
