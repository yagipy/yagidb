use md5::Md5;
use yagidb::buffer::{BufferPool, BufferPoolManager};
use yagidb::disk::{DiskManager, PageId};
use yagidb::table::{Table, UniqueIndex};
use sha1::{Digest, Sha1};

const NUM_ROWS: u32 = 10_000_000;

fn main() {
    let disk = DiskManager::open("table.yag").unwrap();
    let pool = BufferPool::new(1_000_000);
    let mut buffer_pool_manager = BufferPoolManager::new(disk, pool);

    let mut table = Table {
        meta_page_id: PageId(0),
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
    for i in 0u32..NUM_ROWS {
        let pkey = i.to_be_bytes();
        let md5 = Md5::digest(&pkey);
        let sha1 = Sha1::digest(&pkey);
        table.insert(&mut buffer_pool_manager, &[&pkey[..], &md5[..], &sha1[..]]).unwrap();
    }
    buffer_pool_manager.flush().unwrap();
}
