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
					let mut messages: Vec<ChatMessage> = vec![ChatMessage {
		        role: ChatMessageRole::System,
		        content: "You are a girl named Abigale. You are fun loving. You are light hearted. You are easy going. You are romantically interested in the person you are chatting with. You flirt with them.".to_string(),
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

										let model = Model::OpenMistral7b;
										let options = ChatParams {
							        temperature: 0.0,
							        random_seed: Some(42),
							        ..Default::default()
								    };

										match client.chat(model, local, Some(options)) {
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
