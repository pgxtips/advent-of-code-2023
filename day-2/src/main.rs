use std::fs;

struct GameSet{
    red: i32,
    blue: i32,
    green: i32,
}

struct Game{
    id: i32,
    game_sets: Vec<GameSet>,
}


impl GameSet{
    pub fn new() -> GameSet {
        GameSet {
            red: 0,
            blue: 0,
            green: 0,
        }
    }
}

impl Game {
    pub fn new() -> Game {
        Game {
            id: 0,
            game_sets: Vec::new(), 
        }
    }
}

fn get_file_content(file_path: &str) -> Vec<String> {

    let mut file_content: Vec<String> = Vec::new();
    for line in fs::read_to_string(file_path).unwrap().lines() {
       file_content.push(line.to_string())
    }
    return file_content;
}

fn process_line(line: &str) -> Game{
  let mut game = Game::new();
  let mut game_sets = Vec::<GameSet>::new();

  let split_1 = line.split(":").collect::<Vec<&str>>(); // split game id and game sets
  let split_2 = split_1[1].split(";").collect::<Vec<&str>>(); // split game sets

  game.id = split_1[0].split(" ").last().unwrap().parse::<i32>().unwrap();

  //split2 = ["red:1, blue:2, green:3", "red:1, blue:2, green:3"]
  for game_set in split_2 {
    let mut set = GameSet::new();

    //split3 = ["red:1", "blue:2", "green:3"]
    //removes trailing whitespaces from split
    let split_3: Vec<&str> = game_set.split(',').map(|val| val.trim()).collect();

    for val in split_3 {
      //split4 = ["red", "1"]
      let split_4 = val.split(" ").collect::<Vec<&str>>();

      match split_4[1] {
        "red" => set.red = split_4[0].parse::<i32>().unwrap(),
        "blue" => set.blue = split_4[0].parse::<i32>().unwrap(),
        "green" => set.green = split_4[0].parse::<i32>().unwrap(),
        _ => continue,
      }
    } 

    game_sets.push(set);
  }

  game.game_sets = game_sets;

  return game;
}

fn process_data(data: &Vec<String>) -> Vec<Game>{
    let mut games = Vec::<Game>::new();

    for line in data {
        games.push(process_line(line));
    }

    return games;
}

fn main() {
    let file_content = get_file_content("src/input.txt");
    let games_data = process_data(&file_content);

    let max_game_set = GameSet{red: 12, blue: 14, green: 13};

    let mut sum_of_possible = 0;
    let mut sum_power_set = 0;
    for game in games_data{
        let mut possible = true;
        let mut fewest_game_set = GameSet{red:0, green:0, blue:0}; 

        for game_set in game.game_sets{
            if game_set.red > max_game_set.red || 
               game_set.green > max_game_set.green || 
               game_set.blue > max_game_set.blue{
                possible = false;
            }
            
            if game_set.red > fewest_game_set.red {fewest_game_set.red = game_set.red;}
            if game_set.green > fewest_game_set.green {fewest_game_set.green = game_set.green;}
            if game_set.blue > fewest_game_set.blue {fewest_game_set.blue = game_set.blue;}
        }

        if possible {sum_of_possible += game.id;}

        let power_set = fewest_game_set.red * fewest_game_set.green * fewest_game_set.blue;
        println!("Game ID: {}, Power Set: {}", game.id, power_set);
        sum_power_set += power_set;
    } 

    println!("Sum of possible: {}", sum_of_possible);
    println!("Sum of power sets: {}", sum_power_set);
}
