use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn read_example_txt() -> String{
    let mut file = File::open("example.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    // println!("{}", contents);
    return contents;
}


fn part_one(){
    let string_value = read_example_txt();
    
    let mut crossword: HashMap<(i32, i32), char> = HashMap::new();
    
    let mut sum = 0;

    for (y, line) in string_value.lines().enumerate() {

        //println!("{}", line);

        for (x, c) in line.chars().enumerate() {
            crossword.entry((x as i32, y as i32)).or_insert(c);
        }
        
    }
    
    //                                  Right    Down    DRight   URight   Left      Up       DLeft    ULeft
    const DIRECTIONS: [(i32, i32); 8] = [(1, 0), (0, 1), (1, 1), (1, -1), (-1, 0), (0, -1), (-1, 1), (-1, -1)];

    for (key, value) in crossword.clone().into_iter() {
        
        if value == 'X'{
            let x = key.0;
            let y = key.1;
            
            for direction in DIRECTIONS{
            
                let mut possible_match = Vec::new();
                
                for i in 0..4{
                    //Need the unwrap_or(&' ') to avoid the value being Some('<value>') which buggers with my check
                    let next_char = crossword.get(&(x + direction.0*i, y + direction.1*i)).unwrap_or(&' ');
                    possible_match.push(next_char.to_string());
                    //println!("x = {:?} y = {:?}", direction.0 *i, direction.1 *i);
                }
                if possible_match.join("") == "XMAS"{
                    println!("{:?}", possible_match);
                    sum +=1;
                }
                
            }
            
        }
            
    }
    

    println!("The answer to part one is {}", sum);
}


fn part_one(){
    let string_value = read_example_txt();
    
    let mut crossword: HashMap<(i32, i32), char> = HashMap::new();
    
    let mut sum = 0;

    for (y, line) in string_value.lines().enumerate() {

        //println!("{}", line);

        for (x, c) in line.chars().enumerate() {
            crossword.entry((x as i32, y as i32)).or_insert(c);
        }
        
    }
    
    //                                  RDRight   URight   DLeft    ULeft
    const DIRECTIONS: [(i32, i32); 8] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

    for (key, value) in crossword.clone().into_iter() {
        
        if value == 'A'{
            let x = key.0;
            let y = key.1;
            
            for direction in DIRECTIONS{
            
                let mut possible_match = Vec::new();
                
                for i in 0..4{
                    //Need the unwrap_or(&' ') to avoid the value being Some('<value>') which buggers with my check
                    let next_char = crossword.get(&(x + direction.0*i, y + direction.1*i)).unwrap_or(&' ');
                    possible_match.push(next_char.to_string());
                    //println!("x = {:?} y = {:?}", direction.0 *i, direction.1 *i);
                }
                if possible_match.join("") == "MAS"{
                    println!("{:?}", possible_match);
                    sum +=1;
                }
                
            }
            
        }
            
    }
    

    println!("The answer to part one is {}", sum);
}


fn main() {    
    part_one();

    //part_two();
}