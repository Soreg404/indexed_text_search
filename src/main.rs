use std::io::{stdin, stdout, Write};
use indexed_search_text::{TextsCollection, SearchError};

#[allow(unreachable_code)]
fn main() {

	playground();
	return;

	let texts_collection = TextsCollection::from([
		"There is a house in New Orleans",
		"Grind this to a fine powder",
		"Kill la Kill rocks!",
		"Litwo, ojczyzno moja, ty jesteś jak zdrowie",
		"My house stands on rocks!",
		"There, in the corner, stands a fine piece of armor.",
		"Gdzie dwóch się bije, tam pierwsze koty za płoty",
	]);

	/* TEST SEARCHING */
	let search = |cut| test_search_for_cut(cut, &texts_collection);

	search("There");
	search("New");
	search("house");
	search("rocks!");
	search("fine");
	search("turtle");
}

fn test_search_for_cut(
	cut: &str,
	in_collection: &TextsCollection,
) {
	let found = match in_collection.search(cut) {
		Ok(found) => found,
		Err(e) => {
			print!("Cut {cut:?} ");
			match e {
				SearchError::CutIsNotIndexed => println!("is not indexed"),
				SearchError::CutNotFound => println!("was not found"),
				SearchError::CutIsDangling => println!("is dangling"),
			}
			return;
		}
	};


	println!("Found texts ids for cut {:?}:", cut);
	for text_id in found.iter() {
		let text = in_collection.get_text_by_id(*text_id)
			.expect("text id should be in the index, otherwise somehow dangling");

		println!("- {:?}", text);
	}
}

fn playground() {
	println!("playground");

	let mut collection = TextsCollection::default();

	loop {
		print!(">");
		stdout().flush().unwrap();
		let mut line = String::new();
		stdin().read_line(&mut line).unwrap();

		let line = line.trim();

		if line.starts_with('+') {
			collection.store(&line[1..]);
			println!("stored.")
		} else {
			let found_texts = match collection.search(line) {
				Ok(v) => v,
				Err(e) => {
					println!("Not found: {e:?}");
					continue;
				}
			};

			println!("results:");
			for text_id in found_texts {
				let text = collection.get_text_by_id(*text_id).unwrap();

				println!("{:?}", text);
			}
			println!();
		}
	}
}
