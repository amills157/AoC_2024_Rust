use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use std::collections::HashMap;

fn read_example_txt() -> String{
    let mut file = File::open("input.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    // println!("{}", contents);
    return contents;
}

fn solve_for_prize(matrix: HashMap<&str, isize>) -> isize{

    let mut tokens = 0;

    let a_x = matrix.get(&"A_x").unwrap();
    let a_y = matrix.get(&"A_y").unwrap();

    let b_x = matrix.get(&"B_x").unwrap();
    let b_y = matrix.get(&"B_y").unwrap();

    let p_x = matrix.get(&"P_x").unwrap();
    let p_y = matrix.get(&"P_y").unwrap();

    // Cramers rule
    let btn_a_count = (p_x * b_y - p_y * b_x) / (a_x * b_y - a_y * b_x);

    let btn_b_count = (p_y * a_x - p_x * a_y) / (a_x * b_y - a_y * b_x);

    //println!("Button A Count {:?}", btn_a_count);

    //println!("Button B Count {:?}", btn_b_count);

    if *p_x == (btn_a_count * a_x + btn_b_count * b_x) && *p_y == (btn_a_count * a_y + btn_b_count * b_y){
        tokens += 3 * btn_a_count;
        tokens += btn_b_count;
    }

    return tokens;

}



fn part_one(){
    let string_value = read_example_txt();

    let mut matrix: HashMap<&str, isize> = HashMap::new();

    let mut sum = 0;
    
    for line in string_value.lines(){
        //println!("{}", line);
        let x_regex = Regex::new(r"[0-9]{1,9999}").unwrap();

        let mut temp = Vec::new();

        for mat in x_regex.find_iter(line){
            let match_isize = mat.as_str().to_string().parse::<isize>().unwrap();
            temp.push(match_isize);
        }

        if line.contains("Button A") {
            //println!("Button A x {} + y {}", temp[0], temp[1]);
            matrix.entry("A_x").or_insert(temp[0] as isize);
            matrix.entry("A_y").or_insert(temp[1] as isize);
        } else if line.contains("Button B"){
            //println!("Button B x {} + y {}", temp[0], temp[1]);
            matrix.entry("B_x").or_insert(temp[0] as isize);
            matrix.entry("B_y").or_insert(temp[1] as isize);
        } else if line.contains("Prize"){
            //println!("Target x {} + y {}", temp[0], temp[1]);
            matrix.entry("P_x").or_insert(temp[0] as isize);
            matrix.entry("P_y").or_insert(temp[1] as isize);

            sum += solve_for_prize(matrix.clone());

            matrix.clear();

        }
    }
    println!("The answer to part one is {}", sum);
}


fn part_two(){
    let string_value = read_example_txt();

    let mut matrix: HashMap<&str, isize> = HashMap::new();

    let mut sum = 0;
    
    for line in string_value.lines(){
        //println!("{}", line);
        let x_regex = Regex::new(r"[0-9]{1,9999}").unwrap();

        let mut temp = Vec::new();

        for mat in x_regex.find_iter(line){
            let match_isize = mat.as_str().to_string().parse::<isize>().unwrap();
            temp.push(match_isize);
        }

        if line.contains("Button A") {
            //println!("Button A x {} + y {}", temp[0], temp[1]);
            matrix.entry("A_x").or_insert(temp[0] as isize);
            matrix.entry("A_y").or_insert(temp[1] as isize);
        } else if line.contains("Button B"){
            //println!("Button B x {} + y {}", temp[0], temp[1]);
            matrix.entry("B_x").or_insert(temp[0] as isize);
            matrix.entry("B_y").or_insert(temp[1] as isize);
        } else if line.contains("Prize"){
            //println!("Target x {} + y {}", temp[0], temp[1]);
            matrix.entry("P_x").or_insert(temp[0] as isize + 10000000000000);
            matrix.entry("P_y").or_insert(temp[1] as isize + 10000000000000);

            sum += solve_for_prize(matrix.clone());

            matrix.clear();

        }
    }
    println!("The answer to part one is {}", sum);
}


fn main() {    
    part_one();

    part_two();
}