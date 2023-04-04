use super::{nodes::{NodeOneChild, NodeIndex}, arena::VecArena};

pub struct LinkedList<T> {
    arena: VecArena<NodeOneChild<T>>,
    first_idx: NodeIndex
}

impl<T> LinkedList<T> {
    pub fn new(first_val: T) -> LinkedList<T> {
        let first_node = NodeOneChild::new(first_val);
        let mut arena = VecArena::new();
        let first_idx = Some(arena.insert(first_node));
        return LinkedList { 
            arena, 
            first_idx
        };
    }

    pub fn with_capacity(first_val: T, capacity: usize) -> LinkedList<T> {
        let first_node = NodeOneChild::new(first_val);
        let mut arena = VecArena::with_capacity(capacity);
        let first_idx = Some(arena.insert(first_node));
        return LinkedList { 
            arena, 
            first_idx
        };
    }

    pub fn get_node_ref<'a>(&'a self, index: NodeIndex) -> Option<&'a NodeOneChild<T>> {
        match index {
            Some(idx) => self.arena.get_ref(idx),
            None => None
        }
    }

    pub fn get_node_mut<'a>(&'a mut self, index: NodeIndex) -> Option<&'a NodeOneChild<T>> {
        match index {
            Some(idx) => self.arena.get_ref(idx),
            None => None
        }
    }

    #[inline(always)]
    pub fn get_child_node_ref<'a>(&'a self, node: &NodeOneChild<T>) -> Option<&'a NodeOneChild<T>> {
        self.get_node_ref(node.child_idx)
    }

    #[inline(always)]
    pub fn get_child_node_mut<'a>(&'a mut self, node: &NodeOneChild<T>) -> Option<&'a NodeOneChild<T>> {
        self.get_node_mut(node.child_idx)
    }

    #[inline(always)]
    pub fn get_first_node_ref<'a>(&'a self) -> Option<&'a NodeOneChild<T>> {
        self.get_node_ref(self.first_idx)
    }

    #[inline(always)]
    pub fn get_first_node_mut<'a>(&'a mut self) -> Option<&'a NodeOneChild<T>> {
        self.get_node_mut(self.first_idx)
    }

    pub fn delete_child_link_grandchild(&mut self, node: &mut NodeOneChild<T>) {
        if let Some(child_idx) = node.child_idx {
            let child_node = self.get_node_ref(Some(child_idx)).unwrap();
            self.arena.delete(child_idx)
            node.child_idx = self.get_child_node_ref(child_node)
            
        }
    }
}