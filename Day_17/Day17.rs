use std::io::prelude::*;
use std::fs::File;


#[derive(Debug, PartialEq, Eq, Hash, Clone, Ord, PartialOrd)]
struct Computer {
    a: isize,
    b: isize,
    c: isize,
    instructions: Vec<isize>,
    ptr: usize,
    output: Vec<isize>,
}

impl Computer {
    fn new(register: Vec<isize>, instructions: Vec<isize>) -> Self {
        Self {
            a: register[0],
            b: register[1],
            c: register[2],
            instructions,
            ptr: 0,
            output: vec![],
        }
    }

    fn run(&mut self) {
        while self.ptr < self.instructions.len() -1 {
            let opcode = self.instructions[self.ptr];
            let operand = self.instructions[self.ptr + 1];
            
            self.program_blazing(opcode, operand);
        }
    }
    
    //Combo operands 0 through 3 represent literal values 0 through 3.
    //Combo operand 4 represents the value of register A.
    //Combo operand 5 represents the value of register B.
    //Combo operand 6 represents the value of register C.
    
    fn combo(&self, value: isize) -> isize {
        match value {
            0..=3 => value,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }
    
    //adv (opcode 0) == A register = (A register / (2 to the power of the instruction's combo operand))
            
    //bxl (opcode 1) == B register = bitwise XOR of register B and the instruction's literal operand.

    //bst (opcode 2) == B register = combo operand modulo 8 (thereby keeping only its lowest 3 bits)
    
    //jnz (opcode 3) if A register >= 1 ptr == value of its literal operand else continue
    
    //bxc (opcode 4) == B register = bitwise XOR of register B and register C
    
    //out (opcode 5) output appends the value of its combo operand modulo 8
    
    //bdv (opcode 6) works exactly like the adv instruction except that the result is stored in the B register.
    
    //cdv (opcode 7) works exactly like the adv instruction except that the result is stored in the C register
    
    fn program_blazing(&mut self, opcode: isize, operand: isize) {
        let mut jump = false;
        
        match opcode {
            // adv
            0 => {
                let num = self.a;
                let den = 2_i32.pow(self.combo(operand) as u32) as isize;
                self.a = num / den;
            }
            // bxl
            1 => {
                self.b = self.b ^ operand;
            }
            // bst
            2 => {
                self.b = self.combo(operand) % 8;
            }
            // jnz
            3 => {
                if self.a != 0 {
                    self.ptr = operand as usize;
                    jump =true;
                }
            }
            // bxc
            4 => {
                self.b = self.b ^ self.c;
            }
            // out
            5 => {
                self.output.push(self.combo(operand) % 8);
                //println!("{:?}",self.output);
            }
            // bdv
            6 => {
                let num = self.a;
                let den = 2_i32.pow(self.combo(operand) as u32) as isize;
                self.b = num / den;
            }
            // cdv
            7 => {
                let num = self.a;
                let den = 2_i32.pow(self.combo(operand) as u32) as isize;
                self.c = num / den;
            }
            // stop
            _ => unreachable!(),
        }
        
        if !jump{
           self.ptr += 2;
        }
        
    }
}


fn read_example_txt() -> String{
    let mut file = File::open("example.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    // println!("{}", contents);
    return contents;
}


fn punchcard(string_value: String) -> (Vec<isize>, Vec<isize>){

    let mut split_value = "\r\n\r\n";
    
    
    let (temp_registers, temp_instructions) = string_value.trim().split_once(split_value).unwrap();
    
    // Can't use regex as we're using an online compiler - So get funky
    let s:String = temp_registers.chars()
        .map(|x| match x { 
            '!' => '?', 
            'A'..='Z' => 'X', 
            'a'..='z' => 'x',
            _ => x
        }).collect();
        
    let mut registers = Vec::new();
        
    for line in s.lines(){
        let temp_register_value = line.split_once(": ").unwrap().1;
        registers.push(temp_register_value.parse::<isize>().unwrap())
    }
    
    let temp_instructions_ints = temp_instructions.split_once(": ").unwrap().1;
    
    //println!("{:?}", temp_registers);
    
    
    let instructions: Vec<isize> = temp_instructions_ints   
          .split(",")
          .flat_map(|x| x.parse::<isize>())  
          .collect();
    
    // println!("{:?}", registers);
    
    // println!("{:?}", instructions);
        
    return (registers, instructions)
}


fn part_one(){
    let string_value = read_example_txt();
    
    let (registers, instructions) = punchcard(string_value);
    
    let mut computer = Computer::new(registers, instructions);
    
    computer.run();
    
    let answer = computer.output.iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",");
    
    // Put var inside {} removes the "" either side 
    println!("The answer to part one is {answer}");
    
}


fn main() {    
    part_one();

    // Starting value (41644071) is too low. So I know at least I need to be working up.
    // I suspect that the use of modulo % 8 will be a key part in solving this smartly
    //part_two();

}