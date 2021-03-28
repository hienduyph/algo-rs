use std::collections::HashMap;
use arrayvec::ArrayVec;

pub struct LRUCache<K, V, const N: usize> {
    data: HashMap<K, Node<K, V>>,
    order: ArrayVec<K, N>,
    head: u16,
    tail: u16,
}

pub struct Node<K, V> {
    key: K,
    data: V,
    next: u16,
    prev: u16,
}

impl<K, V, const N: usize> LRUCache<K, V, N> {
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&mut self, key: K) -> Option<V> {
        panic!("impl")
    }

    pub fn set(&mut self, key: K, val: V) {
        panic!("impl")
    }

    fn move_to_head(&mut self, key: K) {
        panic!("impl")
    }
}
