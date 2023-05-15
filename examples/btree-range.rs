use yagidb::btree::{BTree, SearchMode};
use yagidb::buffer::{BufferPool, BufferPoolManager};
use yagidb::disk::{DiskManager, PageId};

fn main() {
    let disk = DiskManager::open("test.btr").unwrap();
    let pool = BufferPool::new(10);
    let mut buffer_pool_manager = BufferPoolManager::new(disk, pool);

    let btree = BTree::new(PageId(0));
    let mut iter = btree.search(&mut buffer_pool_manager, SearchMode::Key(b"Gifu".to_vec())).unwrap();
    while let Some((key, value)) = iter.next(&mut buffer_pool_manager).unwrap() {
        println!("{:02x?} = {:02x?}", key, value);
    }
}
