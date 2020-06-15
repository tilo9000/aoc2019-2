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

fn run(filename: &str) -> Result<usize, Box<dyn Error>> {

    let contents = fs::read_to_string(filename)?;
    let mut m: usize;
    let mut mem = Vec::new();

    let mut i: usize = 0;
    for code in contents.split(',') {
        m = code.trim().parse().unwrap();
        println!("{} -> {}", i, m);
        mem.push(m);
        i += 1;
    }

    exec(&mem);

    Ok(mem[0])
}

fn exec(mem: &Vec<usize>) {

}