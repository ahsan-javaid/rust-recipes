trait Sorter {
    fn sort <T> (slice: &mut [T]) where T: Ord;  
}

struct Bubblesort {
}

impl Sorter for Bubblesort {
    fn sort <T> (slice: &mut [T]) where T: Ord {
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 0..(slice.len() - 1) {
                if slice[i] > slice[i+1] {
                    slice.swap(i, i + 1);
                    swapped = true;
                }
            }
        }
    }
}

struct StdSorter;
impl Sorter for StdSorter {
    fn sort <T> (slice: &mut [T]) where T: Ord {
        slice.sort();
    }

}

fn sort<T, S>(slice: &mut [T]) where T: Ord, S: Sorter {
    S::sort(slice)
}




fn main() {
    let mut arr = vec![4,2,3,5,2,1];

   // sort::<_, StdSorter>(&mut arr); 
    sort::<_, Bubblesort>(&mut arr); 

    println!("{:?}", arr);

}
