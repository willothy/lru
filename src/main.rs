use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
};

#[derive(Debug, Clone)]
pub struct LRU<K: PartialEq + Eq + Hash + Clone, V> {
    capacity: usize,
    cache: HashMap<K, V>,
    queue: VecDeque<*const K>,
}

impl<K: PartialEq + Eq + Hash + Clone, V> LRU<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            cache: HashMap::with_capacity(capacity),
            queue: VecDeque::with_capacity(capacity),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        if self.cache.contains_key(&key) {
            self.cache.remove(&key);
        } else if self.queue.len() == self.capacity {
            let old_key = self.queue.pop_front().unwrap();
            self.cache.remove(unsafe { old_key.as_ref().unwrap() });
        }

        self.cache.insert(key.clone(), value);
        self.queue
            .push_back(self.cache.get_key_value(&key).unwrap().0 as *const K);
    }

    pub fn get(&mut self, key: K) -> Option<&V> {
        if self.cache.contains_key(&key) && self.queue.len() > self.capacity {
            let old_key = self.queue.pop_front().unwrap();
            self.cache.remove(unsafe { old_key.as_ref().unwrap() });
        }
        let ret = if let Some(val) = self.cache.get(&key) {
            self.queue
                .push_back(self.cache.get_key_value(&key).unwrap().0 as *const K);
            Some(val)
        } else {
            None
        };
        ret
    }
}

fn main() {
    let mut lru = LRU::new(3);

    lru.insert(1, 1);
    lru.insert(2, 2);
    lru.insert(3, 3);
    lru.insert(4, 4);
    lru.insert(7, 3);

    println!("{:?}", unsafe { **lru.queue.front().unwrap() });
    println!("{:?}", unsafe { **lru.queue.back().unwrap() });

    println!("{:?}", lru.get(7));
    println!("{:?}", lru.get(3));

    println!("{:?}", lru);
}
