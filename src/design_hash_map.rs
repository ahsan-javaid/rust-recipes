
struct MyHashSet {
    inner: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {

    fn new() -> Self {
        Self {
            inner: Vec::new()
        }
    }
    
    fn add(&mut self, key: i32) {
        match self.inner.binary_search(&key) {
            Err(pos) => {
                self.inner.insert(pos, key);
            },
            _ => {}
        }
    }
    
    fn remove(&mut self, key: i32) {
        match self.inner.binary_search(&key) {
            Ok(pos) => {
                self.inner.remove(pos);
            },
            _ => {}
        }
    }
    
    fn contains(&self, key: i32) -> bool {
        self.inner.binary_search(&key).is_ok()
    }
}
fn main() {
  let key: i32 = 5;
  let mut  obj = MyHashSet::new();
  obj.add(key);
  obj.remove(key);
  let ret_3: bool = obj.contains(key);
}
