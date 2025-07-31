use std::collections::{BTreeMap, BTreeSet};

#[derive(Default)]
pub struct IndexSingleStr {
	autoincrement: usize,
	id_owner: BTreeMap<usize, Box<str>>,
	map_cut: BTreeMap<Box<str>, usize>,
}

impl IndexSingleStr {
	pub fn insert(&mut self, s: &str) -> usize {
		if let Some(existing_id) = self.map_cut.get(s) {
			return *existing_id;
		}

		self.autoincrement += 1;
		self.id_owner.insert(self.autoincrement, Box::from(s));
		self.map_cut.insert(Box::from(s), self.autoincrement);

		self.autoincrement
	}
	pub fn get_id(&self, by_cut: &str) -> Option<usize> {
		self.map_cut.get(by_cut).map(|id| *id)
	}

	pub fn get_str(&self, by_id: usize) -> Option<&str> {
		self.id_owner.get(&by_id).map(|s| s.as_ref())
	}
}


fn main() {
	let mut index_main_saved_text = IndexSingleStr::default();

	let mut index_cuts = IndexSingleStr::default();

	let mut cut_to_texts_set_map =
		BTreeMap::<usize, BTreeSet<usize>>::new();
	let mut add_text_id_for_cut = |cut_id: usize, text_id: usize| {
		if let Some(cuts_set) = cut_to_texts_set_map.get_mut(&cut_id) {
			cuts_set.insert(text_id);
		} else {
			cut_to_texts_set_map.insert(cut_id, BTreeSet::from([text_id]));
		}
	};


	let sample_texts = [
		"There is a house in New Orleans",
		"Grind this to a fine powder",
		"Kill la Kill rocks!",
		"Litwo, ojczyzno moja, ty jesteś jak zdrowie",
		"My house stands on rocks!",
		"There, in the corner, stands a fine piece of armor.",
		"Gdzie dwóch się bije, tam pierwsze koty za płoty",
	];

	for text in sample_texts.iter() {
		let text_id = index_main_saved_text.insert(text);

		for word in text.split_whitespace() {
			add_text_id_for_cut(
				index_cuts.insert(word),
				text_id,
			);
		}
	}

	/* TEST SEARCHING */
	let test_search = |cut: &str| test_searching_for(
		cut,
		&index_main_saved_text,
		&index_cuts,
		&cut_to_texts_set_map,
	);

	test_search("There");
	test_search("New");
	test_search("house");
	test_search("rocks!");
	test_search("fine");
	test_search("turtle");
}


enum SearchError {
	CutIsNotIndexed,
	CutNotFound,
	CutIsDangling,
}

type SearchResult<'a> = Result<&'a BTreeSet<usize>, SearchError>;

fn test_searching_for(
	cut: &str,
	index_main_saved_text: &IndexSingleStr,
	index_cuts: &IndexSingleStr,
	cut_to_texts_set_map: &BTreeMap<usize, BTreeSet<usize>>,
) {
	let found = match search_for_texts_ids(
		cut,
		index_cuts,
		cut_to_texts_set_map,
	) {
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
		let text = index_main_saved_text.get_str(*text_id)
			.expect("text id should be in the index, otherwise somehow dangling");

		println!("- {:?}", text);
	}
}

fn search_for_texts_ids<'a>(
	by_cut: &str,
	index_cuts: &IndexSingleStr,
	cut_to_texts_set_map: &'a BTreeMap<usize, BTreeSet<usize>>,
) -> SearchResult<'a> {
	if let Some(cut_id) = index_cuts.get_id(by_cut) {
		match cut_to_texts_set_map.get(&cut_id) {
			None => Err(SearchError::CutNotFound),
			Some(results_set) => {
				if results_set.is_empty() {
					Err(SearchError::CutIsDangling)
				} else {
					Ok(results_set)
				}
			}
		}
	} else {
		Err(SearchError::CutIsNotIndexed)
	}
}
