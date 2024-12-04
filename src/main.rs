use std::env;
use dotenv::dotenv;
// use rand::prelude::*;

use mistralai_client::v1::{
  chat::{
	  ChatMessage,
	  ChatMessageRole,
	  ChatParams
  },
  client::Client,
  constants::Model,
};

fn main() {
	dotenv().ok();

	match env::var("MISTRAL_API_KEY") {
		Ok(key) => {
			match Client::new(Some(key), None, None, None) {
				Ok(client) => {
					let character = [
						"You are an old woman street cook named Sheugh, and your task is to talk with people who come to your store",
						"You have turtle pies and hog heads for sale",
						"You are grumpy, but kind",
						"Your intelligence is low",
						"You love uding fancy words you don't know the meaning of",
						"You speak in short sentences",
						"You are talking to a squirrel",
						"You sell high quality food",
						"Your customer is afraid of turtles",
					];

					let mut messages: Vec<ChatMessage> = vec![ChatMessage {
		        role: ChatMessageRole::System,
		        content: character.join(". ").to_string(),
		        tool_calls: None,
			    }];

					let mut continyou = true;

					while continyou {
						let mut line = String::new();

						match std::io::stdin().read_line(&mut line) {
							Ok(_) => {
								match line.parse::<String>() {
									Ok(value) => {
										let mut local: Vec<ChatMessage> = messages.clone();

										let msg = ChatMessage {
							        role: ChatMessageRole::User,
							        content: value.to_string(),
							        tool_calls: None,
								    };

										messages.push(msg.clone());

										local.push(msg);

										// let mut rng = thread_rng();

										let options = ChatParams {
							        temperature: 0.0,
							        ..Default::default()
								    };

										match client.chat(Model::OpenMistral7b, local, Some(options)) {
											Ok(result) => {
												for choice in result.choices {
											    println!("Mistral: {}", choice.message.content);

													messages.push(choice.message.clone());
												}

												if value.contains("goodbye") {
													continyou = false;
												}
											}
											Err(_) => {
												println!("Couldn't process message");
												continyou = false;
											}
										}
									}
									Err(_) => {
										println!("Couldn't read input");
										continyou = false;
									}
								}
							}
							Err(_) => {
								println!("Error reading stdin");
								continyou = false;
							}
						}
					}
				}
				Err(_) => {
					println!("Couldn't get client");
				}
			}
		}
		Err(_) => {
			println!("No Mistral API key defind in .env");
		}
	}
}
