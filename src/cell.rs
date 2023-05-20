use std::cell::UnsafeCell;

pub struct Cell <T> {
    value : UnsafeCell<T>
}
 
 // implied by unsafe cell
unsafe impl<T> Sync for Cell<T> {}

impl<T> Cell<T> {

    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value)
        }
    }

    pub fn set(&self, value: T) {
        unsafe { *self.value.get()  = value };
    }

    pub fn get(&self) -> &T where T: Copy {
         unsafe {
            &*self.value.get()
         }
    }
}

fn main() {

}

#[cfg(test)]
mod tests {
  use super::Cell;
  use std::sync::Arc;
  use std::thread;

  #[test]
  fn bad() {
    let x = Arc::new (Cell::new([0; 10240]));
    let x1 = Arc::clone(&x);

    let j1 = thread::spawn(move || {
        x1.set([1; 10240]);
    });

    let x2 = Arc::clone(&x);

    let j2 = thread::spawn(move || {
        x2.set([2; 10240]);
    });

    j1.join().unwrap();
    j2.join().unwrap();
    for i in x.get().iter() {
        println!("{}", i);
    }
  }

  #[test]
  fn bad2() {
    let x = Cell::new(String::from("hello"));

    let first = x.get();
    x.set(String::from("h"));

    println!("{}", x);
  }
}
