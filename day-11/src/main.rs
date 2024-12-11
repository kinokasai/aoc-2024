#![feature(linked_list_cursors)]

use std::collections::HashMap;

type Map = HashMap<u64, u64>;

fn blink(nums: &Map) -> Map {
  let mut next_stones = Map::new();
  for stone in nums.keys() {
    let count = nums.get(&stone).unwrap();
    if *stone == 0 {
      next_stones.insert(1, *count + *next_stones.get(&1).unwrap_or(&0));
    } else {
      let digits = u64::ilog10(*stone) + 1;
      if digits % 2 == 0 {
        let op = 10u64.pow(digits / 2);
        let l = stone / op;
        let r = stone % op;
        next_stones.insert(l, *next_stones.get(&l).unwrap_or(&0) + *count);
        next_stones.insert(r, *next_stones.get(&r).unwrap_or(&0) + *count);
      }
      else {
        let new = stone * 2024;
        next_stones.insert(new, *next_stones.get(&new).unwrap_or(&0) + *count); 
      }
    }
  };
  next_stones
}

fn main() {
  let stonevec : Vec<u64> = vec![125, 17];
  let mut stones = Map::new();
  for stone in stonevec.into_iter() {
    stones.insert(stone, 1);
  }
  for i in 0..75 {
    stones = blink(&stones);
    println!("{} -> {}", i + 1, stones.values().sum::<u64>());
  }
}
