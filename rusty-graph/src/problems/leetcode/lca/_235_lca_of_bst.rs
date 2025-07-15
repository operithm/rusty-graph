use solution::Solution;

// Iterative Solution (Optimal for BST)
pub fn lowest_common_ancestor_iterative(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let p_val = p.as_ref().unwrap().borrow().val;
    let q_val = q.as_ref().unwrap().borrow().val;
    let mut current = root;

    while let Some(node) = current {
        let node_val = node.borrow().val;
        if p_val < node_val && q_val < node_val {
            current = node.borrow().left.clone();
        } else if p_val > node_val && q_val > node_val {
            current = node.borrow().right.clone();
        } else {
            return Some(node);
        }
    }
    None
}

fn main() {
    // Example BST: [6,2,8,0,4,7,9,null,null,3,5]
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 6,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        }))),
    })));

    let p = Some(Rc::new(RefCell::new(TreeNode::new(2)))); // Node 2
    let q = Some(Rc::new(RefCell::new(TreeNode::new(8)))); // Node 8

    let lca = lowest_common_ancestor_iterative(root.clone(), p, q);
    println!("LCA: {}", lca.unwrap().borrow().val); // Output: 6
}