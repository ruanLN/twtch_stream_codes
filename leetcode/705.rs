#[derive(Debug)]
struct MyHashSet {
    hash_table: Vec<Vec<i32>>
}

const BASE_PRIME: i32 = 199;

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {
    fn hash(key: i32) -> i32 {
        key % BASE_PRIME
    }
    
    /** Initialize your data structure here. */
    fn new() -> Self {
        MyHashSet {
            hash_table: vec![Vec::new(); BASE_PRIME as usize]
        }
    }
    
    fn add(&mut self, key: i32) {
        let hashed = Self::hash(key);
        if !self.hash_table[hashed as usize].contains(&key) {
            self.hash_table[hashed as usize].push(key);
        }
    }
    
    fn remove(&mut self, key: i32) {
        let hashed = Self::hash(key);
        let position = self.hash_table[hashed as usize]
            .iter()
            .position(|&x| x == key);
        if let Some(index) = position {
            self.hash_table[hashed as usize].remove(index);
        }
    }

    /** Returns true if this set contains the specified element */
    fn contains(&self, key: i32) -> bool {
        let hashed = Self::hash(key);
        self.hash_table[hashed as usize].contains(&key)
    }
}

/**
 * Your MyHashSet object will be instantiated and called as such:
 * let obj = MyHashSet::new();
 * obj.add(key);
 * obj.remove(key);
 * let ret_3: bool = obj.contains(key);
 */