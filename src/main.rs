extern crate serde_derive;

use std::io;
use std::io::Write;
use std::process;

mod blockchain;

fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    print!("Input a miner address: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut miner_addr)
        .expect("Failed to read line");

    print!("Input difficulty: ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut difficulty)
        .expect("Failed to read line");
    let diff = difficulty
        .trim()
        .parse::<u32>()
        .expect("we need an integer");

    println!("generating genesis block...");
    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), diff);

    loop {
        println!("Menu");
        println!("1) New Transaction");
        println!("2) Mine block");
        println!("3) Change Difficulty");
        println!("4) Change Reward");
        println!("0) Exit");
        print!("Enter your choice: ");
        io::stdout().flush().expect("Failed to flush stdout");
        choice.clear();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        println!("");

        match choice.trim().parse().unwrap() {
            0 => {
                println!("Exiting...");
                process::exit(0);
            }
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                print!("enter sender address:");
                io::stdout().flush().expect("failed to flush console");
                io::stdin().read_line(&mut sender).expect("failed to read address");
                print!("enter receiver address: ");
                io::stdout().flush().expect("failed to flush console");
                io::stdin().read_line(&mut receiver).expect("failed to read receiver");
                print!("Enter amount: ");
                io::stdout().flush().expect("failed to flush console");
                io::stdin().read_line(&mut amount).expect("failed to read amount");

                let res = chain.new_transaction(
                    sender.trim().to_string(),
                    receiver.trim().to_string(),
                    amount.trim().parse().unwrap(),
                );

                match res {
                    true => println!("transaction added"),
                    false => println!("transaction failed"),
                }
            }
            2 => {
                println!("generating new block");
                let res = chain.generate_new_block();
                match res {
                    true => println!("block added"),
                    false => println!("block failed"),
                }
            }
            3 => {
                let mut new_diff = String::new();
                print!("enter new difficulty");
                io::stdout().flush().expect("failed to flush console");
                io::stdin().read_line(&mut new_diff).expect("failed to read new difficulty");
                let res = chain.update_difficulty(new_diff.trim().parse().unwrap());
                match res { 
                    true => println!("difficulty updated"),
                    false => println!("difficulty failed"),
                }
            }
            4 => {
                let mut new_reward = String::new();
                print!("enter new reward");
                io::stdout().flush().expect("failed to flush console");
                io::stdin().read_line(&mut new_reward).expect("failed to read new reward");
                let res = chain.update_reward(new_reward.trim().parse().unwrap());
                match res { 
                    true => println!("reward updated"),
                    false => println!("reward failed"),
                }
            }
            _ => println!("\ninvalid choice"),

        }
    }
}
