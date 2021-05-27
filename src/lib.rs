use std::error::Error;
use std::fs;
use std::io;

use easy_error::bail;

pub fn run(filename: &str) -> Result<(), Box<dyn Error>> {
    let src = fs::read_to_string(filename)?;
    interpret(&lex(&src))?;
    Ok(())
}

fn interpret(src: &str) -> Result<(), Box<dyn Error>> {
    let mut tape: Vec<u8> = vec![0];
    let mut ptr = 0; // Memory pointer

    let instructions: Vec<char> = src.chars().collect();
    let mut loop_stack = Vec::new();
    let mut i_ptr = 0; // Instruction pointer

    while i_ptr < instructions.len() {
        let i = instructions[i_ptr];

        match i {
            '+' => tape[ptr] = tape[ptr].overflowing_add(1_u8).0, // 255 + 1 wraps around to 0
            '-' => tape[ptr] = tape[ptr].overflowing_sub(1_u8).0, // 0 - 1 wraps around to 255
            '>' => {
                ptr += 1;

                match tape.get(ptr) {
                   Some(_) => (),
                   None => tape.push(0),
                }
            }
            '<' => ptr = ptr.saturating_sub(1), // Pointer won't go below 0
            '[' => {
                if tape[ptr] == 0 {
                    let mut open = 1;

                    while open > 0 {
                        i_ptr += 1;

                        if let Some(i) = instructions.get(i_ptr) {
                            match i {
                                '[' => open += 1,
                                ']' => open -= 1,
                                _ => continue,
                            }
                        } else {
                            bail!("unmatched loop encountered");
                        }
                    }
                } else {
                    loop_stack.push(i_ptr)
                }
            }
            ']' => {
                if let Some(&i) = &mut loop_stack.last() {
                    if tape[ptr] == 0 {
                        loop_stack.pop();
                    } else {
                        i_ptr = i;
                    }
                } else {
                    bail!("unmatched loop encountered");
                }
            }
            ',' => {
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                tape[ptr] = input.chars().next().unwrap() as u8;
            }
            '.' => print!("{}", tape[ptr] as char),
            _ => bail!("lexing error: non-token found during interpretation"),
        }

        i_ptr += 1;
    }

    Ok(())
}

fn lex(src: &str) -> String {
    let mut result = String::new();

    for ch in src.chars() {
        match ch {
            '+' | '-' | '>' | '<' | '[' | ']' | ',' | '.' => result.push(ch),
            _ => continue,
        }
    }

    result
}

#[test]
fn tic_tac_toe() {
    run("tic-tac-toe.bf").unwrap();
}