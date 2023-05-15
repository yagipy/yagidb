use yagidb::buffer::{BufferPool, BufferPoolManager};
use yagidb::disk::{DiskManager, PageId};
use yagidb::table::{Table, UniqueIndex};

fn main() {
    let disk = DiskManager::open("table.yag").unwrap();
    let pool = BufferPool::new(10);
    let mut buffer_pool_manager = BufferPoolManager::new(disk, pool);

    let mut table = Table {
        meta_page_id: PageId::INVALID_PAGE_ID,
        num_key_elems: 1,
        unique_indexes: vec![
            UniqueIndex {
                meta_page_id: PageId::INVALID_PAGE_ID,
                skey: vec![2],
            },
        ],
    };
    table.create(&mut buffer_pool_manager).unwrap();
    dbg!(&table);
    table.insert(&mut buffer_pool_manager, &[b"z", b"Alice", b"Smith"]).unwrap();
    table.insert(&mut buffer_pool_manager, &[b"x", b"Bob", b"Johnson"]).unwrap();
    table.insert(&mut buffer_pool_manager, &[b"y", b"Charlie", b"Williams"]).unwrap();
    table.insert(&mut buffer_pool_manager, &[b"w", b"Dave", b"Miller"]).unwrap();
    table.insert(&mut buffer_pool_manager, &[b"v", b"Eve", b"Brown"]).unwrap();

    buffer_pool_manager.flush().unwrap();
}
