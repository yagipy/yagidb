use md5::{Digest, Md5};
use yagidb::btree::BTree;
use yagidb::buffer::{BufferPool, BufferPoolManager};
use yagidb::disk::DiskManager;

const NUM_PAIRS: u32 = 1_000_000;

fn main() {
    let disk = DiskManager::open("large.btr").unwrap();
    let pool = BufferPool::new(100);
    let mut buffer_pool_manager = BufferPoolManager::new(disk, pool);

    let btree = BTree::create(&mut buffer_pool_manager).unwrap();
    for i in 1u32..=NUM_PAIRS {
        let pkey = i.to_be_bytes();
        let md5 = Md5::digest(&pkey);
        btree.insert(&mut buffer_pool_manager, &md5[..], &pkey[..]).unwrap();
    }
    buffer_pool_manager.flush().unwrap();
}
