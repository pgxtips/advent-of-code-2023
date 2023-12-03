use std::fs;

struct Block {
    value: String,
    positions: Vec<Coordinates>,
}

impl Block {
    fn new(value: String, positions: Vec<Coordinates>) -> Block {
        Block {value, positions}
    }
}


struct Coordinates {
    x: i32,
    y: i32,
}

impl Coordinates {
    fn new(x: i32, y: i32) -> Coordinates {
        Coordinates {x, y}
    }
}

fn is_special(c: char) -> bool {
    if !c.is_digit(10) && !c.is_alphabetic() && c != '.' {return true;}
    else {return false;} 
}

fn is_number(c: char) -> bool {
    if c.is_digit(10) {return true;}
    else {return false;}
}

fn get_file_content(file_path: &str) -> (Vec<Block>, Vec<Block>) {

    let mut result_numbers:Vec<Block> = Vec::new();
    let mut result_specials:Vec<Block> = Vec::new(); 

    for (index_line, lines) in fs::read_to_string(file_path).unwrap().lines().enumerate() {

        let mut block_numbers = Block::new(String::new(), Vec::new());
        let mut block_specials = Block::new(String::new(), Vec::new());

        for (index_char, char) in lines.chars().enumerate() {

            if is_number(char) {
                block_numbers.value.push(char);
                block_numbers.positions.push(Coordinates::new(index_char as i32, index_line as i32));
                if index_char == lines.len() - 1 {
                    result_numbers.push(block_numbers);
                    block_numbers = Block::new(String::new(), Vec::new());
                }
                continue;
            } 
            else {
                if block_numbers.value.len() > 0 {result_numbers.push(block_numbers);}
                block_numbers = Block::new(String::new(), Vec::new());
            }

            if is_special(char) {
                block_specials.value.push(char);
                block_specials.positions.push(Coordinates::new(index_char as i32, index_line as i32));
            } 
            else {
                if block_specials.value.len() > 0 {result_specials.push(block_specials);}
                block_specials= Block::new(String::new(), Vec::new());
            }

        }
        
    }

    (result_numbers, result_specials)
}

fn is_adjacent(number: &Block, special: &Block) -> bool {
    for number_pos in number.positions.iter() {
        for special_pos in special.positions.iter() {
            if (number_pos.x - special_pos.x).abs() <= 1 && (number_pos.y - special_pos.y).abs() <= 1 {
                return true;
            }
        }
    }
    return false;
}

fn process_data(numbers: &Vec<Block>, specials: &Vec<Block>){
    let mut result_sum: i32 = 0;
    let mut result_sum_gear_ratio: i32 = 0;

    for number in numbers {
        for special in specials {
            if is_adjacent(number, special) {
                let parsed_num = number.value.parse::<i32>().unwrap();
                //println!("{} is adjacent to {}", number.value, special.value);
                result_sum += parsed_num; 
            }
        }
    }

    for special in specials {
        if special.value == "*" {
            let mut gear_ratio_nums: Vec<i32> = Vec::new();

            for number in numbers {
                if is_adjacent(number, special) {
                    let parsed_num = number.value.parse::<i32>().unwrap();
                    gear_ratio_nums.push(parsed_num);
                }
            }

            if gear_ratio_nums.len() == 2 {
                let gear_ratio = gear_ratio_nums[0] * gear_ratio_nums[1];
                result_sum_gear_ratio += gear_ratio;
            } 
        }
    }

    
    println!("result gear ratio sum: {}", result_sum_gear_ratio);
    println!("result sum: {}", result_sum);
}

fn main() {
    let (numbers, specials) = get_file_content("src/input.txt");
    process_data(&numbers, &specials);
}
