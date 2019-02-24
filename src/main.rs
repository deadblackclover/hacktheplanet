extern crate reqwest;

use std::collections::HashMap;
use std::time::Duration;
use std::thread;
use std::env;

fn main() {

	let mut token: String = String::new();

	match env::var("BOT_TOKEN") {
		Ok(t) => token=t,
		Err(e) => println!("Error={:?}", e),
	}

	loop {
		let mut map = HashMap::new();
		map.insert("status", "Hack the planet!");

		let client = reqwest::Client::new();
		let _res = client.post("https://botsin.space/api/v1/statuses")
			.bearer_auth(&token)
			.json(&map)
			.send()
			.map_err(|err| println!("request error: {}", err))
			.map(|mut body| {
				let status = body.status();
				if status == 200 {
					println!("Status send");
					println!("Status code:{:?}", body.status());
				} else {
					println!("Error send");
					println!("Status code:{:?}", body.status());
					println!("{:?}", body.text());
				}
			});

		thread::sleep(Duration::from_secs(60 * 60))	
	}
}
