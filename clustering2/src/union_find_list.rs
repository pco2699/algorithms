use std::collections::HashMap;

#[derive(Debug)]
pub struct UnionFindList {
    nodes: Vec<Node>,
    cluster_size: HashMap<usize, usize> 
}

#[derive(Debug)]
struct Node {
    leader: usize,
}

impl UnionFindList {
    pub fn new(size: usize) -> Self {
        let mut cluster_size = HashMap::new();
        let mut nodes = vec![];
        for i in 0..size {
            nodes.push(Node {leader: i});
            cluster_size.insert(i, 1);
        }
        Self {
            nodes,
            cluster_size
        }
    }
    pub fn merge(&mut self, u: usize, v: usize) {
        if self.is_same(u, v) {
            return
        }

        let size_u = self.get_size(u);
        let size_v = self.get_size(v);

        let leader_u = self.find(u);
        let leader_v = self.find(v);

        if size_u > size_v {
            for n in self.nodes.iter_mut() {
                if n.leader == leader_v {
                    n.leader = leader_u;
                }
            }
            self.cluster_size.insert(leader_u, size_u + size_v);
            self.cluster_size.remove(&leader_v);
        } else {
            for n in self.nodes.iter_mut() {
                if n.leader == leader_u {
                    n.leader = leader_v;
                }
            }
            self.cluster_size.insert(leader_v, size_u + size_v);
            self.cluster_size.remove(&leader_u);
        }
    }
    pub fn get_size(&self, u: usize) -> usize {
        let leader = self.find(u);
        *self.cluster_size.get(&leader).unwrap()
    }
    pub fn is_same(&self, u: usize, v: usize) -> bool {
        self.find(u) == self.find(v)
    }
    pub fn find(&self, u: usize) -> usize {
        self.nodes[u].leader
    }
    pub fn len(&self) -> usize {
        self.cluster_size.len()
    }
}