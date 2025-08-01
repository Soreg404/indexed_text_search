use std::collections::BTreeSet;
use std::ops::Sub;
use crate::index_combined::IndexManyToMany;
use crate::index_single_str::IndexSingleStr;

#[derive(Default)]
pub struct TextsCollection {
	index_of_texts: IndexSingleStr,
	index_of_cuts: IndexSingleStr,
	index_cut_to_texts: IndexManyToMany,
}

#[derive(Debug)]
pub enum SearchError {
	CutIsNotIndexed,
	CutNotFound,
	CutIsDangling,
}

// pub struct IdHandle(usize);
// pub type SearchResult<'a> = Result<&'a BTreeSet<IdHandle>, SearchError>;

pub type SearchResult<'a> = Result<&'a BTreeSet<usize>, SearchError>;


impl TextsCollection {
	pub fn store(&mut self, text: &str) {
		let text_id =
			if !self.index_of_texts.contains_text(text) {
				self.index_of_texts.insert(text)
			} else {
				return
			};

		println!("calculating cuts...");
		let text_chars: Vec<char> = text.chars().collect();
		let mut cuts = Vec::<Box<str>>::new();

		let mut it_first = text_chars.iter().enumerate();
		while let Some((i, _c)) = it_first.next() {
			let mut it_second = it_first.clone();
			let mut j = i;
			loop {
				let cut = String::from_iter(text_chars[i..=j].iter());
				// print!("{cut:?},");
				cuts.push(Box::from(cut.as_str()));

				if let Some((idx, _c)) = it_second.next() {
					j = idx;
				} else {
					break;
				}

				if j - i == 7 {
					break;
				}
			}
		}
		println!("done calculating cuts");

		let len_before = self.index_of_cuts.len();
		let cuts_len = cuts.len();

		for cut in cuts {
			let cut_id = self.index_of_cuts.insert(cut.as_ref());
			self.index_cut_to_texts.associate(cut_id, text_id);
		}

		println!("n={}, total={}, added={}", cuts_len, self.index_of_cuts.len(),
				 self.index_of_cuts.len() - len_before);

		// println!("{:?}", &self.index_of_cuts);
	}
	pub fn store_assign(&mut self, text: &str, cuts: &[&str]) {
		let text_id = self.index_of_texts.insert(text);

		for cut in cuts {
			let cut_id = self.index_of_cuts.insert(cut);
			self.index_cut_to_texts.associate(cut_id, text_id);
		}
	}

	// todo: maybe lowercase?
	pub fn search(&self, cut: &str) -> SearchResult {
		if let Some(cut_id) = self.index_of_cuts.get_id(cut) {
			match self.index_cut_to_texts.get_set(cut_id) {
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

	pub fn get_text_by_id(&self, text_id: usize) -> Option<&str> {
		self.index_of_texts.get_text(text_id)
	}
}

impl From<&[&str]> for TextsCollection {
	fn from(src: &[&str]) -> Self {
		let mut tmp_self = Self::default();
		for text in src.iter() {
			tmp_self.store(text);
		}
		tmp_self
	}
}

impl<const N: usize> From<[&str; N]> for TextsCollection {
	fn from(value: [&str; N]) -> Self {
		Self::from(value.as_slice())
	}
}
