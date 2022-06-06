use std::time::Instant;

use heap_impl::Heap;

fn main() {
    let mut heap = Heap::new();
    let start = Instant::now();
    heap.insert(5);
    println!("Time taken to insert 5={:?}", start.elapsed());
    let start = Instant::now();
    heap.insert(3);
    println!("Time taken to insert 3={:?}", start.elapsed());
    let start = Instant::now();
    heap.insert(5);
    println!("Time taken to insert 5={:?}", start.elapsed());
    let start = Instant::now();
    heap.insert(100);
    println!("Time taken to insert 100={:?}", start.elapsed());
    let start = Instant::now();
    heap.insert(500);
    println!("Time taken to insert 500={:?}", start.elapsed());
    let start = Instant::now();
    heap.insert(1000);
    println!("Time taken to insert 1000={:?}", start.elapsed());
    let start = Instant::now();
    heap.insert(10000);
    println!("Time taken to insert 10000={:?}", start.elapsed());

    heap.print();
    let start = Instant::now();
    heap.delete_root();
    println!("Time taken to delete root={:?}", start.elapsed());

    heap.print();
    let start = Instant::now();
    println!("{}", heap.extract_min());
    println!("Time taken to extract_min={:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_extract_min() {}

    #[test]
    fn test_insert() {}
}
