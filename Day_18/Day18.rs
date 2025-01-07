use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::ops::Add;
use std::ops::AddAssign;


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Ord, PartialOrd)]
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


fn mem_map(string_value: String, max_x: i32, max_y: i32, max_bytes: usize) -> (HashMap<Point, char>, Vec<Point>){

    let mut bytes = Vec::new();
    
    for line in string_value.lines(){
        let (x, y) = line.split_once(',').unwrap();
        let temp = Point::new(x.parse().unwrap(), y.parse().unwrap());
        bytes.push(temp);
    }
    
    let mut map: HashMap<Point, char> = HashMap::new();
    
    // I'm a moron who did not take into a account that these loops needed to be max+1 to work for waaayyy to long
    for x in 0..max_x+1 {

        for y in 0..max_y+1 {
            map.entry(Point::new(x as i32, y as i32)).or_insert('.');
        }
        
    }
    
    for byte in 0..max_bytes{
        map.insert(bytes[byte], '#' );
    }
        
    return (map, bytes)
}


fn print_map(map: &HashMap<Point, char>, max_x: i32, max_y: i32) {
    let mut grid = vec![vec!['.'; max_x as usize]; max_y as usize];

    for (point, &ch) in map {
        if point.x < max_x && point.y < max_y {
            grid[point.y as usize][point.x as usize] = ch; // Note: y is the row, x is the column
        }
    }

    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}


fn ram_running(map: &mut HashMap<Point, char>, start: &Point, end: &Point) -> Option<usize> {
    let mut best_score = 99999999;
    let mut visited: HashSet<Point> = HashSet::default();
    let mut queue = VecDeque::new();
    queue.push_back((0, *start));
    
    // //                                             Right    Down       Left      Up
    // const ADJANCENT_DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    
    let mut adj_dirs = Vec::new();
    
    adj_dirs.push(Point::new(1, 0));
    adj_dirs.push(Point::new(0, 1));
    adj_dirs.push(Point::new(-1, 0));
    adj_dirs.push(Point::new(0, -1));

    while let Some((score, pos)) = queue.pop_front() {
        if score > best_score {
            continue;
        }
        if !visited.insert(pos) {
            continue;
        }
        if pos == *end {
            best_score = score;
            continue;
        }

        for dir in &adj_dirs {
            let neighbour = pos + *dir;
            //println!("{:?} == {:?}", neighbour, map.get(&neighbour));
            if map.get(&neighbour) == Some(&'.') {
                queue.push_back((score + 1, neighbour));
            }
        }
    }
    
    // for pos in visited{
    //     //map.entry(bytes[byte]).or_insert('#');
    //     map.insert(pos, 'O' );
    // }

    if best_score == 99999999 {
        None
    } else {
        Some(best_score)
    }
}


fn part_one_and_two(){
    let string_value = read_example_txt();
    
     
    let max_x = 70;
    
    let max_y = 70;
    
    let max_bytes = 1024;
    
    let start = Point::new(0, 0);
    let end = Point::new(max_x, max_y);
    
    let (mut map, bytes) = mem_map(string_value, max_x, max_y, max_bytes);
    
    // for row in map{
    //     println!("{:?}", row);
    // }
    
    //print_map(&map, max_x+1, max_y+1);
    
    let best_score = ram_running(&mut map, &start, &end);

    println!("The answer to part one is {:?}", best_score.unwrap());
    
    for i in max_bytes..bytes.len() {
        map.insert(bytes[i], 'O' );

        if ram_running(&mut map, &start, &end).is_none() {
            println!("The answer to part two is {:?},{:?}", bytes[i].x, bytes[i].y);
            break;
        }
        
    }
    
    //print_map(&map, max_x+1, max_y+1);
    
}


fn main() {    
    part_one_and_two();

}