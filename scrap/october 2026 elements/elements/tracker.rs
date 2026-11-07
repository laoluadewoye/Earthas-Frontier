use std::collections::{HashSet, HashMap};

#[derive(Debug, Clone)]
pub struct EFItemTracker<T: Clone> {
    items: Vec<T>,
    tombstones: HashSet<usize>
}

impl<T: Clone> EFItemTracker<T> {
    pub fn new() -> Self {
        EFItemTracker { items: Vec::new(), tombstones: HashSet::new() }
    }

    pub fn build(items: Vec<T>, tombstones: HashSet<usize>) -> Self {
        EFItemTracker { items, tombstones }
    }

    pub fn get_length(&self) -> usize {
        self.items.len() - self.tombstones.len()
    }

    pub fn get_item(&self, item_index: usize) -> Option<&T> {
        // Check if there's an out of bounds possibility
        if item_index >= self.items.len() {
            return None;
        }

        // Check if the index is in the tombstone
        if self.tombstones.contains(&item_index) {
            return None;
        }

        // Retrieve item
        Some(&self.items[item_index])
    }

    pub fn get_multiple_items(&self, item_indexes: &Vec<usize>) -> Vec<Option<&T>> {
        let mut gotten_items: Vec<Option<&T>> = Vec::new();

        for item_index in item_indexes {
            gotten_items.push(self.get_item(item_index.clone()))
        }

        gotten_items
    }

    pub fn get_all_items(&self) -> Vec<&T> {
        let mut gotten_items: Vec<&T> = Vec::new();

        for (item_index, item) in self.items.iter().enumerate() {
            if !self.tombstones.contains(&item_index) {
                gotten_items.push(item);
            }
        }

        gotten_items
    }

    pub fn push_item(&mut self, new_item: T) -> usize {
        // Use a dead index if there is one, otherwise just add to end
        if !self.tombstones.is_empty() {
            let new_item_index: usize = self.tombstones.iter().next().unwrap().clone();
            self.tombstones.remove(&new_item_index);
            self.items.insert(new_item_index, new_item);
            new_item_index
        }
        else {
            let new_item_index: usize = self.items.len();
            self.items.push(new_item);
            new_item_index
        }
    }

    pub fn push_multiple_items(&mut self, new_items: &Vec<T>) -> Vec<usize> {
        let mut indexes: Vec<usize> = Vec::new();

        for new_item in new_items {
            indexes.push(self.push_item(new_item.clone()));
        }

        indexes
    }

    pub fn pop_item(&mut self, item_index: usize) -> Option<T> {
        // Check if there's an out of bounds possibility
        if item_index >= self.items.len() {
            return None;
        }

        // Check if the index is in the tombstone
        if self.tombstones.contains(&item_index) {
            return None;
        }

        // Add tombstone entry
        self.tombstones.insert(item_index);

        // Retrieve item
        Some(self.items[item_index].clone())
    }

    pub fn pop_multiple_items(&mut self, item_indexes: &Vec<usize>) -> Vec<Option<T>> {
        let mut popped_items: Vec<Option<T>> = Vec::new();

        for item_index in item_indexes {
            popped_items.push(self.pop_item(item_index.clone()));
        }

        popped_items
    }

    pub fn pop_all_items(&mut self) -> Vec<T> {
        // Pop items into new vector
        let mut popped_items: Vec<T> = Vec::new();

        for (item_index, item) in self.items.iter().enumerate() {
            if !self.tombstones.contains(&item_index) {
                popped_items.push(item.clone());
            }
        }

        // Reset attributes
        self.reset_items();

        // Return vector
        popped_items
    }

    pub fn compact_items(&mut self) -> HashMap<usize, usize> {
        // Prepare expensive operation
        let mut compaction_diff: usize = 0;
        let mut new_items: Vec<T> = Vec::new();
        let mut translation_map: HashMap<usize, usize> = HashMap::new();

        // Create the new clean items
        for item_index in 0..self.items.len() {
            if self.tombstones.contains(&item_index) {
                compaction_diff = compaction_diff + 1;
            }
            else {
                new_items.push(self.items[item_index].clone());
                translation_map.insert(item_index, item_index - compaction_diff);
            }
        }

        // Change the tracker vector
        self.items = new_items;

        // Empty out the tombstones
        self.tombstones = HashSet::new();

        // Return the translation map
        translation_map
    }

    pub fn reset_items(&mut self) {
        self.items = Vec::new();
        self.tombstones = HashSet::new();
    }
}
