use std::{collections::HashSet, fs, hash::Hash};

fn parse_map(filename: &str) -> Map {
  let s = fs::read_to_string(filename).unwrap();
  let s = s.split("\n").filter(|s| s.len() > 0)
    .collect::<Vec<&str>>();
  let width = s[0].len() as isize;
  let height = s.len() as isize;
  let mut tiles = s.into_iter()
    .map(|s| s.chars().map(|c| match c {
      '#' => Tile::Wall,
      'O' => Tile::Crate(Crate { pos: 0, side: CrateSide::Left}),
      '@' => Tile::Robot,
      '.' => Tile::Floor,
      c => panic!("We don't know this tile: {:?}", c)
    }).collect::<Vec<Tile>>())
    .flatten()
    .collect::<Vec<Tile>>();
  for i in 0..tiles.len() {
    match tiles[i] {
      Tile::Crate(c) =>
        tiles[i] = Tile::Crate(Crate { pos:i as isize, side:c.side}),
      _ => ()
    }
  };
  // part2 expand
  let tiles = tiles.into_iter()
    .map(|t| match t {
      Tile::Crate(c) => {
        let l = Crate { pos: c.pos * 2, side: CrateSide::Left };
        let r = Crate { pos: c.pos * 2+ 1, side: CrateSide::Right };
        [Tile::Crate(l), Tile::Crate(r)]
      },
      Tile::Wall => [Tile::Wall, Tile::Wall],
      Tile::Floor => [Tile::Floor, Tile::Floor],
      Tile::Robot => [Tile::Robot, Tile::Floor]
      })
    .flatten()
    .collect::<Vec<Tile>>();
  let width = width * 2;
  Map { width, height, tiles }
}

fn parse_cmds(filename: &str) -> Vec<char> {
  fs::read_to_string(filename).unwrap()
    .split("\n")
    .filter(|s| s.len() > 0)
    .map(|s| s.chars())
    .flatten()
    .collect::<Vec<char>>()
}
#[derive(Debug, Clone,Copy)]
enum CrateSide {
  Left,
  Right
}
#[derive(Debug, Clone, Copy)]
struct Crate {
  pos: isize,
  side: CrateSide
}

#[derive(Debug, Clone)]
struct CrateTrslt {
  tile: Tile,
  from: isize,
  dest: isize
}

impl Crate {
  fn move_to(&self, map: &mut Map, ofst: isize) -> bool {
    fn move_inner(crt: Crate, map: &mut Map, ofst: isize, potential_moves: &mut Vec<Option<CrateTrslt>>, sib_flag: bool,
      regs: &mut HashSet<isize>) {
      if !sib_flag && ofst.abs() > 1{
        let sib_ofst = match crt.side { CrateSide::Left => 1, _ => -1 };
        let sib_idx = sib_ofst + crt.pos;
        let crt = match map.get_unsafe(sib_idx) {
          Tile::Crate(crt) => crt,
          tile => panic!("Sibling is not a crate!: {:?}", tile),
        };
        move_inner(crt, map, ofst, potential_moves, true, regs);
      }
      let dest = crt.pos + ofst;
      let tile = map.get_unsafe(dest);
      let new_tile = Tile::Crate(Crate { pos: dest, side: crt.side});
      let trslt = CrateTrslt { tile: new_tile, from: crt.pos, dest };
      let push = |moves : &mut Vec<Option<CrateTrslt>>, regs : &mut HashSet<isize>, trslt: CrateTrslt| {
        let hash = crt.pos + 100000 + ofst;
        if regs.get(&hash).is_none() {
          regs.insert(hash);
          moves.push(Some(trslt));
        }
      };
      match tile {
        Tile::Robot => panic!(),
        Tile::Floor => push(potential_moves, regs, trslt),
        Tile::Wall => potential_moves.push(None),
        Tile::Crate(c) => {
          move_inner(c, map, ofst, potential_moves, false, regs);
          push(potential_moves, regs, trslt);
        }
      }
    }
    let mut potential_moves : Vec<Option<CrateTrslt>>= vec![];
    let mut regs = HashSet::<isize>::new();
    move_inner(*self, map, ofst, &mut potential_moves, false, &mut regs);
    match potential_moves.iter().fold(true, |acc, e| e.is_some() && acc) {
      false => false,
      true => {
        println!("{:?}", potential_moves);
        potential_moves.into_iter().map(Option::unwrap).for_each(|trnslt| map.overwrite(trnslt.tile, trnslt.from, trnslt.dest));
        true
      }
    }
  }
}

#[derive(Debug, Clone, Copy)]
enum Tile {
  Wall,
  Floor,
  Crate(Crate),
  Robot
}

impl Tile {
  fn print(&self) {
    let c = match self {
      Tile::Wall => '#',
      Tile::Floor => '.',
      Tile::Crate(Crate {side: CrateSide::Left, ..}) => '[',
      Tile::Crate(Crate {side: CrateSide::Right, ..}) => ']',
      _ => '_'
    };
    print!("{}", c)
  }
}

#[derive(Debug)]
struct Map {
  width: isize,
  height: isize,
  tiles: Vec<Tile>
}

impl Map {
  fn get_unsafe(&self, i: isize) -> Tile {
    self.tiles[i as usize]
  }

  fn overwrite(&mut self, tile: Tile, from: isize, i: isize) {
    self.tiles[i as usize] = tile;
    self.tiles[from as usize] = Tile::Floor
  }

  fn print(&self, rob_pos: isize) {
    for y in 0..self.height {
      for x in 0..self.width {
        let i = (x + y * self.width) as usize;
        if i == rob_pos as usize {
          print!("@");
          continue
        }
        self.tiles[i].print();
      }
      print!("\n");
    }
  }

  fn gps(&self) -> isize {
    self.tiles.iter()
      .filter_map(|t| match t { Tile::Crate(Crate { pos: i, side : CrateSide::Left}) => Some(i), _ => None})
      .map(|pos| (pos / self.width) * 100 + pos % self.width )
      .sum()
  }
}

#[derive(Debug)]
struct Robot {
  pos: isize
}

impl Robot {
  fn move_(&mut self, map: &mut Map, ofst: isize) {
    match map.get_unsafe(self.pos + ofst) {
      Tile::Wall => (),
      Tile::Floor => self.pos += ofst,
      Tile::Crate(c) => {
        match c.move_to(map, ofst) {
          true => self.pos += ofst,
          false => ()
        }
      }
      _ => ()
    }
  }
}

fn run(rob: &mut Robot, cmds: Vec<char>, map: &mut Map) {
  for cmd in cmds {
    let ofst = match cmd {
      '>' => 1,
      '<' => -1,
      '^' => -map.width,
      'v' => map.width,
      _ => panic!("unknown cmd: {:?}", cmd)
    };
    println!("Moving {:?}", cmd);
    rob.move_(map, ofst);
    // map.print(rob.pos);
  }
}

fn main() {
  let mut robot = Robot { pos: 0};
  let mut map = parse_map("./src/full_input");
  let cmds = parse_cmds("./src/full_cmds");
  // let mut map = parse_map("./src/small_map");
  // let cmds = parse_cmds("./src/small_commands");
  map.tiles = map.tiles.iter().enumerate().map(|(i, t)| match t{
    Tile::Robot => {robot.pos = i as isize; Tile::Floor},
    _ => *t
  })
  .collect::<Vec<Tile>>();
    map.print(robot.pos);
    run(&mut robot, cmds, &mut map);
    println!("gps: {}", map.gps());
}
