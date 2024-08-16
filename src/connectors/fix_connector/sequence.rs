pub struct SequenceManager {
    current_seq_num: u64,
}

impl SequenceManager {
    pub fn new(start_seq_num: u64) -> Self {
        SequenceManager {
            current_seq_num: start_seq_num,
        }
    }

    pub fn get_next_seq_num(&mut self) -> u64 {
        self.current_seq_num += 1;
        self.current_seq_num
    }

    pub fn reset_seq_num(&mut self, new_seq_num: u64) {
        self.current_seq_num = new_seq_num;
    }
}
