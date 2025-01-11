use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::collections::VecDeque;



fn read_example_txt() -> String{
    let mut file = File::open("example.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    // println!("{}", contents);
    return contents;
}


fn punchcard_v2(string_value: &String) -> (HashMap<&str, bool>, Vec<(&str, &str, &str, &str)>){

    let mut split_value = "\r\n\r\n";
    
    let (temp_init_values, temp_instructions) = string_value.trim().split_once(split_value).unwrap();
    
    let mut init_values: HashMap<&str, bool> = HashMap::new();
    
    for line in temp_init_values.lines(){
        //println!("{:?}", line);
        let (var, val) = line.split_once(": ").unwrap();
        // if val.parse::<i64>().unwrap() != 0 (true or false)
         init_values.entry(var).or_insert(val.parse::<i64>().unwrap() != 0);
    }
    
    
    let mut instructions = Vec::new();
    
    for line in temp_instructions.lines(){
        //println!("{:?}", line);
        let split = line.split_whitespace().collect::<Vec<_>>();
        // println!("{:?}", split[0]);
        // println!("{:?}", split[1]);
        // println!("{:?}", split[2]);
        // println!("{:?}", split[4]);
        
        //                 value1      op       value2   value out
        instructions.push((split[0], split[1], split[2], split[4]))
    }
        
    return (init_values, instructions)
}


fn read(memory: &HashMap<&str, bool>, ch: char) -> usize {
    let mut char_values_only: Vec<(&&str, &bool)> = memory
        .iter()
        .filter(|(var, _)| var.starts_with(ch))
        .collect();

    char_values_only.sort_by(|(var1, _), (var2, _)| var1.cmp(var2));

    char_values_only.reverse();
    
    //println!("{:?}", char_values_only);
    
    let binary_string: String = char_values_only
        .into_iter()
        .map(|(_, value)| if *value { '1' } else { '0' })
        .collect();
    
    //println!("{:?}", binary_string);

    return usize::from_str_radix(&binary_string, 2).unwrap();
}


fn part_one(){
    let string_value = read_example_txt();
    
    
    let (init_values, instructions) = punchcard_v2(&string_value);
    
    // for (key, value) in init_values{
    //     println!("{:?},{:?}", key, value);
    // }
    
    // for item in instructions
    
    let mut memory = init_values.clone();
    let mut queue = VecDeque::from(instructions.clone());

    while let Some((value_1, op, value_2, out)) = queue.pop_front() {
        if let (Some(&left), Some(&right)) = (memory.get(value_1), memory.get(value_2)) {
            let res = match op {
                "AND" => left & right,
                "OR" => left | right,
                "XOR" => left ^ right,
                _ => unreachable!(),
            };
            memory.insert(out, res);
        } else {
            queue.push_back((value_1, op, value_2, out));
            continue;
        }
    }
    
    let result = read(&memory, 'z');

    println!("The answer to part one is {}", result);
}


fn main() {    
    part_one();

    // Starting value (41644071) is too low. So I know at least I need to be working up.
    // I suspect that the use of modulo % 8 will be a key part in solving this smartly
    //part_two();

}