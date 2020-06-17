use std::error::Error;
use std::fs;
use std::process;

const PART_1_RESULT: usize = 3409710;
const PART_2_RESULT: usize = 19690720;

fn main() {
    match run("input.txt", PART_2_RESULT) {
        Ok(t) => println!("Memory Cell 0: {} with {} and {}", t.0, t.1, t.2),
        Err(e) => {
            println!("Application error: {}", e);
            process::exit(1)
        }
    }
}

struct Memory {
    data: Vec<usize>,
}

impl Memory {
    pub fn new(init: &[usize]) -> Memory {
        let mut nv = Memory { data: Vec::new() };
        nv.data.extend(init);
        nv
    }

    pub fn push(&mut self, n: usize) {
        self.data.push(n);
    }

    pub fn get(&self, idx: usize) -> usize {
        if self.data.len() > idx {
            return self.data[idx];
        }
        // out of bounds, we treat this as empty = 0
        0
    }

    pub fn put(&mut self, idx: usize, value: usize) {
        if self.data.len() > idx {
            self.data[idx] = value;
        } else {
            // enhance data vector to get value in
            for _ in self.data.len()..idx {
                self.data.push(0)
            }
            self.data.push(value)
        }
    }

    pub fn dump(&self) {
        let mut pos: usize = 0;
        loop {
            for i in 0..4 {
                print!("{:>7}|", &self.get(pos + i))
            }
            println!();
            pos += 4;
            if pos > self.data.len() {
                break;
            }
        }
    }
}

fn run(filename: &str, target: usize) -> Result<(usize, usize, usize), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let mut result: usize = 0;
    let mut start1: usize = 12;
    let mut start2: usize = 1;

    for start1 in 0..100 {
        for start2 in 0..100 {
            let mut mem = Memory::new(&[]);
            let mut m: usize;

            for code in contents.split(',') {
                m = code.trim().parse().unwrap();
                mem.push(m);
            }

            result = exec_with_start(mem, start1, start2);
            // println!("{:>8} {:>8} {:>8}", result, start1, start2);
            if result == target {
                return Ok((result, start1, start2));
            }
        }
    }
    // if we end up here, we did not find it
    Ok((0, 0, 0))
}

fn exec_with_start(mut mem: Memory, start1: usize, start2: usize) -> usize {
    // set the starting values
    mem.put(1, start1);
    mem.put(2, start2);
    exec(mem).get(0)
}

fn exec(mut mem: Memory) -> Memory {
    const ADD: usize = 1;
    const MUL: usize = 2;
    const END: usize = 99;

    let mut pc: usize = 0; // program counter
    let mut opcode: usize;

    // check  opcode
    loop {
        opcode = mem.get(pc);
        match opcode {
            ADD => {
                let op1 = mem.get(mem.get(pc + 1));
                let op2 = mem.get(mem.get(pc + 2));
                let res = op1 + op2;
                //println!("{:>5}: ADD [{}], [{}] --> {}", pc, op1, op2, res);
                mem.put(mem.get(pc + 3), res)
            }
            MUL => {
                let op1 = mem.get(mem.get(pc + 1));
                let op2 = mem.get(mem.get(pc + 2));
                let res = op1 * op2;
                //println!("{:>5}: MUL [{}], [{}] --> {}", pc, op1, op2, res);
                mem.put(mem.get(pc + 3), res)
            }
            END => {
                //println!("{:>5}: END", pc);
                break;
            }
            _ => println!("ERROR: UNKNOWN OPCODE {} at position {}", opcode, pc),
        }
        pc += 4;
    }
    mem
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mem_empty() {
        let m = Memory::new(&[]);
        assert_eq!(0, m.get(0));
    }
    #[test]
    fn mem_in_bounds() {
        let m = Memory::new(&[0, 0, 1, 2, 3]);
        assert_eq!(1, m.get(2));
    }
    #[test]
    fn mem_out_bounds() {
        let m = Memory::new(&[0, 0, 1, 2, 3]);
        assert_eq!(0, m.get(5));
    }
    #[test]
    fn mem_put_in_bounds() {
        let mut m = Memory::new(&[0, 0, 1, 2, 3]);
        m.put(1, 9);
        assert_eq!(m.data, [0, 9, 1, 2, 3]);
    }
    #[test]
    fn mem_put_out_bounds() {
        let mut m = Memory::new(&[0, 1, 2, 3]);
        m.put(6, 9);
        assert_eq!(m.data, [0, 1, 2, 3, 0, 0, 9]);
    }

    #[test]
    fn mem_exec_1() {
        let m = Memory::new(&[1, 0, 0, 0, 99]);
        let mc = exec(m);
        assert_eq!(mc.data, [2, 0, 0, 0, 99]);
    }
    #[test]
    fn mem_exec_2() {
        let m = Memory::new(&[2, 3, 0, 3, 99]);
        let mc = exec(m);
        assert_eq!(mc.data, [2, 3, 0, 6, 99]);
    }
    #[test]
    fn mem_exec_3() {
        let m = Memory::new(&[2, 4, 4, 5, 99, 0]);
        let mc = exec(m);
        assert_eq!(mc.data, [2, 4, 4, 5, 99, 9801]);
    }
    #[test]
    fn mem_exec_4() {
        let m = Memory::new(&[1, 1, 1, 4, 99, 5, 6, 0, 99]);
        let mc = exec(m);
        assert_eq!(mc.data, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
