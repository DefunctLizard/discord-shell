extern crate discord;

use std::process::Command;
use discord::model::Event;
use discord::Discord;
use std::str;

fn main() {
    // Log in to Discord using a bot token from the environment
    let discord =
        Discord::from_bot_token("DISCORD_TOKEN_HERE")
            .expect("login failed");

    // Establish and use a websocket connection
    let (mut connection, _) = discord.connect().expect("connect failed");
    println!("Ready.");
    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                if message.author.name == "Linux" {
                    continue;
                } else {
                    if message.content.chars().nth(0).unwrap() == '$' {
                        let command_text: String = message.content.chars().skip(2).collect();

		                let cmd = Command::new("sh")
                            .arg("-c")
                            .arg(&command_text)
                            .output()
                            .expect("failed to execute process");

		                let output = cmd.stdout;
		                let output = str::from_utf8(&output).unwrap();

                        let _ =
                            discord.send_message(message.channel_id, &format!("```{}```", output), "", false);
                    }
                }
            }
            Ok(_) => {}
            Err(discord::Error::Closed(code, body)) => {
                println!("Gateway closed on us with code {:?}: {}", code, body);
                break;
            }
            Err(err) => println!("Receive error: {:?}", err),
        }
    }
}
