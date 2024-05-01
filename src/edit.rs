use std::default::Default;

#[derive(Debug)]
pub enum Edit {
    Delete {
        pos: usize
    },
    Insert {
        pos: usize,
        value: usize
    },
}

#[derive(Debug)]
pub struct TotalEdits {
    pub inserts: usize,
    pub deletes: usize,
    pub ops: Vec<Edit>,
}

impl TotalEdits {
    #[allow(dead_code)]
    pub fn merge(mut self, other: Self) -> Self {
        self.inserts += other.inserts;
        self.deletes += other.deletes;
        self.ops.extend(other.ops);
        self
    }
}

impl Default for TotalEdits {
    fn default() -> Self {
        TotalEdits {
            inserts: 0,
            deletes: 0,
            ops: Vec::new()
        }
    }
}
