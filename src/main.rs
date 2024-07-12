use std::{char, env, fs::File, io::{self, Read, Write}, path::Path, time::Instant};

enum BFInstruction
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
    let instructions: Vec<BFInstruction> = read_to_format(code).unwrap_or_else(|err| {panic!("Error parsing file: {err}")});

    execute(instructions).unwrap_or_else(|err| {panic!("{err}")});

    let duration = start.elapsed();

    println!();
    println!("Time elapsed is: {:?}", duration);
    
}


fn read_to_format(code: String) -> Result<Vec<BFInstruction>, String>
{
    let mut instructions: Vec<BFInstruction> = Vec::new();

    let mut dir_stack: Vec<usize> = Vec::new();
    let mut last_char: char = code.chars().next().unwrap();
    let mut acumulator: usize = 0;

    for char in code.chars()
    {
        if last_char != char
        {
            match last_char {
                '+' => instructions.push(BFInstruction::Add(acumulator as u8)),
                '-' => instructions.push(BFInstruction::Subtract(acumulator as u8)),
                '>' => instructions.push(BFInstruction::MoveRight(acumulator)),
                '<' => instructions.push(BFInstruction::MoveLeft(acumulator)),
                _ => ()
            }
            acumulator = 0;

        }
        match char {
            '[' => {
                dir_stack.push(instructions.len()); 
                instructions.push(BFInstruction::Jump);
            },
            ']' => {
                match dir_stack.pop() {
                    Some(dir) => {
                        instructions[dir] = BFInstruction::IfJump(instructions.len());
                        instructions.push(BFInstruction::IfNotJump(dir));
                    },
                    None => return Err(String::from("Syntax error: Unopened loop"))
                }
            },
            '.' => instructions.push(BFInstruction::Print),
            ',' => instructions.push(BFInstruction::Read),
            _ => ()
        }
        acumulator += 1;
        last_char = char;
    }

    Ok(instructions)
}


fn execute(instructions: Vec<BFInstruction>) -> Result<(), String>
{
    const MEM_SIZE: usize = 30000;
    let mut mem: [u8; MEM_SIZE] = [0; MEM_SIZE];
    let mut current_index: usize = 0;
    let mut current_instruction: usize = 0;

    while current_instruction < instructions.len()
    {
        match instructions[current_instruction] 
        {
            BFInstruction::Add(amount) => mem[current_index] = mem[current_index].wrapping_add(amount),
            BFInstruction::Subtract(amount) => mem[current_index] = mem[current_index].wrapping_sub(amount),
            BFInstruction::MoveRight(amount) => {
                current_index += amount;
                if current_index >= MEM_SIZE {
                    return Err(String::from("Runtime error: Memory overflow"));
                }
            },
            BFInstruction::MoveLeft(amount) => {
                if amount > current_index {
                    return Err(String::from("Runtime error: Negative indexing found"));
                } else {
                    current_index -= amount;
                }
            },
            BFInstruction::IfJump(new_index) => {
                if mem[current_index] == 0 {
                    current_instruction = new_index;
                }
            },
            BFInstruction::IfNotJump(new_index) => {
                if mem[current_index] != 0 {
                    current_instruction = new_index;
                }
            },
            BFInstruction::Print => print!("{}", char::from_u32(mem[current_index] as u32).unwrap()),
            BFInstruction::Read => {
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Runtime error: Error reading input");
                if let Some(character) = input.chars().next() {
                    mem[current_index] = character as u8;
                } else {
                    return Err(String::from("Runtime error: Missed input"));
                }
            },
            BFInstruction::Jump => return Err(String::from("Runtime error: Unfinished loop")),
        }
        current_instruction += 1;
    }
    
    Ok(())
}



#[allow(dead_code)]
fn print_instructions(instructions: &Vec<BFInstruction>)
{
    for instruction in instructions.iter()
    {
        print_instruction(instruction)
    }
}


fn print_instruction(instruction: &BFInstruction)
{
    match instruction {
        BFInstruction::Add(amount) => println!("Add({})", amount),
        BFInstruction::Subtract(amount) => println!("Subtract({})", amount),
        BFInstruction::MoveLeft(amount) => println!("MoveLeft({})", amount),
        BFInstruction::MoveRight(amount) => println!("MoveRight({})", amount),
        BFInstruction::IfJump(amount) => println!("IfJump({})", amount),
        BFInstruction::IfNotJump(amount) => println!("IfNotJump({})", amount),
        BFInstruction::Print => println!("Print"),
        BFInstruction::Read => println!("Read"),
        BFInstruction::Jump => println!("Jump"),
    }
}


fn read_file_to_string(file_path: &str) -> Result<String, String> {
    // Intentamos abrir el archivo en modo lectura
    let path = Path::new(file_path);
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => return Err(format!("Error opening file '{}': {}", file_path, err)),
    };

    // Creamos un buffer para almacenar el contenido del archivo
    let mut content = String::new();
    
    // Leemos el contenido del archivo dentro del buffer
    match file.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(err) => Err(format!("Error reading file '{}': {}", file_path, err)),
    }
}
