use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::ops::Add;
use std::ops::AddAssign;



#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
    
}

impl Add for Point{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}


fn read_example_txt() -> String{
    let mut file = File::open("example.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    // println!("{}", contents);
    return contents;
}

fn MomCorp_Map(string_value: String, pt2: bool) -> (HashMap<Point, char>, Vec<Point>){

    //println!("{:?}", string_value);
    
    // I think something to do with the way I've replaced values or the fact I'm using an online compilier has messed with this
    // So even with trim when I do the replace the \r remain.
    let mut split_value = "";
    
    if pt2{
        split_value = "\r\n\r\n";
    } else{
        split_value = "\n\n";
    }
    
    
    let (temp_map, temp_moves) = string_value.trim().split_once(split_value).unwrap();
    

    let mut map: HashMap<Point, char> = HashMap::new();
    
    for (y, line) in temp_map.lines().enumerate() {

        for (x, c) in line.chars().enumerate() {
            map.entry(Point::new(x as i32, y as i32)).or_insert(c);
        }
        
    } 
    
    //println!("{:?}", map);
    
    //                                             Right    Down       Left      Up
    //const ADJANCENT_DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    
    let mut moves = Vec::new();
        
    for char in temp_moves.chars(){
        match char {
            '<' => moves.push(Point::new(-1, 0)),
            '>' => moves.push(Point::new(1, 0)),
            'v' => moves.push(Point::new(0, 1)),
            '^' => moves.push(Point::new(0, -1)),
            //_ => println!("{:?}", char),
            _ => continue,
        }
        
    }
        
    //println!("{:?}", moves);
        
    return (map, moves)
}


fn robot_map_blazing(map: &mut HashMap<Point, char>, moves: &Vec<Point>){
    let mut robot = map
        .iter()
        .find(|(_, &ch)| ch == '@')
        .map(|(k, _)| *k)
        .unwrap();
        
    //println!("{:?}", robot);
    
    'outer: for dir in moves {
        let mut queue = VecDeque::from([robot]);
        let mut visited = vec![];

        while let Some(current_pos) = queue.pop_front() {
            if visited.contains(&current_pos) {
                continue;
            }
            visited.push(current_pos);

            let next_pos = current_pos + *dir;

            match map[&next_pos] {
                '@' => continue,
                '.' => continue,
                '#' => continue 'outer,
                'O' => queue.push_back(next_pos),
                '[' => {
                    queue.push_back(next_pos);
                    queue.push_back(next_pos + Point::new(1, 0));
                }
                ']' => {
                    queue.push_back(next_pos);
                    queue.push_back(next_pos + Point::new(-1, 0));
                }
                _ => unreachable!(),
            }
        }

        while let Some(current_pos) = visited.pop() {
            let next_pos = current_pos + *dir;

            if !visited.contains(&next_pos) {
                map.insert(next_pos, map[&current_pos]);
                map.insert(current_pos, '.');
            }
        }

        robot += *dir;
    }
}


fn goods_pos_sys(map: &HashMap<Point, char>, marker: char) -> i32 {
    
    let mut sum = 0;
    
    for (point, char) in map.into_iter(){
        if *char == marker{
            sum += 100 * point.y + point.x;
        }
    }
    
    return sum;
}


fn part_one(){
    let string_value = read_example_txt().replace('\r', "");
    
    let (mut map, moves) = MomCorp_Map(string_value, false);
    
    robot_map_blazing(&mut map, &moves);
    
    let sum = goods_pos_sys(&map, 'O');
    
    println!("The answer to part one is {}", sum);
    
}


fn part_two(){
    let string_value = read_example_txt();

    let wider_map = string_value
        .replace("#", "##")
        .replace("O", "[]")
        .replace(".", "..")
        .replace("@", "@.");
    
    let (mut map, moves) = MomCorp_Map(wider_map, true);
    
    //println!("{:?}", map);
    
    robot_map_blazing(&mut map, &moves);
    
    let sum = goods_pos_sys(&map, '[');
    
    println!("The answer to part two is {}", sum);

}


fn main() {    
    part_one();

    part_two();
}