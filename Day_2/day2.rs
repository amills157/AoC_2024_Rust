use std::io::prelude::*;
use std::fs::File;

fn read_example_txt() -> String{
    let mut file = File::open("example.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    // println!("{}", contents);
    return contents;
}

fn part_one() {

    let mut safe_count = 0;

    let string_value = read_example_txt();

    for line in string_value.lines() {
        
        let int_vec: Vec<i32> = line.split(' ').map(|x|->i32{x.parse().unwrap()}).collect();
        
       //println!("{:?}", int_vec); 
       
       let mut int_vec_diff: Vec<i32> = int_vec.windows(2).map(|s| s[1] - s[0]).collect();
       
       if (int_vec_diff.iter().all(|&i| i >= 0)) || (int_vec_diff.iter().all(|&i| i <= 0)) {
            
            for i in 0..int_vec_diff.len() {
                int_vec_diff[i] = int_vec_diff[i].abs();
            }
            
            // println!("{:?}", int_vec_diff);
            
            if !int_vec_diff.iter().any(|&i| i > 3 || i == 0 ) {
                //println!("Safe");
                safe_count += 1;
                //println!("{}", safe_count);
            }
       }
       
       //println!("----------------------------");
       
    }
    
    println!("The answer to part one is {}", safe_count);

}


fn part_two() {

    let mut safe_count = 0;

    let string_value = read_example_txt();

    for line in string_value.lines() {
        
        let int_vec: Vec<i32> = line.split(' ').map(|x|->i32{x.parse().unwrap()}).collect();
        
        //println!("{:?}", int_vec);
        
        for i in 0..int_vec.len() {
            let mut modified_vec = Vec::with_capacity(int_vec.len() - 1);
            modified_vec.extend_from_slice(&int_vec[..i]);
            modified_vec.extend_from_slice(&int_vec[i + 1..]);
            
            //println!("{:?}", modified_vec);
        
           let mut int_vec_diff: Vec<i32> = modified_vec.windows(2).map(|s| s[1] - s[0]).collect();
           
           if (int_vec_diff.iter().all(|&i| i >= 0)) || (int_vec_diff.iter().all(|&i| i <= 0)) {
                
                for i in 0..int_vec_diff.len() {
                    int_vec_diff[i] = int_vec_diff[i].abs();
                }
                
                //println!("{:?}", int_vec_diff);
                
                if !int_vec_diff.iter().any(|&i| i > 3 || i == 0 ) {
                    //println!("Safe");
                    safe_count += 1;
                    //println!("{}", safe_count);
                    break;
                }
                
            }
       }
       
       //println!("----------------------------");
       
    }
    
    println!("The answer to part two is {}", safe_count);

}

// https://www.onlinegdb.com/online_rust_compiler
fn main() {    
    part_one();
    part_two();
}