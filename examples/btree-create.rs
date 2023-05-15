use yagidb::btree::BTree;
use yagidb::buffer::{BufferPool, BufferPoolManager};
use yagidb::disk::DiskManager;

fn main() {
    let disk = DiskManager::open("test.btr").unwrap();
    let pool = BufferPool::new(10);
    let mut buffer_pool_manager = BufferPoolManager::new(disk, pool);

    let btree = BTree::create(&mut buffer_pool_manager).unwrap();
    btree.insert(&mut buffer_pool_manager, b"Kanagawa", b"Yokohama").unwrap();
    btree.insert(&mut buffer_pool_manager, b"Osaka", b"Osaka").unwrap();
    btree.insert(&mut buffer_pool_manager, b"Aichi", b"Nagoya").unwrap();
    btree.insert(&mut buffer_pool_manager, b"Hokkaido", b"Sapporo").unwrap();
    btree.insert(&mut buffer_pool_manager, b"Fukuoka", b"Fukuoka").unwrap();
    btree.insert(&mut buffer_pool_manager, b"Hyogo", b"Kobe").unwrap();

    buffer_pool_manager.flush().unwrap();
}
