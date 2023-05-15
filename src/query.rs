use std::io::Error;
use crate::{btree, tuple};
use crate::btree::{BTree, SearchMode};
use crate::buffer::BufferPoolManager;
use crate::disk::PageId;

pub type Tuple = Vec<Vec<u8>>;
pub type TupleSlice<'a> = &'a [Vec<u8>];

pub enum TupleSearchMode<'a> {
    Start,
    Key(&'a [&'a [u8]]),
}

impl<'a> TupleSearchMode<'a> {
    fn encode(&self) -> SearchMode {
        match self {
            TupleSearchMode::Start => SearchMode::Start,
            TupleSearchMode::Key(tuple) => {
                let mut key = vec![];
                tuple::encode(tuple.iter(), &mut key);
                SearchMode::Key(key)
            }
        }
    }
}

pub trait Executor {
    fn next(&mut self, buffer_pool_manager: &mut BufferPoolManager) -> Result<Option<Tuple>, Error>;
}

pub type BoxExecutor<'a> = Box<dyn Executor + 'a>;

pub trait PlanNode {
    fn start(&self, buffer_pool_manager: &mut BufferPoolManager) -> Result<BoxExecutor, Error>;
}

pub struct SeqScan<'a> {
    pub table_meta_page_id: PageId,
    pub search_mode: TupleSearchMode<'a>,
    pub while_cond: &'a dyn Fn(TupleSlice) -> bool,
}

impl<'a> PlanNode for SeqScan<'a> {
    fn start(&self, buffer_pool_manager: &mut BufferPoolManager) -> Result<BoxExecutor, Error> {
        let btree = BTree::new(self.table_meta_page_id);
        let table_iter = btree.search(buffer_pool_manager, self.search_mode.encode())?;
        Ok(Box::new(ExecSeqScan {
            table_iter,
            while_cond: self.while_cond,
        }))
    }
}

pub struct ExecSeqScan<'a> {
    table_iter: btree::Iter,
    while_cond: &'a dyn Fn(TupleSlice) -> bool,
}

impl<'a> Executor for ExecSeqScan<'a> {
    fn next(&mut self, buffer_pool_manager: &mut BufferPoolManager) -> Result<Option<Tuple>, Error> {
        let (pkey_bytes, tuple_bytes) = match self.table_iter.next(buffer_pool_manager)? {
            Some(pair) => pair,
            None => return Ok(None),
        };
        let mut pkey = vec![];
        tuple::decode(&pkey_bytes, &mut pkey);
        if !(self.while_cond)(&pkey) {
            return Ok(None);
        }
        let mut tuple = pkey;
        tuple::decode(&tuple_bytes, &mut tuple);
        Ok(Some(tuple))
    }
}

pub struct Filter<'a> {
    pub inner_plan: &'a dyn PlanNode,
    pub cond: &'a dyn Fn(TupleSlice) -> bool,
}

impl<'a> PlanNode for Filter<'a> {
    fn start(&self, buffer_pool_manager: &mut BufferPoolManager) -> Result<BoxExecutor, Error> {
        let inner_iter = self.inner_plan.start(buffer_pool_manager)?;
        Ok(Box::new(ExecFilter {
            inner_iter,
            cond: self.cond,
        }))
    }
}

pub struct ExecFilter<'a> {
    inner_iter: BoxExecutor<'a>,
    cond: &'a dyn Fn(TupleSlice) -> bool,
}

impl<'a> Executor for ExecFilter<'a> {
    fn next(&mut self, buffer_pool_manager: &mut BufferPoolManager) -> Result<Option<Tuple>, Error> {
        loop {
            match self.inner_iter.next(buffer_pool_manager)? {
                Some(tuple) => {
                    if (self.cond)(&tuple) {
                        return Ok(Some(tuple));
                    }
                }
                None => return Ok(None),
            }
        }
    }
}
