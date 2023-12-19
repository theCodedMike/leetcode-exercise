use leetcode_exercise::leetcode::editor::en::_146_l_r_u_cache::LRUCache;

#[test]
fn lru_cache() {
    let mut cache = LRUCache::new(2);
    cache.put(1, 1);
    cache.put(2, 2);
    assert_eq!(cache.get(1), 1);
    cache.put(3, 3);
    assert_eq!(cache.get(2), -1);
    cache.put(4, 4);
    assert_eq!(cache.get(1), -1);
    assert_eq!(cache.get(3), 3);
    assert_eq!(cache.get(4), 4);
}

#[test]
fn lru_cache2() {
    let mut cache = LRUCache::new(2);
    assert_eq!(cache.get(2), -1);
    cache.put(2, 6);
    assert_eq!(cache.get(1), -1);
    cache.put(1, 5);
    cache.put(1, 2);
    assert_eq!(cache.get(1), 2);
    assert_eq!(cache.get(2), 6);
}
