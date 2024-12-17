use std::ops::BitXor;

#[derive(Debug, PartialEq)]
enum State {
  Running,
  Stopped
}

#[derive(Debug)]
struct Proc<'a> {
  a_reg: u64,
  b_reg: u64,
  c_reg: u64,
  pointer: usize,
  program: &'a Vec<u32>,
  out: Vec<u32>,
  state: State
}

impl <'a> Proc<'a> {
  fn make(a: u64, b:u64, c: u64, program: &'a Vec<u32>) -> Self {
    Proc {
      a_reg: a,
      b_reg: b,
      c_reg: c,
      pointer: 0,
      out: vec![],
      program,
      state: State::Running
    }
  }
  fn get_combo(&self, op: u32) -> u64 {
    match op {
      0 | 1 | 2 | 3 => op.into(),
      4 => self.a_reg,
      5 => self.b_reg,
      6 => self.c_reg,
      _ => panic!("bad operand: {:?}", op)
    }
  }

  fn get_next(&self) -> u32 {
    self.program[self.pointer + 1]
  }

  fn adv(&mut self, op: u32) {
    let op = self.get_combo(op);
    let res = self.a_reg / 2_u64.pow(op.try_into().unwrap());
    self.a_reg = res;
    self.pointer += 2;
  }

  fn bxl(&mut self, op: u32) {
    let res = self.b_reg.bitxor(op as u64);
    self.b_reg = res;
    self.pointer += 2;
  }

  fn bst(&mut self, op: u32) {
    let op = self.get_combo(op);
    let res = op % 8;
    self.b_reg = res;
    self.pointer += 2;
  }

  fn jnz(&mut self, op: u32) {
    match self.a_reg {
      0 => self.pointer += 2,
      _ => self.pointer = op as usize
    }
  }

  fn bxc(&mut self, _op: u32) {
    let res = self.b_reg.bitxor(self.c_reg);
    self.b_reg = res;
    self.pointer += 2;
  }

  fn out(&mut self, op: u32) {
    let op = self.get_combo(op);
    self.out.push((op % 8).try_into().unwrap());
    self.pointer += 2;
  }

  fn bdv(&mut self, op: u32) {
    let op = self.get_combo(op);
    let res = self.a_reg / 2_u64.pow(op.try_into().unwrap());
    self.b_reg = res;
    self.pointer += 2;
  }

  fn cdv(&mut self, op: u32) {
    let op = self.get_combo(op);
    let res = self.a_reg / 2_u64.pow(op.try_into().unwrap());
    self.c_reg = res;
    self.pointer += 2;
  }

  fn apply(&mut self) {
    match self.program.get(self.pointer) {
      None => self.state = State::Stopped,
      Some(0) => self.adv(self.get_next()),
      Some(1) => self.bxl(self.get_next()),
      Some(2) => self.bst(self.get_next()),
      Some(3) => self.jnz(self.get_next()),
      Some(4) => self.bxc(self.get_next()),
      Some(5) => self.out(self.get_next()),
      Some(6) => self.bdv(self.get_next()),
      Some(7) => self.cdv(self.get_next()),
      Some(opc) => panic!("Unknown opcode: {:?}", opc)
    }
  }

  fn run(&mut self) {
    while self.state == State::Running {
      self.apply();
    }
  }
}

fn main() {
  let mut key = vec![0; 16];
  key[15] = 1;
  let instr = vec![2,4,1,3,7,5,1,5,0,3,4,3,5,5,3,0];
  let code = instr.clone();
  let mut a = key.iter().enumerate().map(|(i, k)| k * 8_u64.pow(i.try_into().unwrap())).reduce(|acc, i| acc + i).unwrap();
  let mut proc = Proc::make(a, 0, 0, &instr);
  proc.run();
  let mut i = 16;
  while i >= 1 {
    while proc.out[i - 1] != code[i - 1] {
      if i < 16 && proc.out[i] != code[i] {
        key[i-1] = 0;
        i += 1;
      }
      key[i - 1] += 1;
      a = key.iter().enumerate().map(|(i, k)| k * 8_u64.pow(i.try_into().unwrap()))
        .reduce(|acc, i| acc + i).unwrap();
        proc = Proc::make(a, 0, 0, &instr);
        proc.run();
    }
    i -= 1;
  }
  println!("Found a {:?}", a);
}
