use std::{cmp::min, vec::Vec};

pub struct Partition<T> 
    where T: Clone
    {
    chunk_list: Vec<Vec<T>>,
    chunk_size: usize,
}

impl<T> Partition<T>
    where T: Clone {
    
    pub fn new(token_list: Vec<T>, chunk_size: usize) -> Self {
        let mut chunks: Vec<Vec<T>> = Vec::new();
        let n_chunks =    (token_list.len() as f64 / chunk_size as f64).ceil() as usize;
        for i in 0..=n_chunks {
            let end = min(token_list.len(), i * chunk_size + chunk_size);
            chunks.push(token_list[i..end].to_vec());
        }
        Partition {
            chunk_list: chunks,
            chunk_size,
        }
    }

    // Static method to create a Partition
    pub fn of_size(list: Vec<T>, chunk_size: usize) -> Self {
        Partition::new(list, chunk_size)
    }

    // Get the chunk at a given index
    pub fn get(&self, index: usize) -> Option<Vec<T>> {
        if index > self.chunk_list.len() {
            None // Index out of bounds
        } else {
            Some(self.chunk_list[index].to_vec()) // Return the sublist as a Vec
        }
    }

    /// Calculate the number of partitions of the list, equivalent to ceiling(list.size()/chunkSize)
    pub fn count(&self) -> usize {
        (self.chunk_list.len() + self.chunk_size - 1) / self.chunk_size 
    }

    pub fn get_chunks(&self) -> &Vec<Vec<T>> {
        &self.chunk_list
    }
}



#[cfg(test)]
mod test {
    use std::assert_eq;

    use super::*;

    #[test]
    fn test_integer_partition() {
        let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let partition = Partition::of_size(list, 3);
        assert_eq!(4, partition.count());
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

