extern crate core;

use yagidb::buffer::{BufferPool, BufferPoolManager};
use yagidb::disk::{DiskManager, PageId};
use yagidb::query::{Filter, PlanNode, SeqScan, TupleSearchMode};
use yagidb::tuple;

fn main() {
    let disk = DiskManager::open("simple.yag").unwrap();
    let pool = BufferPool::new(10);
    let mut buffer_pool_manager = BufferPoolManager::new(disk, pool);

    let plan = Filter {
        cond: &|record| record[1].as_slice() < b"Dave",
        inner_plan: &SeqScan {
            table_meta_page_id: PageId(0),
            search_mode: TupleSearchMode::Key(&[b"w"]),
            while_cond: &|pkey| pkey[0].as_slice() < b"z",
        },
    };
    let mut exec = plan.start(&mut buffer_pool_manager).unwrap();

    while let Some(record) = exec.next(&mut buffer_pool_manager).unwrap() {
        println!("{:?}", tuple::Pretty(&record));
    }
}
