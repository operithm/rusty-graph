
use std::rc::Rc;
use std::cell::RefCell;

/// Definition for a binary tree node.
/// To apply this tree node to bidirectional algorithm, a child-to-parent map
/// is to prebuild.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

/// Definition for an arbitrary tree
///
use std::collections::HashMap;

#[derive(Debug)]
pub struct Employee {
    id: i32,
    children: Vec<Employee>,
}

impl Employee {
    fn new(id: i32) -> Self {
        Employee {
            id,
            children: Vec::new(),
        }
    }
}
