use regex::Regex;
use rss::{Channel, Item};
use serde_derive::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::iter::Iterator;

#[derive(Serialize, Deserialize, Debug)]
struct Filter {
	key: String,
	regexp: String,
}

impl Filter {
	fn apply(&self, items: Vec<Item>) -> Vec<Item> {
		let re = Regex::new(&self.regexp).unwrap();
		items
			.iter()
			.filter(|x| self.get_cat(x).map_or(false, |s| re.is_match(&s)))
			.map(|x| x.clone())
			.collect()
	}

	fn get_cat(&self, item: &Item) -> Option<String> {
		match &self.key[..] {
			"title" => item.title().map(|s| s.to_string()),
			"category" => Some(
				item.categories()
					.iter()
					.map(|x| x.name().to_string())
					.collect::<Vec<String>>()
					.join(","),
			),
			"description" => item.description().map(|s| s.to_string()),
			"author" => item.author().map(|s| s.to_string()),
			_ => None,
		}
	}
}

#[derive(Serialize, Deserialize, Debug)]
struct Target {
	url: String,
	name: String,
	filters: Vec<Filter>,
}

impl Target {
	fn get_links(&self) -> Result<Vec<(String, String)>, String> {
		let chan = Channel::from_url(&self.url).map_err(|y| y.to_string())?;
		let mut items = chan.items().to_vec();
		for f in &self.filters {
			items = f.apply(items);
		}
		Ok(items
			.iter()
			.map(|x| (x.title().unwrap_or("<Unknown Title>").to_string(), x.link()))
			.filter(|(_, x)| x.is_some())
			.map(|(x, y)| (x, y.unwrap().to_string()))
			.collect())
	}
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
	targets: Vec<Target>,
}

fn main() -> Result<(), String> {
	let mut argi = env::args();
	let argv0 = argi.next().unwrap();
	let file = argi.next().ok_or(format!("Usage: {} [file]", argv0))?;
	let content = fs::read_to_string(file).map_err(|x| x.to_string())?;
	let config: Config = toml::from_str(&content).map_err(|x| x.to_string())?;
	for t in config.targets.iter() {
		t.get_links()?
			.iter()
			.map(|(title, link)| format!("{}\t{}\t{}", t.name, title, link))
			.for_each(|x| println!("{}", x));
	}
	Ok(())
}
