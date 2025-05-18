use std::collections::HashMap;

#[derive(Debug)]
pub enum TreeNode {
    Branch {
        attribute: String,
        children: HashMap<String, TreeNode>,
    },
    Leaf {
        class: String,
    },
}

pub struct DecisionTree {
    pub root: TreeNode,
}