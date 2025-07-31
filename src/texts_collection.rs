use std::collections::BTreeSet;
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
		self.index_of_cuts.contains_id(0);
		if self.index_of_texts.contains_text(text) {
			return;
		}
		self.store_assign(text, &[])
	}
	pub fn store_assign(&mut self, text: &str, cuts: &[&str]) {
		let text_id = self.index_of_texts.insert(text);

		// todo: properly calculate cuts and weights
		let calculated_cuts = text.split_whitespace();

		for cut in calculated_cuts {
			let cut_id = self.index_of_cuts.insert(cut);
			self.index_cut_to_texts.associate(cut_id, text_id);
		}

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
