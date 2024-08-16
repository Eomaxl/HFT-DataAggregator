use std::collections::{VecDeque, HashSet};
use crate::processing::data_processing::StockData;

pub struct DataBuffer {
    queue: VecDeque<StockData>,
    seen: HashSet<String>,
}

impl DataBuffer {
    pub fn new() -> Self {
        DataBuffer {
            queue: VecDeque::new(),
            seen: HashSet::new(),
        }
    }

    pub fn add_data(&mut self, data: StockData){
        if self.seen.contains(&data.symbol){
            return;
        }
        self.seen.insert(data.symbol.clone());
        self.queue.push_back(data);
    }

    pub fn get_ordered_data(&mut self)-> Vec<StockData>{
        let mut data_vec: Vec<StockData> = self.queue.drain(..).collect();
        data_vec.sort_by_key(|d|d.timestamp);
        data_vec
    }
}
