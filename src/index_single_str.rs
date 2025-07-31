use std::collections::BTreeMap;

#[derive(Default)]
pub struct IndexSingleStr {
	autoincrement: usize,
	primary: BTreeMap<usize, Box<str>>,
	reverse: BTreeMap<Box<str>, usize>,
}

impl IndexSingleStr {
	pub fn insert(&mut self, text: &str) -> usize {
		if let Some(existing_id) = self.reverse.get(text) {
			return *existing_id;
		}

		self.autoincrement += 1;
		self.primary.insert(self.autoincrement, Box::from(text));
		self.reverse.insert(Box::from(text), self.autoincrement);

		self.autoincrement
	}
	pub fn get_id(&self, by_text: &str) -> Option<usize> {
		self.reverse.get(by_text).map(|id| *id)
	}
	pub fn get_text(&self, by_id: usize) -> Option<&str> {
		self.primary.get(&by_id).map(|s| s.as_ref())
	}
}

#[allow(unused)]
impl IndexSingleStr {
	pub fn contains_id(&self, id: usize) -> bool {
		self.primary.contains_key(&id)
	}
	pub fn contains_text(&self, text: &str) -> bool {
		self.reverse.contains_key(text)
	}
}
