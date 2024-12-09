type Space = Option<usize>;

#[derive(Debug)]
struct FileIndex {
  idx: usize,
  len: usize,
  id: Space
}

type FileMap = Vec<FileIndex>;

#[derive(Debug)]
struct Fsys {
  mem: Vec<Space>,
  space_map: FileMap,
  file_map: FileMap
}

impl Fsys {
  fn print(&self) {
    for space in self.mem.iter() {
      let c = match space {
        None => ".",
        Some(id) => &id.to_string()
      };
      print!("{}", c);
    }
    print!("\n");
  }

  fn compress(&mut self) {
    let mut i = 0;
    let mut j = self.mem.len() - 1;
    while self.mem[i] != None { i += 1};
    while i != j {
      match (self.mem[i], self.mem[j]) {
        (Some(_), _) => {i += 1; continue}
        (_, None) => {j -= 1; continue}
        (None, Some(_)) => {
          self.mem[i] = self.mem[j];
          self.mem[j] = None;
          i += 1;
          j -= 1;
        }
      }
    }
  }

  fn compress_block(&mut self) {
    let mut j = self.file_map.len() - 1;
    // let space_map_len = self.space_map.len();
    // let mut space_index = &mut self.space_map[i];
    // let mut file_index = &self.file_map[j];
    while j > 0 {
      let mut i = 0;
      let space_map_len = self.space_map.len();
      let mut space_index = &mut self.space_map[i];
      let mut file_index = &self.file_map[j];
      //find a space, then copy
      while i < space_map_len && space_index.idx < file_index.idx {
        if space_index.len >= file_index.len {
          //copy the file && zero it out
          for i in 0..file_index.len {
            self.mem[i + space_index.idx] = file_index.id;
            self.mem[i + file_index.idx] = space_index.id;
          }
          space_index.len = space_index.len - file_index.len;
          space_index.idx += file_index.len;
          break;
        }
        i += 1;
        space_index = &mut self.space_map[i];
      }
      j -= 1;
      println!("{:?}", file_index)
    }
  }

  fn checksum(&self) -> usize {
    self.mem.iter()
      .enumerate()
      .filter(|(_, mem)| mem.is_some())
      .map(|(i, mem)| i * mem.expect("This should have been filtered"))
      .fold(0, |acc, e| acc + e)
  }
}

fn expand(disk_map: &str) -> Fsys {
  let mut space_map : FileMap = vec![];
  let mut file_map : FileMap = vec![];
  let parsed = disk_map.bytes().into_iter()
    .map(|d| char::to_digit(d as char, 10).unwrap() as usize)
    .enumerate()
    .map(|(i, d)| {let space = if i % 2 == 0 { Some(i/2)} else { None }; (d, space)})
    .collect::<Vec<(usize, Space)>>();
  parsed.iter()
    .fold(0, |acc, (len, id)| {
      match id {
        Some(_) => {
          file_map.push(FileIndex {idx: acc, len: *len, id: *id});
          acc + len
        }
        None => {
          space_map.push(FileIndex { idx: acc, len: *len, id: *id});
          acc + len
        }
    }
    });
  let fsys = parsed.into_iter().map(|(len, id)| vec![id; len])
    .flatten()
    .collect::<Vec<Space>>();
  Fsys { mem: fsys, space_map, file_map}
}

fn main() {
  println!("Hello, world!");
  let mut fsys = expand("12345");
  println!("{:?}", fsys.space_map);
  fsys.print();
  fsys.compress_block();
  fsys.print();
  print!("{}", fsys.checksum());
}
