use std::io::Error;
use crate::btree::BTree;
use crate::buffer::BufferPoolManager;
use crate::disk::PageId;
use crate::tuple;

#[derive(Debug)]
pub struct SimpleTable {
    pub meta_page_id: PageId,
    pub num_key_elems: usize,
}

impl SimpleTable {
    pub fn create(&mut self, buffer_pool_manager: &mut BufferPoolManager) -> Result<(), Error> {
        let btree = BTree::create(buffer_pool_manager)?;
        self.meta_page_id = btree.meta_page_id;
        Ok(())
    }

    pub fn insert(&self, buffer_pool_manager: &mut BufferPoolManager, record: &[&[u8]]) -> Result<(), Error> {
        let btree = BTree::new(self.meta_page_id);
        let mut key = vec![];
        tuple::encode(record[..self.num_key_elems].iter(), &mut key);
        let mut value = vec![];
        tuple::encode(record[self.num_key_elems..].iter(), &mut value);
        btree.insert(buffer_pool_manager, &key, &value)
    }
}
