use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::BufWriter,
};

#[derive(Parser, Debug)]
struct Args {
    action: String,
    amount: u16,
    effort: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Player {
    name: String,
    tickets: u32,
    a_pity: u8,
    b_pity: u8,
    total_pulls: u128,
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
    let mut player = Player {
        name: "Axol".to_string(),
        total_pulls: 0,
        tickets: 15,
        a_pity: 0,
        b_pity: 0,
    };
    match data {
        Ok(data) => {
            println!("Success!");
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
            println!("Added {} tickets to {}'s wallet!", args.amount, player.name);
            let _ = save_user(&player);
        }
        //"pull" => {
        //    machine::pull()
        //    println!("")
        //}
        _ => {
            println!("Unknown command!");
            panic!()
        }
    }
}
