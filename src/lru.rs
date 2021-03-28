use arrayvec::ArrayVec;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};

pub struct LruCache<K: Eq + Hash + Copy, V, const N: usize> {
    data: HashMap<K, Node<K, V>>,
    order: ArrayVec<K, N>,
    head: usize,
    tail: usize,
}

pub struct Node<K, V> {
    key: K,
    data: V,
    next: usize,
    prev: usize,
}

impl<K, V> Node<K, V> {
    fn new(key: K, val: V) -> Self {
        Self {
            key,
            data: val,
            next: 0,
            prev: 0,
        }
    }
}

impl<K: Eq + Hash + Copy, V, const N: usize> LruCache<K, V, N> {
    pub fn len(&self) -> usize {
        self.order.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&mut self, key: K) -> Option<&V> {
        if let Some(mut node) = self.data.get_mut(&key) {
            LruCache::<K, V, N>::move_to_head(&mut self.order, &mut node);
        }

        self.data.get(&key).map(|v| &v.data)
    }

    pub fn set(&mut self, key: K, val: V) {
        if let Some(mut node) = self.data.get_mut(&key) {
            LruCache::<K, V, N>::move_to_head(&mut self.order, &mut node);
            node.data = val;
            return;
        }

        // cache evection
        if self.order.is_full() {
            // drop tail
            let tail_key = self.order.remove(self.tail);
            let node = self.data.remove(&tail_key).unwrap();
            self.tail = node.prev;
        }

        let node = Node::new(key, val);
        self.data.insert(key, node);
        self.order.push(key);
    }

    fn move_to_head(order: &mut ArrayVec<K, N>, key: &mut Node<K, V>) {
        panic!("impl")
    }
}
