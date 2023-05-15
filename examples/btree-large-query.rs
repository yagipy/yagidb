use yagidb::btree::{BTree, SearchMode};
use yagidb::buffer::{BufferPool, BufferPoolManager};
use yagidb::disk::{DiskManager, PageId};

fn main() {
    let disk = DiskManager::open("large.btr").unwrap();
    let pool = BufferPool::new(10);
    let mut buffer_pool_manager = BufferPoolManager::new(disk, pool);

    let btree = BTree::new(PageId(0));
    let mut iter = btree.search(
        &mut buffer_pool_manager,
        SearchMode::Key(vec![
            0xec, 0x2c, 0xdd, 0x0e,
            0x4d, 0x0c, 0x94, 0x67,
            0x30, 0x58, 0xc7, 0xd7,
            0xbe, 0x7b, 0x85, 0xd2,
        ]),
    ).unwrap();

    let (key, value) = iter.next(&mut buffer_pool_manager).unwrap().unwrap();
    println!("{:02x?} = {:02x?}", key, value);
}
