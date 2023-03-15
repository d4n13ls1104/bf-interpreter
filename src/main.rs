use std::{
    env, 
    fs, 
    path, 
    io,
    io::Read,
};

const MEM_SIZE: usize = 30000;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: bf <myprogram.bf>");
        std::process::exit(1);
    }

    let mut memory = vec![0u8; MEM_SIZE];
    let mut ptr: usize = 0;
    let mut stack: Vec<usize> = vec![];

    let program_path = path::PathBuf::from(&args[1]);

    let program: Vec<char> = fs::read_to_string(program_path)
        .expect("Couldn't read program file")
        .chars()
        .filter(|c| "+-<>[],.".contains(*c))
        .collect();

    let mut instruction_ptr: usize = 0;

    while instruction_ptr < program.len() {
        match program[instruction_ptr] {
            '>' => ptr = ptr.wrapping_add(1),
            '<' => ptr = ptr.wrapping_sub(1),
            '+' => memory[ptr] = memory[ptr].wrapping_add(1),
            '-' => memory[ptr] = memory[ptr].wrapping_sub(1),
            '[' => {
                if memory[ptr] == 0 {
                    let mut level = 0;
                    while instruction_ptr < program.len() {
                        if program[instruction_ptr] == '[' {
                            level += 1;
                        } else if program[instruction_ptr] == ']' {
                            level -= 1;
                            if level == 0 { break; }
                        }
                    }
                    instruction_ptr += 1;
                } else { stack.push(instruction_ptr); }
            },
            ']' => {
                if memory[ptr] != 0 {
                    instruction_ptr = stack[stack.len() - 1];
                } else { stack.pop(); }
            },
            ',' => {
                println!("Read char: ");
                let mut buf: [u8; 1] = [0; 1];
                io::stdin().read_exact(&mut buf)?;
                memory[ptr] = buf[0];
            },
            '.' => print!("{}", memory[ptr] as char),

            _ => ()
        }

        instruction_ptr += 1;
    }

    Ok(())
}
