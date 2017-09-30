#[macro_use]
extern crate nom;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str;
use std::str::FromStr;

use nom::{ IResult };

named!(node_field<&[u8], (String, String)>, 
		do_parse!(
			key: alt!(tag!("id")) >>
			value: take_until!("\n") >>
			((String::from_utf8_lossy(key).into_owned(), String::from_utf8_lossy(value).into_owned()))
		)
);

named!(node<&[u8], Vec<(String, String)>>, 
	ws!(
		do_parse!(
			tag!("node") >>
			tag!("[") >>
			node_fields: many1!(node_field) >>
			tag!("]") >>
			(node_fields)
		)
	)
);

fn main() {
	let path = Path::new("data/power.gml");
	let display = path.display();

	let mut file = match File::open(&path) {
		Err(why) => panic!("couldn't open {}: {}", display, why.description()),
		Ok(file) => file,
	};

	let mut s = String::new();
	file.read_to_string(&mut s).unwrap();

	let test = "node 
	[
		id 0
	]";

	let thing = node(&test.as_bytes());

	match thing {
		IResult::Done(remaining, parsed) => {
			println!("{:?}", parsed);

			println!("Remaining bytes: {:?}", remaining.len());
		},
		_ => println!("Failed to parse")
	}
}
