use std::fs;

fn get_file_content(file_path: &str) -> Vec<String>{
    fs::read_to_string(file_path)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
}

fn process_data(content: &Vec<String>) {

    let mut extra_iterations:Vec<i32> = vec![1; content.len()];

    for (cont_index, line) in content.iter().enumerate() {

        let mut split = line.split(":").last().unwrap().split("|");

        let player_nums: Vec<i32> = split
            .next()
            .unwrap()
            .split_whitespace()
            .flat_map(|x| x.parse::<i32>())
            .collect::<Vec<i32>>();

        let winning_nums= split
            .next()
            .unwrap()
            .split_whitespace()
            .flat_map(|x| x.parse::<i32>())
            .collect::<Vec<i32>>(); 

        let win_count = player_nums.iter().filter(|&x| winning_nums.contains(x)).count();

        for n in 0..win_count {
            for _ in 0..extra_iterations[cont_index] {
                extra_iterations[cont_index+n+1] += 1;
            }
        }

    }
    
    let total_iterations = extra_iterations.iter().sum::<i32>();
    println!("Total iterations: {}", total_iterations);
}

fn main() {
    let content = get_file_content("src/input.txt");
    process_data(&content);
}
