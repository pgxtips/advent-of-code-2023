use std::fs::read_to_string;

const STR_NUMBERS:[&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];


fn get_file_content(file_path: &str) -> Vec<String>{
    let mut result = Vec::new();

    for line in read_to_string(file_path).unwrap().lines() {
        result.push(line.to_string());
    }

    return result;
}

fn process_line(line: &str) -> i32 {
    let mut result_string = String::new(); 

    // get all digits from line
    for (index, char) in line.chars().enumerate() {

        for (index2, pattern) in STR_NUMBERS.iter().enumerate() {
            if (index + pattern.len()) > line.len() {continue;}
            
            if &line[index..(index + pattern.len())].to_string() == pattern {
                result_string.push_str(&(index2 + 1).to_string());
                break;
            }
        }

        match char.to_digit(10){
            Some(_) => result_string.push(char),
            None => {continue},
        };
    }

    // get first and last digits from line
    if result_string.len() < 1 {return 0;}

    let mut result = String::new();

    let first_digit = result_string.chars().next().unwrap();
    let last_digit= result_string.chars().last().unwrap();
    
    result.push(first_digit);
    result.push(last_digit);

    return result.parse::<i32>().unwrap();
}

fn process_file(content: &Vec<String>) -> i32 {
    let mut result = 0;

    for line in content {
        let line_result = process_line(line);    
        result += line_result;
        println!("Result: {}", line_result);
    }

    return result;
}

fn main() {
    let input = get_file_content("src/input.txt");
    let result = process_file(&input);
    println!("Total: {}", result);
}
