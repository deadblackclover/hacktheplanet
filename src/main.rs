extern crate reqwest;

use std::collections::HashMap;
use std::time::Duration;
use std::thread;

fn main() {
	loop {
		let mut map = HashMap::new();
		map.insert("status", "Hack the planet!");

		let client = reqwest::Client::new();
		let _res = client.post("https://botsin.space/api/v1/statuses")
			.bearer_auth("60435da8498eb3a3780ab1a8a758648a780e2449e96372baf35e009296858558")
			.json(&map)
			.send()
			.map_err(|err| println!("request error: {}", err))
			.map(|mut body| {
				println!("Status:{:?}", body.status());
				println!("{:?}", body.text());
			});

    	thread::sleep(Duration::from_secs(60 * 60))	
	}
}