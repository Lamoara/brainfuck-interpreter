use std::{char, env, fmt::format, fs::File, io::{self, Read, Write}, path::Path, time::Instant};

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
    let args: Vec<String> = env::args().collect();


    let file_path = &args[1][..];
    let code = read_file_to_string(file_path).unwrap();
    
    let start = Instant::now();
    let instructions: Vec<Instruction> = read_to_format(code).unwrap_or_else(|err| {panic!("Error reading file: {err}")});

    //print_instructions(&instructions);

    execute(instructions).unwrap();

    let duration = start.elapsed();

    println!();
    println!("Time elapsed is: {:?}", duration);
    
}


fn read_to_format(code: String) -> Result<Vec<Instruction>, String>
{
    let mut instructions: Vec<Instruction> = Vec::new();

    let mut dir_stack: Vec<usize> = Vec::new();
    let mut last_char: char = code.chars().next().unwrap();
    let mut acumulator: usize = 0;

    for char in code.chars()
    {
        if last_char != char
        {
            match last_char {
                '+' => instructions.push(Instruction::Add(acumulator as u8)),
                '-' => instructions.push(Instruction::Subtract(acumulator as u8)),
                '>' => instructions.push(Instruction::MoveRight(acumulator)),
                '<' => instructions.push(Instruction::MoveLeft(acumulator)),
                _ => ()
            }
            acumulator = 0;

        }
        match char {
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
            _ => ()
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
        if mem.len() <= current_index{
            let additional_size = current_index - mem.len() + 1;
            mem.resize(mem.len() + additional_size, 0);
        }
        //print_instruction(&instructions[current_instruction]);
        //println!("Len: {}, Index: {}", mem.len(), current_index);

        match instructions[current_instruction] 
        {
            Instruction::Add(amount) => mem[current_index] = mem[current_index].wrapping_add(amount),
            Instruction::Subtract(amount) => mem[current_index] = mem[current_index].wrapping_sub(amount),
            Instruction::MoveRight(amount) => current_index += amount,
            Instruction::MoveLeft(amount) => {
                if amount > current_index
                {
                    return Err(format!("Negative indexing found"));
                    // for _ in 0..(amount - current_index) 
                    // {
                    //     mem.insert(0, 0);
                    // }
                    // current_index = 0
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
            Instruction::Read => {
                io::stdout().flush().unwrap();
                let mut input = String::new();

                io::stdin().read_line(&mut input).expect("Runtime error: Error reading input");

                if let Some(character) = input.chars().next() {
                    mem[current_index] = character as u8;
                } else {
                    return Err(String::from("Runtime error: Missed input"));
                }
            },
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


fn read_file_to_string(file_path: &str) -> Result<String, String> {
    // Intentamos abrir el archivo en modo lectura
    let path = Path::new(file_path);
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => return Err(format!("Error al abrir el archivo '{}': {}", file_path, err)),
    };

    // Creamos un buffer para almacenar el contenido del archivo
    let mut content = String::new();
    
    // Leemos el contenido del archivo dentro del buffer
    match file.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(err) => Err(format!("Error al leer el archivo '{}': {}", file_path, err)),
    }
}
