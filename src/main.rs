use murmur3::murmur3_x64_128;
use std::{
    collections::hash_map::DefaultHasher,
    fmt::Display,
    hash::{Hash, Hasher},
    io::Cursor,
    marker::PhantomData,
};

/* */
struct BloomFilter<T: Hash> {
    elements: Vec<bool>,
    num_hash_functions: usize,
    phantom: PhantomData<T>,
}

#[derive(Debug)]
enum BloomFilterError {
    HashError
}
impl Display for BloomFilterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::HashError => f.write_str("Error While Hashing")
        }
    }
}
impl<T: Hash> BloomFilter<T> {
    pub fn new(size: usize, num_hash_fns: usize) -> Self {
        return BloomFilter::<T> {
            elements: vec![false; size],
            num_hash_functions: num_hash_fns,
            phantom: PhantomData,
        };
    }

    pub fn hash(&self, element: &T, index: usize) -> Result<usize, BloomFilterError> {
        let mut hasher = DefaultHasher::new();
        element.hash(&mut hasher);
        let hash = hasher.finish().to_be_bytes();
        let mut cursor = Cursor::new(hash);
        if let Ok(out) = murmur3_x64_128(&mut cursor, index as u32) {
            return Ok(out as usize);
        }
        return Err(BloomFilterError::HashError);
    }

    pub fn add_element(&mut self, el: T) -> Result<(), BloomFilterError> {
        for i in 0..self.num_hash_functions {
            let hash = self.hash(&el, i)?;
            let num_els = self.elements.len();
            self.elements[hash % num_els] = true;
        }
        return Ok(());
    }

    pub fn check_for(&self, el: T) -> Result<bool, BloomFilterError> {
        for i in 0..self.num_hash_functions {
            let hash = self.hash(&el, i)?;
            if !self.elements[hash % self.elements.len()] {
                return Ok(false);
            }
        }
        return Ok(true);
    }
}
fn main() {
    let mut blf: BloomFilter<i32> = BloomFilter::new(1000000, 3);
    for i in 0..1000 {
        match blf.add_element(i) {
            Ok(_) => println!("Added Item: {}", i),
            Err(err) => println!("Error: {}", err),
        }
    }
    println!("{:?}", blf.check_for(998).expect("Not hashable"));
    println!("{:?}", blf.check_for(9989).expect("Not hashable"));
}
