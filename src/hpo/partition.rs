//! This module implements a function to partition a list into sublists of a determined size.
//! 
//! In the following example, we extract three partitions [1,2,3], [4,5,6], and [7,8,9]
//! The final number 10 is ignored because we only want partitions of exactly the indicated size
//! The sentence mapper tries partiotions of sizes 1 to 14 to match HPO terms, and thus
//! the smaller "fragments" of any given partition will already have been tested
//! 
//! This is a private module.
//! 
//! # Examples
//! ```ignore
//! let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//! let partition = Partition::new(&list, 3);
//! for i in 0..partition.size() {
//!     let part = partition.get(i); // first parition will be an option 
//! }
//! ```
//! 


pub struct Partition<'a, T> {
    original_list: &'a [T],
    chunk_size: usize,
}

impl<'a, T> Partition<'a, T> {
    pub fn new(original: &'a [T], chunk_size: usize) -> Self {    
        Self {
            original_list: original,
            chunk_size: chunk_size,
        }
    }

    // Get the chunk at a given index
    pub fn get(&self, index: usize) -> Option<&'a [T]> {
       let start = index * self.chunk_size;
       if start > self.original_list.len() {
            return None;
       }
       let end = start + self.chunk_size;
       if end > self.original_list.len() {
        return None; // We do not want chunks that are smaller than chunk size
       }
       Some(&self.original_list[start..end])
    }

    /// Calculate the number of partitions of the list, equivalent to floor(list.size()/chunkSize)
    pub fn count(&self) -> usize {
        self.original_list.len() / self.chunk_size 
    }

}



#[cfg(test)]
mod test {
    use std::assert_eq;

    use super::*;

    #[test]
    fn test_integer_partition() {
        let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let partition = Partition::new(&list, 3);
        assert_eq!(3, partition.count());
        let p1 = partition.get(0);
        assert!(p1.is_some());
        let p1 = p1.unwrap();
        assert_eq!(vec![1,2,3], p1);
        let p2 = partition.get(1);
        assert!(p2.is_some());
        let p2 = p2.unwrap();
        assert_eq!(vec![4,5,6], p2);
    }


}

