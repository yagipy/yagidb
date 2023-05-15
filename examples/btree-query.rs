use yagidb::disk::{DiskManager, PageId};
use yagidb::buffer::{BufferPool, BufferPoolManager};
use yagidb::btree::{BTree, SearchMode};

fn main() {
    let disk = DiskManager::open("test.btr").unwrap();
    let pool = BufferPool::new(10);
    let mut buffer_pool_manager = BufferPoolManager::new(disk, pool);

    let btree = BTree::new(PageId(0));
    let mut iter = btree.search(&mut buffer_pool_manager, SearchMode::Key(b"Hyogo".to_vec())).unwrap();
    let (key, value) = iter.next(&mut buffer_pool_manager).unwrap().unwrap();
    println!("{:02x?} {:02x?}", key, value);
}
