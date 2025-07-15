/// Link-Cut Tree for dynamic LCA queries
pub struct LinkCutTree {
    parent: Vec<Option<usize>>,
    left: Vec<Option<usize>>,
    right: Vec<Option<usize>>,
    flip: Vec<bool>,
}

impl LinkCutTree {
    pub fn new(size: usize) -> Self {
        Self {
            parent: vec![None; size],
            left: vec![None; size],
            right: vec![None; size],
            flip: vec![false; size],
        }
    }

    /// Makes the node the root of its auxiliary tree
    fn splay(&mut self, x: usize) {
        self.push(x);
        while let Some(p) = self.parent[x] {
            let g = self.parent[p];
            if g.is_some() {
                self.push(g.unwrap());
            }
            self.push(p);
            self.push(x);

            let x_is_left = self.left[p] == Some(x);
            let p_is_left = g.map_or(false, |g| self.left[g] == Some(p));

            if p_is_left == x_is_left {
                self.rotate(p, x_is_left);
            } else {
                self.rotate(x, x_is_left);
            }
            self.rotate(x, p_is_left);
        }
    }

    /// Rotates x with its parent
    fn rotate(&mut self, x: usize, left: bool) {
        let p = self.parent[x].unwrap();
        let g = self.parent[p];

        if left {
            let b = self.right[x];
            self.right[x] = Some(p);
            self.left[p] = b;
        } else {
            let b = self.left[x];
            self.left[x] = Some(p);
            self.right[p] = b;
        }

        self.parent[p] = Some(x);
        self.parent[x] = g;

        if let Some(b) = if left { self.left[p] } else { self.right[p] } {
            self.parent[b] = Some(p);
        }

        if let Some(g) = g {
            if self.left[g] == Some(p) {
                self.left[g] = Some(x);
            } else {
                self.right[g] = Some(x);
            }
        }
    }

    /// Pushes lazy flip flags
    fn push(&mut self, x: usize) {
        if self.flip[x] {
            self.flip[x] = false;
            std::mem::swap(&mut self.left[x], &mut self.right[x]);
            if let Some(l) = self.left[x] {
                self.flip[l] ^= true;
            }
            if let Some(r) = self.right[x] {
                self.flip[r] ^= true;
            }
        }
    }

    /// Makes x the root of its represented tree
    pub fn make_root(&mut self, x: usize) {
        self.access(x);
        self.flip[x] = true;
    }

    /// Connects x to the root of y's tree
    pub fn link(&mut self, x: usize, y: usize) {
        self.make_root(x);
        self.parent[x] = Some(y);
    }

    /// Cuts x from its parent
    pub fn cut(&mut self, x: usize) {
        self.access(x);
        if let Some(l) = self.left[x] {
            self.parent[l] = None;
            self.left[x] = None;
        }
    }

    /// Brings x to the root of its auxiliary tree
    pub fn access(&mut self, mut x: usize) {
        let mut last = None;
        let mut y = x;

        while let Some(p) = self.parent[y] {
            self.splay(p);
            self.right[p] = last;
            last = Some(p);
            y = p;
        }

        self.splay(x);
        self.right[x] = last;
    }

    /// Finds LCA of x and y
    pub fn lca(&mut self, x: usize, y: usize) -> Option<usize> {
        if x == y {
            return Some(x);
        }

        self.access(x);
        self.access(y);

        self.splay(x);
        if self.parent[x].is_some() {
            Some(x)
        } else {
            None
        }
    }
}