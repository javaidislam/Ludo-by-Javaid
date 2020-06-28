use std::io;
use std::collections::HashMap;
use rand::Rng;
use std::convert::TryInto;

fn main() {
    println!("Welcome to the game!");
    println!("Please enter numbers of players");
    let mut no_players = String::new();
    io::stdin().read_line(&mut no_players).expect("error in reading");
    let no_players: i32 = no_players.trim().parse().expect("Please type a numeric value greater than zero!");
    let mut player_names:Vec<String> = Vec::new();
    let mut player_total :Vec<u32> = Vec::new();
    let max_score = 3;
    let mut winner_score = 0;
    let mut turn_count = 1;
    
    //let participants = HashMap::new();
    let mut i = 1;
    for names in (0..no_players) {
        println!("Enter Name of Player {}", i );
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("please re enter name");
        name = name.trim().to_string();
        player_names.push(name);
        player_total.push(0);
        i +=1;
    }
    println!("{}",i);
    println!("Welcome to the game {:?}",player_names);
    let gamers : HashMap<_,_> = player_names.iter().zip(player_total.iter()).collect();
    println!("The Game Started {:#?}", gamers);
    while winner_score < max_score {
        let mut j = 0;
            for player_score in &player_names {
            let mut turn_score = rand::thread_rng().gen_range(1, 7);
            if turn_score != 6 { 
                turn_score = turn_score + 0;
            } else {
                let mut turn_aft_1st_6 = rand::thread_rng().gen_range(1, 7);
                if turn_aft_1st_6 !=6 {
                    turn_score = turn_score + turn_aft_1st_6;
                } else { 
                    let mut turn_aft_2nd_6 = rand::thread_rng().gen_range(1, 7);
                    if turn_aft_2nd_6 !=6 {
                        turn_score = turn_aft_1st_6 + turn_aft_2nd_6 + turn_score;
                    } else {
                        turn_score = 0;
                    }
                }
            }
            player_total[j] = player_total[j] + turn_score;
            if player_total[j]> 100 {
                player_total[j] = player_total[j] - turn_score;
            }
            println!("Score of player {} for turn count {} is {} and total score is {}",player_names[j],turn_count,turn_score,player_total[j]);
            if player_total[j] > 99 { 
                println!("Player {} won on turn no {} with total score {}", player_names[j],turn_count,player_total[j]);
                winner_score = player_total[j];
                break ;
            } 
            let mut counter_to_kill = 0;
        while counter_to_kill < no_players.try_into().unwrap() {
           if counter_to_kill == j { 
               counter_to_kill +=1;
           } else if player_total[j] == player_total[counter_to_kill] {
               player_total[counter_to_kill] = 0;
               println!("Opppsss!!! Bad Luck Player {} struk by Player {}",player_names[counter_to_kill],player_names[j]);
           }
            counter_to_kill +=1;
        }
            j +=1;
            
        }
        
        turn_count +=1;
        
        let mut game_cont = String::new();
        println!("Press Enter for next dice roll");
        io::stdin().read_line(&mut game_cont).expect("please re enter name");
    }
    println!("Game End");
}
