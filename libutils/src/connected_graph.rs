use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

pub struct ConnectionGraph<TNodeId>
where
    TNodeId: Copy + Hash,
{
    circuit_parents: HashMap<TNodeId, TNodeId>,
}

impl<T> Debug for ConnectionGraph<T>
where
    T: Debug + Copy + Eq + Hash,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConnectionGraph")
            .field("circuit_parents", &self.circuit_parents)
            .finish()
    }
}

impl<TNodeId> ConnectionGraph<TNodeId>
where
    TNodeId: Copy + Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            circuit_parents: HashMap::new(),
        }
    }
    pub fn connect(&mut self, parent: TNodeId, child: TNodeId) {
        let parent_parent = self.get_network_id_with_fixup(parent);

        let child_parent = self.get_network_id_with_fixup(child);

        if let Some(existing) = self.circuit_parents.get_mut(&child_parent) {
            *existing = parent_parent;
        } else {
            self.circuit_parents.insert(child_parent, parent_parent);
            self.circuit_parents.insert(child, parent_parent);
        }
    }
    pub fn get_network_id_with_fixup(&mut self, id: TNodeId) -> TNodeId {
        if let Some(parent) = self.circuit_parents.get(&id).cloned() {
            if parent == id {
                return parent;
            }
            let new_parent = self.get_network_id_with_fixup(parent);
            self.circuit_parents.insert(id, new_parent);
            return new_parent;
        } else {
            return id;
        }
    }
    pub fn get_network_id(&self, id: TNodeId) -> TNodeId {
        if let Some(parent) = self.circuit_parents.get(&id).cloned() {
            if parent == id {
                return parent;
            }
            self.get_network_id(parent)
        } else {
            return id;
        }
    }
    pub fn all_networks(&self) -> HashMap<TNodeId, HashSet<TNodeId>> {
        let mut result: HashMap<TNodeId, HashSet<TNodeId>> = HashMap::new();
        for child in self.circuit_parents.keys() {
            let parent = self.get_network_id(*child);
            let val = result.entry(parent).or_insert(HashSet::new());

            val.insert(parent);
            val.insert(*child);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::connected_graph::ConnectionGraph;

    #[test]
    pub fn test_connected_graphs() {
        let mut connected_graph = ConnectionGraph::new();
        connected_graph.connect(0, 19);
        println!("{:?}", connected_graph);
        connected_graph.connect(0, 7);
        println!("{:?}", connected_graph);
        connected_graph.connect(7, 19);
        println!("{:?}", connected_graph);
        connected_graph.connect(14, 19);
        println!("{:?}", connected_graph);
        println!("{:?}", connected_graph.all_networks());
        println!("{:?}", connected_graph);
        assert_eq!(connected_graph.all_networks().len(), 1);
    }
}
