mod machine;

use clap::Parser;
use machine::Rarities;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::BufWriter,
};

#[derive(Parser, Debug)]
struct Args {
    action: String,
    amount: usize,
    effort: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Player {
    name: String,
    tickets: u32,
    a_pity: u8,
    b_pity: u8,
    has_slip: bool,
    accumulated_increase: f64,

    total_pulls: u128,
}
impl Player {
    fn get_player_ref(&self) -> &Player {
        return self;
    }
    fn new(name: String) -> Self {
        Self {
            name,
            tickets: 0,
            a_pity: 0,
            b_pity: 0,
            has_slip: false,
            accumulated_increase: 0.0,
            total_pulls: 0,
        }
    }
}

struct JsonMembers {
    members: Vec<Player>,
}
fn save_user(player: &Player) -> std::io::Result<()> {
    let file = File::create("./user_data.json").unwrap();
    let mut writer = BufWriter::new(file);

    serde_json::to_writer_pretty(&mut writer, player)?;
    Ok(())
}

fn main() {
    let data = fs::read_to_string("./user_data.json");
    let mut player = Player::new("axol".to_string());
    let mut wish = Vec::new();

    match data {
        Ok(data) => {
            if !data.is_empty() {
                player = serde_json::from_str(&data).unwrap();
            }
        }
        Err(_) => {
            let _ = File::create("./user_data.json").unwrap();
        }
    }

    let args = Args::parse();
    match args.action.as_str() {
        "add" => {
            player.tickets = player.tickets + args.amount as u32;
            println!("Added {} tickets to {}'s wallet!", args.amount, player.name);
        }
        "pull" => {
            wish = match args.amount {
                1 => Some(machine::handle_pull(&mut player)),
                10 => Some(machine::handle_pull_ten(&mut player)),
                _ => {
                    println!("Invalid pull count");
                    None
                }
            }
        }
        _ => {
            println!("Unknown command!");
            panic!()
        }
    }
    let _ = save_user(&player);
}
