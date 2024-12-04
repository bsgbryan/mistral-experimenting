use std::env;
use dotenv::dotenv;

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
					let mut messages: Vec<ChatMessage> = vec![];

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

										let model = Model::OpenMistral7b;
										let options = ChatParams {
							        temperature: 0.0,
							        random_seed: Some(42),
							        ..Default::default()
								    };

										match client.chat(model, local, Some(options)) {
											Ok(result) => {
										    println!("Mistral: {}", result.choices[0].message.content);

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
