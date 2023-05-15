use yagidb::buffer::{BufferPool, BufferPoolManager};
use yagidb::disk::{DiskManager, PageId};
use yagidb::query::{IndexScan, PlanNode, TupleSearchMode};
use yagidb::tuple;

fn main() {
    let disk = DiskManager::open("table.yag").unwrap();
    let pool = BufferPool::new(10);
    let mut buffer_pool_manager = BufferPoolManager::new(disk, pool);

    let plan = IndexScan {
        table_meta_page_id: PageId(0),
        index_meta_page_id: PageId(2),
        search_mode: TupleSearchMode::Key(&[b"Smith"]),
        while_cond: &|skey| skey[0].as_slice() == b"Smith",
    };
    let mut exec = plan.start(&mut buffer_pool_manager).unwrap();

    while let Some(record) = exec.next(&mut buffer_pool_manager).unwrap() {
        println!("{:?}", tuple::Pretty(&record));
    }
}
