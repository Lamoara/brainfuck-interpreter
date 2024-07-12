use std::{char, collections::btree_map::Range};

enum Instruction
{
    Add(u8),
    Subtract(u8),
    MoveLeft(usize),
    MoveRight(usize),
    IfJump(usize),
    IfNotJump(usize),
    Print,
    Read,
    Jump,
}

fn main() 
{
    let code: String= String::from(">++++++++++[-<+++++++++++>]<+              Inicialización de las celdas de memoria con 0 y 1 para Fibonacci
[                                            Inicio del bucle principal
    [-<+>]                                      Suma los dos números anteriores y avanza el puntero
    >+                                           Mueve el resultado a la siguiente celda
    <<<<                                        Vuelve al inicio del bucle principal
]                                            Fin del bucle principal
>+++++++.                                 Imprime el primer número (0)
>+.                                            Imprime el segundo número (1)
>+.                                            Imprime el tercer número (1)
>+.                                            Imprime el cuarto número (2)
>+.                                            Imprime el quinto número (3)
>+.                                            Imprime el sexto número (5)
>+.                                            Imprime el séptimo número (8)
>+.                                            Imprime el octavo número (13)
>+.                                            Imprime el noveno número (21)
>+.                                            Imprime el décimo número (34)
");
    
    let instructions: Vec<Instruction> = read_to_format(code).unwrap_or_else(|err| {panic!("Error reading file: {err}")});

    //print_instructions(&instructions);

    execute(instructions).unwrap();

    
}

fn read_to_format(code: String) -> Result<Vec<Instruction>, String>
{
    let mut instructions: Vec<Instruction> = Vec::new();


    let iter = code.chars();
    let mut dir_stack: Vec<usize> = Vec::new();
    let mut last_char: char = code.chars().next().unwrap();
    let mut acumulator: usize = 0;

    for char in iter
    {
        if last_char != char
        {
            match last_char {
                '+' => instructions.push(Instruction::Add(acumulator as u8)),
                '-' => instructions.push(Instruction::Subtract(acumulator as u8)),
                '>' => instructions.push(Instruction::MoveRight(acumulator)),
                '<' => instructions.push(Instruction::MoveLeft(acumulator)),
                '[' => {
                    dir_stack.push(instructions.len()); 
                    instructions.push(Instruction::Jump);
                },
                ']' => {
                    match dir_stack.pop() {
                        Some(dir) => {
                            instructions[dir] = Instruction::IfJump(instructions.len());
                            instructions.push(Instruction::IfNotJump(dir));
                        },
                        None => return Err(String::from("Syntax error: Unopened loop"))
                    }
                },
                '.' => instructions.push(Instruction::Print),
                ',' => instructions.push(Instruction::Read),
                _ => ()// return Err(format!("Syntax error: mismatched character '{}'", last_char))
            }
            acumulator = 0;
        }
        acumulator += 1;
        last_char = char;
    }

    Ok(instructions)
}

fn execute(instructions: Vec<Instruction>) -> Result<(), String>
{
    let mut mem: Vec<u8> = Vec::new();
    let mut current_index: usize = 0;
    let mut current_instruction: usize = 0;

    while current_instruction < instructions.len()
    {
        while mem.len() <= current_index {
            mem.push(0)
        }
        //print_instruction(&instructions[current_instruction]);
        //println!("Len: {}, Index: {}", mem.len(), current_index);

        match instructions[current_instruction] 
        {
            Instruction::Add(amount) => mem[current_index] = ((mem[current_index] + amount) as i32 % 256) as u8,
            Instruction::Subtract(amount) => mem[current_index] = ((mem[current_index] - amount)as i32 % 256) as u8,
            Instruction::MoveRight(amount) => current_index += amount,
            Instruction::MoveLeft(amount) => {
                if amount > current_index
                {
                    for _ in 0..(amount - current_index) 
                    {
                        mem.insert(0, 0);
                    }
                    current_index = 0
                }
                else{
                    current_index -= amount
                }
            },
            Instruction::IfJump(new_index) => {
                if mem[current_index] == 0{
                    current_instruction = new_index
                }
            },
            Instruction::IfNotJump(new_index) => {
                if mem[current_index] != 0{
                    current_instruction = new_index
                }
            },
            Instruction::Print => print!("{}", char::from_u32(mem[current_index] as u32).unwrap()), //println!("{:?}", mem),
            Instruction::Read => mem[current_index] = 2,
            Instruction::Jump => return Err(String::from("Runtime error: Unfinished loop")),
        }
        current_instruction += 1
    }

    Ok(())
}

#[allow(dead_code)]
fn print_instructions(instructions: &Vec<Instruction>)
{
    for instruction in instructions.iter()
    {
        print_instruction(instruction)
    }
}

fn print_instruction(instruction: &Instruction)
{
    match instruction {
        Instruction::Add(amount) => println!("Add({})", amount),
        Instruction::Subtract(amount) => println!("Subtract({})", amount),
        Instruction::MoveLeft(amount) => println!("MoveLeft({})", amount),
        Instruction::MoveRight(amount) => println!("MoveRight({})", amount),
        Instruction::IfJump(amount) => println!("IfJump({})", amount),
        Instruction::IfNotJump(amount) => println!("IfNotJump({})", amount),
        Instruction::Print => println!("Print"),
        Instruction::Read => println!("Read"),
        Instruction::Jump => println!("Jump"),
    }
}


