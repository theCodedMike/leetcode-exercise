//Design a data structure that follows the constraints of a Least Recently Used
//(LRU) cache.
//
// Implement the LRUCache class:
//
//
// LRUCache(int capacity) Initialize the LRU cache with positive size capacity.
//
// int get(int key) Return the value of the key if the key exists, otherwise
//return -1.
// void put(int key, int value) Update the value of the key if the key exists.
//Otherwise, add the key-value pair to the cache. If the number of keys exceeds
//the capacity from this operation, evict the least recently used key.
//
//
// The functions get and put must each run in O(1) average time complexity.
//
//
// Example 1:
//
//
//Input
//["LRUCache", "put", "put", "get", "put", "get", "put", "get", "get", "get"]
//[[2], [1, 1], [2, 2], [1], [3, 3], [2], [4, 4], [1], [3], [4]]
//Output
//[null, null, null, 1, null, -1, null, -1, 3, 4]
//
//Explanation
//LRUCache lRUCache = new LRUCache(2);
//lRUCache.put(1, 1); // cache is {1=1}
//lRUCache.put(2, 2); // cache is {1=1, 2=2}
//lRUCache.get(1);    // return 1
//lRUCache.put(3, 3); // LRU key was 2, evicts key 2, cache is {1=1, 3=3}
//lRUCache.get(2);    // returns -1 (not found)
//lRUCache.put(4, 4); // LRU key was 1, evicts key 1, cache is {4=4, 3=3}
//lRUCache.get(1);    // return -1 (not found)
//lRUCache.get(3);    // return 3
//lRUCache.get(4);    // return 4
//
//
//
// Constraints:
//
//
// 1 <= capacity <= 3000
// 0 <= key <= 10⁴
// 0 <= value <= 10⁵
// At most 2 * 10⁵ calls will be made to get and put.
//
//
// Related Topics Hash Table Linked List Design Doubly-Linked List 👍 19196 👎 8
//55

#![allow(dead_code)]

pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashMap;

pub struct LRUCache {
    capacity: usize,
    data: HashMap<i32, (i32, i32, u128)>, // key -> (key, val, time)
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        let capacity = capacity as usize;
        LRUCache {
            capacity,
            data: HashMap::with_capacity(capacity),
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        self.data.get_mut(&key).map_or(-1, |v| {
            v.2 = Self::get_time();
            v.1
        })
    }

    pub fn put(&mut self, key: i32, value: i32) {
        let len = self.data.len();
        if self.data.contains_key(&key) {
            self.data.get_mut(&key).map(|v| {
                v.1 = value;
                v.2 = Self::get_time();
            });
        } else {
            if self.capacity == len {
                let option = self.data.values().min_by(|&&x, &&y| x.2.cmp(&y.2));
                match option {
                    None => {
                        panic!("not find key")
                    }
                    Some(&val) => {
                        self.data.remove(&val.0);
                    }
                }
            }
            self.data.insert(key, (key, value, Self::get_time()));
        }
    }

    fn get_time() -> u128 {
        let now = std::time::SystemTime::now();
        now.duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    }
}

//
// Your LRUCache object will be instantiated and called as such:
// let obj = LRUCache::new(capacity);
// let ret_1: i32 = obj.get(key);
// obj.put(key, value);
//
//leetcode submit region end(Prohibit modification and deletion)
