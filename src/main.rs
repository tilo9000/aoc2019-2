use std::error::Error;
use std::fs;
use std::process;

fn main() {
    match run("input.txt") {
        Ok(t) => println!("Memory Cell 0: {}", t),
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

fn run(filename: &str) -> Result<usize, Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let mut m: usize;
    let mut mem = Memory::new(&[]);

    for code in contents.split(',') {
        m = code.trim().parse().unwrap();
        mem.push(m);
    }

    mem.dump();

    // Once you have a working computer, 
    // the first step is to restore the gravity assist program
    // (your puzzle input) to the "1202 program alarm" state 
    // it had just before the last computer caught fire. 
    // To do this, before running the program, 
    // replace position 1 with the value 12 and 
    // replace position 2 with the value 2. 
    // What value is left at position 0 after the program halts?
    mem.put(1, 12);
    mem.put(2, 2);

    let changed_mem = exec(mem);

    changed_mem.dump();

    Ok(changed_mem.get(0))
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
                println!(
                    "{}, {}, {}, {}",
                    opcode,
                    mem.get(pc + 1),
                    mem.get(pc + 2),
                    mem.get(pc + 3)
                );
                let op1 = mem.get(mem.get(pc + 1));
                let op2 = mem.get(mem.get(pc + 2));
                let res = op1 + op2;
                println!("{:>5}: ADD [{}], [{}] --> {}", pc, op1, op2, res);
                mem.put(mem.get(pc + 3), res)
            }
            MUL => {
                println!(
                    "{}, {}, {}, {}",
                    opcode,
                    mem.get(pc + 1),
                    mem.get(pc + 2),
                    mem.get(pc + 3)
                );
                let op1 = mem.get(mem.get(pc + 1));
                let op2 = mem.get(mem.get(pc + 2));
                let res = op1 * op2;
                println!("{:>5}: MUL [{}], [{}] --> {}", pc, op1, op2, res);
                mem.put(mem.get(pc + 3), res)
            }
            END => {
                println!("{:>5}: END", pc);
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
