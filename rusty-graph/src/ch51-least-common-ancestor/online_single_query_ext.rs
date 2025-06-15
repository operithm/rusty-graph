/// 2. Online Single Query Extension
///
/// This is the second part of single-query LCA algorithm compilation, with extending
/// to arbitrary tree, bidirectional trees, and k-node LCA finding algorithm.

///
/// Algorithm: Recursive DFS (No Preprocessing)
/// Use Case: Single LCA query in an arbitrary tree.
///
/// Approach:
///
/// Tree Structure:
/// Employee has an id and a Vec<Employee> for children.
///
/// DFS Logic:
///
/// For each node, check if it matches p_id or q_id.
///
/// Recursively search all children.
///
/// If both p and q are found in the subtree of the current node, and no LCA has been set yet, the current node is the LCA.
///
/// Termination:
///
/// The first node where p and q appear in different subtrees is the LCA.
///
/// Time: O(N) per query.  Space: O(N) (recursion stack).
pub fn find_lca(root: &Employee, p_id: i32, q_id: i32) -> Option<i32> {
    fn dfs(node: &Employee, p_id: i32, q_id: i32, lca: &mut Option<i32>) -> (bool, bool) {
        let mut found_p = node.id == p_id;
        let mut found_q = node.id == q_id;

        for child in &node.children {
            let (child_found_p, child_found_q) = dfs(child, p_id, q_id, lca);
            found_p = found_p || child_found_p;
            found_q = found_q || child_found_q;
        }

        // If both p and q are found for the first time, set LCA.
        if found_p && found_q && lca.is_none() {
            *lca = Some(node.id);
        }

        (found_p, found_q)
    }

    let mut lca = None;
    dfs(root, p_id, q_id, &mut lca);
    lca
}

/// B. Arbitrary Tree (Parent → Child)
/// Algorithm: Path Comparison (No Preprocessing)
/// Use Case: Single LCA query in a general tree.
///
/// Approach:
///
/// Find paths from root to p and q using DFS.
///
/// Compare paths to find the last common node.
///
/// Time: O(N) per query. Space: O(N) (path storage).
///
fn lca_arbitrary(root: &Employee, p: i32, q: i32) -> Option<i32> {
    fn get_path(node: &Employee, target: i32, path: &mut Vec<i32>) -> bool {
        path.push(node.id);
        if node.id == target {
            return true;
        }
        for child in &node.children {
            if get_path(child, target, path) {
                return true;
            }
        }
        path.pop();
        false
    }

    let mut path_p = Vec::new();
    let mut path_q = Vec::new();
    get_path(root, p, &mut path_p);
    get_path(root, q, &mut path_q);

    path_p.iter()
        .zip(path_q.iter())
        .take_while(|(a, b)| a == b)
        .last()
        .map(|(x, _)| *x)
}

/// C. Bidirectional Tree (Parent ↔ Child)
/// Algorithm: Bidirectional BFS (No Preprocessing)
/// Use Case: Single LCA query in a tree with bidirectional edges (e.g., employee can access manager).
///
/// Approach:
///
/// From p and q, perform BFS upwards (toward root).
///
/// The first common node in their paths is the LCA.
///
/// Time: O(N) per query.
///
/// Space: O(N) (visited nodes).

use std::collections::{HashMap, HashSet, VecDeque};

pub fn lca_bidirectional(root: &Employee, p: i32, q: i32) -> Option<i32> {
    let mut parent_map = HashMap::new();
    let mut stack = vec![root];
    while let Some(node) = stack.pop() {
        for child in &node.children {
            parent_map.insert(child.id, node.id);
            stack.push(child);
        }
    }

    let mut ancestors_p = HashSet::new();
    let mut current = p;
    while let Some(&parent) = parent_map.get(current) {
        ancestors_p.insert(current);
        current = parent;
    }
    ancestors_p.insert(current); // Include root

    let mut current_q = q;
    while !ancestors_p.contains(current_q) {
        current_q = parent_map[current_q];
    }
    Some(current_q)
}

pub fn lca_k_nodes(root: &Employee, nodes: &[i32]) -> Option<i32> {
    fn get_path(node: &Employee, target: i32, path: &mut Vec<i32>) -> bool {
        path.push(node.id);
        if node.id == target {
            return true;
        }
        for child in &node.children {
            if get_path(child, target, path) {
                return true;
            }
        }
        path.pop();
        false
    }

    let mut paths: Vec<Vec<i32>> = Vec::new();
    for &node_id in nodes {
        let mut path = Vec::new();
        if !get_path(root, node_id, &mut path) {
            return None; // Node not found
        }
        paths.push(path);
    }

    let mut lca = root.id;
    for i in 0.. {
        let mut current = None;
        for path in &paths {
            if i >= path.len() || (current.is_some() && path[i] != current.unwrap()) {
                return Some(lca);
            }
            current = Some(path[i]);
        }
        lca = current.unwrap();
    }
    Some(lca)
}


///
pub fn lca_with_count(root: &Rc<RefCell<Employee>>, p: i32, q: i32) -> Option<i32> {
    fn dfs(
        node: &Rc<RefCell<Employee>>,
        p: i32,
        q: i32,
        lca: &mut Option<i32>,
    ) -> (bool, bool) {
        let node_ref = node.borrow();
        let mut found_p = node_ref.id == p;
        let mut found_q = node_ref.id == q;

        for child in &node_ref.children {
            let (child_found_p, child_found_q) = dfs(child, p, q, lca);
            found_p = found_p || child_found_p;
            found_q = found_q || child_found_q;
        }

        // If both p and q are found for the first time, set LCA.
        if found_p && found_q && lca.is_none() {
            *lca = Some(node_ref.id);
        }

        (found_p, found_q)
    }

    let mut lca = None;
    dfs(root, p, q, &mut lca);
    lca
}

fn find_lca_by_count(root: &Rc<RefCell<Employee>>, p: i32, q: i32) -> Option<Rc<RefCell<Employee>>> {
    fn dfs(
        node: &Rc<RefCell<Employee>>,
        p: i32,
        q: i32,
        lca: &mut Option<Rc<RefCell<Employee>>>,
    ) -> i32 {
        let node_ref = node.borrow();
        let mut count = if node_ref.id == p || node_ref.id == q { 1 } else { 0 };

        for child in &node_ref.children {
            let child_count = dfs(child, p, q, lca);
            // LCA condition: Current node is between two targets.
            if child_count == 1 && count == 1 && lca.is_none() {
                *lca = Some(node.clone());
            }
            count += child_count;
        }

        count
    }

    let mut lca = None;
    dfs(root, p, q, &mut lca);
    lca
}

pub fn lca_k_nodes_with_count(root: &Rc<RefCell<Employee>>, nodes: &[i32],
    node: &Rc<RefCell<Employee>>,
    targets: &HashSet<i32>,
    lca: &mut Option<i32>,
) -> usize {
    let node_ref = node.borrow();
    let mut count = if targets.contains(&node_ref.id) { 1 } else { 0 };

    for child in &node_ref.children {
        count += lca_k_nodes_with_count(child, targets, lca);
    }

    if count == targets.len() && lca.is_none() {
        *lca = Some(node_ref.id);
    }
    count
}

fn dfs_k_nodes(
    node: &Rc<RefCell<Employee>>,
    targets: &HashSet<i32>,
    lca: &mut Option<Rc<RefCell<Employee>>>,
) -> usize {
    let node_ref = node.borrow();
    let mut count = if targets.contains(&node_ref.id) { 1 } else { 0 };

    for child in &node_ref.children {
        let child_count = dfs_k_nodes(child, targets, lca);
        if child_count == 1 && count == targets.len() - 1 && lca.is_none() {
            *lca = Some(node.clone());
        }
        count += child_count;
    }

    count
}



#[cfg(test)]
mod tests {}


