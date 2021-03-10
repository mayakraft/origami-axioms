use Vector;

// type Tree = Vec<Vec<Vec<Vector>>>; 

const BUCKET: usize = 100;

// #[derive(Copy, Clone)]
pub struct Tree {
    pub buckets: Vec<Vec<Vec<(Vector, u64)>>>
}

impl Tree {
    pub fn push(&mut self, point: &Vector) {
        let idx = point.index();
        let bucket = &mut self.buckets[idx.0][idx.1];
        for i in 0..bucket.len() {
            if point.equivalent(&bucket[i].0) {
                bucket[i].1 += 1;
                return;
            }
        }
        self.buckets[idx.0][idx.1].push((*point, 1));
    }
}

pub fn make_tree () -> Tree {
    let mut buckets: Vec<Vec<Vec<(Vector, u64)>>> = Vec::new();
    for i in 0..BUCKET {
        let row: Vec<Vec<(Vector, u64)>> = Vec::new();
        buckets.push(row);
        for _j in 0..BUCKET {
            let col: Vec<(Vector, u64)> = Vec::new();
            buckets[i].push(col);
        }
    }
    return Tree { buckets };
}


