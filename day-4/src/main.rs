use std::{arch::x86_64::_XCR_XFEATURE_ENABLED_MASK, fs};

#[derive(Debug)]
struct Parsed {
  len: usize,
  str: String,
}

fn get_input() -> Parsed {
  let s = fs::read_to_string("./src/full_input");
  let input = match s {
    Ok(string) => string,
    Err(error) => panic!("couldn't open input  {:?}", error),
  };
  let parsed: Vec<&str> = input.split_whitespace().collect();
  let len = parsed[0].len();
  let str = parsed.concat();
  Parsed {len, str}
}

fn match_mas(bytes: &Vec<char>, a: usize, b:usize, c:usize) -> bool {
  match (bytes.get(a), bytes.get(b), bytes.get(c)) {
    (Some('M'), Some('A'), Some('S')) => true,
    _ => false
  }
}

fn part_1(parsed : Parsed) -> Vec<[usize; 5]>{
  let bytes :Vec<char> = parsed.str.as_bytes().into_iter().map(|c| *c as char).collect();
  let idxs : Vec<[usize; 3]> = vec![
    [1, 2, 3],
    [parsed.len, parsed.len*2, parsed.len*3],
    [parsed.len + 1, parsed.len * 2 + 2, parsed.len * 3 + 3],
    [parsed.len - 1, parsed.len * 2 - 2, parsed.len * 3 - 3]
  ];
    let mut ranges = vec![];
    let mut i = 0;
    while i < parsed.str.len() {
      println!("{}", i);
      let c = bytes[i];
      println!("{}", c);
      if c == 'X' {
        for [a, b,c] in idxs.clone().into_iter() {
          if match_mas(&bytes, i + a, i + b, i + c) {
            ranges.push([i + a, i + b, i + c, i, i]);
          }
          if i > a && i > b && i > c {
            if match_mas(&bytes, i - a, i - b, i - c) {
              ranges.push([i, i - a, i - b, i - c, i])
            }
          }
        }
      }
      i += 1;
    }
  ranges
}

fn part_2(parsed : &Parsed) -> Vec<[usize; 5]> {
  let bytes :Vec<char> = parsed.str.as_bytes().into_iter().map(|c| *c as char).collect();
  let len = parsed.len as i32;
  let mas_idx = vec![len + 1,-len - 1, len - 1, -len+ 1];
  let mut ranges = vec![];
  let mut i = 0;
  while i < parsed.str.len() {
    if bytes[i] == 'A' {
      let idxs: Vec<usize> = mas_idx.clone()
        .into_iter()
        .map(|idx| (idx + (i as i32)) as usize)
        .collect();
      let letters = idxs.clone()
          .into_iter()
          .map(|idx| bytes[idx])
          .collect::<Vec<char>>();
      if letters.clone().into_iter().fold(true, |acc, c| acc && (c == 'M' || c == 'S' ))
        && letters[0] != letters[1] && letters[2] != letters[3] {
          ranges.push([i, i + idxs[0], i + idxs[1], i + idxs[2], i + idxs[3]]);
      }
    }
    i+=1
  };
  ranges
}

fn draw_array(parsed : &Parsed, ranges: Vec<[usize; 5]>) {
  let xmas_idxs : Vec<usize> = ranges.into_iter().flatten().collect();
  for i in 0..parsed.str.len() {
    if i % parsed.len == 0 {
      print!("\n");
    }
    if xmas_idxs.contains(&i) {
      print!("{}", parsed.str.as_bytes()[i] as char);
    } else {
      print!(".")
    }
  }
}


fn main() {
  let parsed = get_input();
  let ranges = part_2(&parsed);
  let count = ranges.len();
  draw_array(&parsed, ranges);
  println!("\nCount: {}", count);
}
