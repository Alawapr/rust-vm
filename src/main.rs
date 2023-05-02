use colored::control;
use colored::Colorize;

mod vm;
use crate::{parser::parse_program_string, vm::*};

pub mod parser;

pub const VM_STACK_CAPACITY: usize = 1024;
pub const PROGRAM_LENGTH: usize = 1024;

fn main() {
    control::set_override(true);

    let time = std::time::Instant::now();
    let mut vm = VM {
        stack: [0; VM_STACK_CAPACITY],
        stack_size: 0,
        halt: false,
    };

    let programstring = r#"Push 5
    Push 10
    Push 3
    Mul      
    Plus   
    Push 7   
    Minus   
    Push 2  
    Div      
    Pop      
    Push 6
    Push 4
    Plus     
    Halt // Remove if you want it to continue running
    Push 2
    Div     
    Push 3
    Mul     
    Push 8
    Minus    
    Push 9
    Div      
    Pop"#;

    let program = parse_program_string(programstring);

    let err = run(&mut vm, program);
    if err != Error::OK {
        println!("{}", error_to_string(err));
        return;
    }

    println!("\nStack: ");
    if vm.stack_size == 0 {
        println!("    {}", "Empty".red().bold());
    } else {
        for i in 0..vm.stack_size {
            if !(vm.stack[i] == 0) {
                println!("    {}", vm.stack[i].to_string().blue());
            } else {
                println!("    {}", "Empty / 0".red().bold());
            }
        }
    }

    println!("\nTime taken to execute: {:#?}", time.elapsed());
}

fn error_to_string(error: Error) -> String {
    match error {
        Error::OK => "\nProgram finished".green().to_string(),
        Error::StackOverflow => "ERROR: Stack overflow".bold().red().to_string(),
        Error::StackUnderflow => "ERROR: Stack underflow".bold().red().to_string(),
        Error::DivisionByZero => "ERROR: Division by zero".bold().red().to_string(),
        _ => "ERROR: Unknown error".red().bold().to_string(),
    }
}
