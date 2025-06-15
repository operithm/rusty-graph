impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let n = is_connected.len();
        let mut uf = UnionFind::new(n);

        for i in 0..n {
            for j in 0..n {
                if is_connected[i][j] == 1 {
                    uf.union(i, j);
                }
            }
        }

        uf.count as i32
    }
}