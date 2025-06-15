/// 1. Simple BST LCA
///
///
/// BST Property Exploitation:
///
/// Iterative Approach: Traverse the tree by comparing p and q values with the current node. Move left/right based on comparisons until the LCA is found.
///
/// Recursive Approach: Same logic as the iterative version, but uses recursion to traverse the tree.
///
///
/// Time Complexity:
///
/// O(H) where H is the height of the tree (best case O(log N) for balanced BST, worst case O(N) for skewed trees).
///
/// Space Complexity:
///
/// O(1) (iterative), O(H) (recursive due to call stack).
///
/// Iterative Solution (Optimal for BST)
pub fn lca_bst_iterative(
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

/// Recursive Solution (Elegant but uses call stack)
pub fn lca_bst_recursive(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let root_val = root.as_ref().unwrap().borrow().val;
    let p_val = p.as_ref().unwrap().borrow().val;
    let q_val = q.as_ref().unwrap().borrow().val;

    if p_val < root_val && q_val < root_val {
        lowest_common_ancestor_recursive(root.unwrap().borrow().left.clone(), p, q)
    } else if p_val > root_val && q_val > root_val {
        lowest_common_ancestor_recursive(root.unwrap().borrow().right.clone(), p, q)
    } else {
        root
    }
}

/// 2. General Binary Tree LCA
///
/// The binary tree is a single-rooted tree and maximum two children are allowed for each
/// tree node. The tree node may not have orders except for its binary topological order


/// The default implementation for single-query online Binary Tree LCA with recursive DFS
/// support
pub fn lca (
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {

    /// The default DFS implementation to support binary tree LCA(p, q) finding
    fn dfs(
        node: &Option<Rc<RefCell<TreeNode>>>,
        p_val: i32,
        q_val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match node {
            None => None,
            Some(inner) => {
                let node_ref = inner.borrow();
                // If current node is p or q, return it.
                if node_ref.val == p_val || node_ref.val == q_val {
                    return Some(inner.clone());
                }

                // Recursively search left and right subtrees.
                let left = dfs(&node_ref.left, p_val, q_val);
                let right = dfs(&node_ref.right, p_val, q_val);

                // If both left and right are non-null, current node is LCA.
                if left.is_some() && right.is_some() {
                    return Some(inner.clone());
                }
                // Otherwise, propagate the non-null result.
                left.or(right)
            }
        }
    }

    dfs(&root, p.as_ref().unwrap().borrow().val, q.as_ref().unwrap().borrow().val)
}

use std::collections::{HashMap, HashSet};

/// The default interactive path-compression LCA method
pub fn lca_iterative(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let mut parent_map = HashMap::new();
    let mut stack = vec![root.unwrap()]; // Assume root is not None.

    // Build parent pointers via iterative DFS.
    while let Some(node) = stack.pop() {
        let node_ref = node.borrow();
        if let Some(left) = &node_ref.left {
            parent_map.insert(left.borrow().val, node.clone());
            stack.push(left.clone());
        }
        if let Some(right) = &node_ref.right {
            parent_map.insert(right.borrow().val, node.clone());
            stack.push(right.clone());
        }
    }

    // Collect ancestors of p.
    let mut ancestors = HashSet::new();
    let mut current = p.unwrap();
    loop {
        ancestors.insert(current.borrow().val);
        match parent_map.get(&current.borrow().val) {
            Some(parent) => current = parent.clone(),
            None => break,
        }
    }

    // Find the first common ancestor of q in p's ancestors.
    let mut current_q = q.unwrap();
    loop {
        if ancestors.contains(&current_q.borrow().val) {
            return Some(current_q);
        }
        current_q = parent_map.get(&current_q.borrow().val).unwrap().clone();
    }
}

/// Alternative recursive Binary Tree LCA implementation without explicit DFS
pub fn lca_binary(root: &TreeNode, p: i32, q: i32) -> Option<i32> {
    //the node itself is one of the target node, return itself
    if root.val == p || root.val == q {
        return Some(root.val);
    }

    //depth-first search on each of children if exists
    let left = root.left.as_ref().and_then(|n| lca_binary(n, p, q));
    let right = root.right.as_ref().and_then(|n| lca_binary(n, p, q));

    //fist found parent, i.e. the deepest from root, or the shallowest from
    //the two children, is the LCA
    match (left, right) {
        (Some(_), Some(_)) => Some(root.val), // Current node is LCA
        (Some(l), None) => Some(l),           // Propagate left result
        (None, Some(r)) => Some(r),           // Propagate right result
        _ => None,                            // Not found
    }
}

/// A variation of the above implementation with node reference
pub fn lca_binary(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() || root == p || root == q {
        return root;
    }
    let left = lca_binary(root.as_ref().unwrap().borrow().left.clone(), p.clone(), q.clone());
    let right = lca_binary(root.as_ref().unwrap().borrow().right.clone(), p, q);

    match (left.is_some(), right.is_some()) {
        (true, true) => root,
        (true, false) => left,
        (false, true) => right,
        _ => None,
    }
}

/// A recursive implementation with DFS returning count of found node LCA.
/// it slightly has more complex logic than enumerating 4 finding-missing cases,
/// but it is easier to generalize for k-node LCA problem, and 2-node arbitrary
/// tree LCA
///
pub fn lca_count(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    fn dfs(
        node: &Option<Rc<RefCell<TreeNode>>>,
        p_val: i32,
        q_val: i32,
        lca: &mut Option<Rc<RefCell<TreeNode>>>,
    ) -> i32 {
        if let Some(inner) = node {
            let node_ref = inner.borrow();
            let mut count = if node_ref.val == p_val || node_ref.val == q_val { 1 } else { 0 };
            let left_count = dfs(&node_ref.left, p_val, q_val, lca);
            let right_count = dfs(&node_ref.right, p_val, q_val, lca);
            count += left_count + right_count;

            // Set LCA if current node is the first where count == 2.
            if count == 2 && lca.is_none() {
                *lca = Some(inner.clone());
            }
            count
        } else {
            0
        }
    }

    let mut lca = None;
    dfs(&root, p.unwrap().borrow().val, q.unwrap().borrow().val, &mut lca);
    lca
}

