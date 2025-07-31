use std::collections::{BTreeMap, BTreeSet};

#[derive(Default)]
pub struct IndexManyToMany {
	primary: BTreeMap<usize, BTreeSet<usize>>,
	reverse: BTreeMap<usize, BTreeSet<usize>>
}

impl IndexManyToMany {
	pub fn associate(&mut self, primary_id: usize, secondary_id: usize) {
		if let Some(set_primary) = self.primary.get_mut(&primary_id) {
			set_primary.insert(secondary_id);
		} else {
			self.primary.insert(primary_id, BTreeSet::from([secondary_id]));
		}

		if let Some(set_reverse) = self.reverse.get_mut(&secondary_id) {
			set_reverse.insert(primary_id);
		} else {
			self.reverse.insert(secondary_id, BTreeSet::from([primary_id]));
		}
	}

	pub fn get_set(&self, primary_id: usize) -> Option<&BTreeSet<usize>> {
		self.primary.get(&primary_id)
	}
}
