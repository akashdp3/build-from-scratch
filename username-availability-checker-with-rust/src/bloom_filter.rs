use std::hash::{BuildHasher, DefaultHasher, Hash, Hasher, RandomState};

pub struct BloomFilter {
    bitmap: Vec<bool>,
    hashers: [DefaultHasher; 2],
    pub size: usize,
}

impl BloomFilter {
    pub fn new(size: usize) -> Self {
        Self {
            bitmap: vec![false; size],
            hashers: [
                RandomState::new().build_hasher(),
                RandomState::new().build_hasher(),
            ],
            size: size,
        }
    }

    fn hash<T: Hash + ?Sized>(&self, hash_index: usize, item: &T) -> usize {
        let mut hasher = self.hashers[hash_index].clone();

        item.hash(&mut hasher);

        hasher.finish() as usize
    }

    fn get_index(&self, hash_value: usize) -> usize {
        hash_value % self.size
    }

    pub fn clear(&mut self) {
        self.bitmap.fill(false);
    }

    pub fn insert<T: Hash + ?Sized>(&mut self, item: &T) {
        for i in 0..self.hashers.len() {
            let hash_value = self.hash(i, item);

            let hash_index = self.get_index(hash_value);

            self.bitmap[hash_index] = true;
        }
    }

    pub fn contains<T: Hash + ?Sized>(&self, item: &T) -> bool {
        let h1 = self.get_index(self.hash(0, item));
        let h2 = self.get_index(self.hash(1, item));

        if self.bitmap.get(h1).unwrap() & self.bitmap.get(h2).unwrap() {
            return true;
        } else {
            return false;
        }
    }
}
