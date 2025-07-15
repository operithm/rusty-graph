
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

pub struct BinaryTreeNode<T> {
    pub value: T,
    pub left: Option<Rc<RefCell<BinaryTreeNode<T>>>>,
    pub right: Option<Rc<RefCell<BinaryTreeNode<T>>>>,
}
pub struct BidirectionalTreeNode<T> {
    pub value: T,
    pub parent: Option<Rc<RefCell<BidirectionalTreeNode<T>>>>, // Weak reference preferred for parent
    pub children: Vec<Rc<RefCell<BidirectionalTreeNode<T>>>>,
}


pub struct DagNode<T> {
    pub value: T,
    pub successors: Vec<Rc<RefCell<DagNode<T>>>>,
    // For topological sorting/traversal:
    pub in_degree: Cell<usize>,
}

pub struct ThreadedBinaryTreeNode<T> {
    pub value: T,
    pub left: Option<Rc<RefCell<Self>>>,
    pub right: Option<Rc<RefCell<Self>>>,
    pub is_threaded: bool, // True if right points to in-order successor
}

pub struct ArenaTree<T> {
    nodes: Vec<Node<T>>,
    root: Option<NodeId>,
}

pub struct Node<T> {
    pub value: T,
    pub left: Option<NodeId>,
    pub right: Option<NodeId>,
    pub parent: Option<NodeId>,
}

type NodeId = usize;
