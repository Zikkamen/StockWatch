struct TreeNode {
    pub v: i64,
    pub l: Option<Box<TreeNode>>,
    pub r: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn new() -> Self {
        TreeNode{ v: 0, l: None, r: None }
    }
}

pub struct FenwickTree {
    root: Box<TreeNode>,
}

impl FenwickTree {
    pub fn new() -> Self {
        FenwickTree { root: Box::new(TreeNode::new()) }
    }

    pub fn find_num(&self, x: i64) -> (i64, i64) {
        let mut node = self.root.as_ref();
        let mut val: i64 = 0;

        for i in (0..64).rev() {
            if x & (1 << i) == 0 {
                if node.l.is_none() { return (val, val); }

                node = node.l.as_ref().unwrap();
            } else {
                if node.l.is_some() { val += node.l.as_ref().unwrap().v; }
                if node.r.is_none() { return (val, val); }

                node = node.r.as_ref().unwrap();
            }
        }

        (val, val + node.v)
    }

    pub fn insert(&mut self, x: i64, val: i64) {
        let mut node = self.root.as_mut();

        for i in (0..64).rev() {
            if x & (1 << i) == 0 {
                if node.l.is_none() { node.l = Some(Box::new(TreeNode::new())); }

                if val < 0 && node.l.as_ref().unwrap().v + val == 0 {
                    node.l = None;

                    return;
                }

                node = node.l.as_mut().unwrap();
            } else {
                if node.r.is_none() { node.r = Some(Box::new(TreeNode::new())); }

                if val < 0 && node.r.as_ref().unwrap().v + val == 0 {
                    node.r = None;

                    return;
                }

                node = node.r.as_mut().unwrap();
            }

            node.v += val;
        }
    }

    pub fn find_min(&self) -> i64 {
        let mut node = self.root.as_ref();
        let mut min_val: i64 = 0;

        for i in (0..64).rev() {
            if node.l.is_some() {
                node = node.l.as_ref().unwrap();
                continue;
            }

            if node.r.is_some() {
                min_val += 1 << i;
                node = node.r.as_ref().unwrap();
                continue;
            }

            break;
        }

        min_val
    }

    pub fn find_max(&self) -> i64 {
        let mut node = self.root.as_ref();
        let mut max_val: i64 = 0;

        for i in (0..64).rev() {
            if node.r.is_some() {
                max_val += 1 << i;
                node = node.r.as_ref().unwrap();
                continue;
            }

            if node.l.is_some() {
                node = node.l.as_ref().unwrap();
                continue;
            }

            break;
        }

        max_val
    }
}
