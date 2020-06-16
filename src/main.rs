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

impl Memory{
    pub fn new(init: &[usize]) -> Memory {
        let mut nv = Memory {
            data: Vec::new(),
        };
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

    pub fn put(&mut self, idx:usize, value: usize) {
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
}


fn run(filename: &str) -> Result<usize, Box<dyn Error>> {

    let contents = fs::read_to_string(filename)?;
    let mut m: usize;
    let mut mem = Memory::new(&[]);

    let mut i: usize = 0;
    for code in contents.split(',') {
        m = code.trim().parse().unwrap();
        println!("{} -> {}", i, m);
        mem.push(m);
        i += 1;
    }

    let changed_mem = exec(mem);

    Ok(changed_mem.get(0))
}

fn exec(mut mem: Memory) -> Memory {
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
        let m = Memory::new(&[0,0,1,2,3]);
        assert_eq!(1, m.get(2));
    }
    #[test]
    fn mem_out_bounds() {
        let m = Memory::new(&[0,0,1,2,3]);
        assert_eq!(0, m.get(5));
    }
    #[test]
    fn mem_put_in_bounds() {
        let mut m = Memory::new(&[0,0,1,2,3]);
        m.put(1, 9);
        assert_eq!(m.data,[0,9,1,2,3]);
    }
    #[test]
    fn mem_put_out_bounds() {
        let mut m = Memory::new(&[0,1,2,3]);
        m.put(6, 9);
        assert_eq!(m.data,[0,1,2,3,0,0,9]);
    }
}