use yagidb::btree::{BTree, SearchMode};
use yagidb::buffer::{BufferPool, BufferPoolManager};
use yagidb::disk::{DiskManager, PageId};
use yagidb::tuple;

fn main() {
    let disk = DiskManager::open("simple.yag").unwrap();
    let pool = BufferPool::new(10);
    let mut buffer_pool_manager = BufferPoolManager::new(disk, pool);

    let btree = BTree::new(PageId(0));
    let mut iter = btree.search(&mut buffer_pool_manager, SearchMode::Start).unwrap();

    while let Some((key, value)) = iter.next(&mut buffer_pool_manager).unwrap() {
        let mut record = vec![];
        tuple::decode(&key, &mut record);
        tuple::decode(&value, &mut record);
        if record[2] == b"Smith" {
            println!("{:?}", tuple::Pretty(&record));
        }
    }
}
