use super::{nodes::{NodeParentOneChild, NodeIndex}, arena::VecArena};

pub struct DoublyLinkedList<T> {
    arena: VecArena<NodeParentOneChild<T>>,
    first_idx: NodeIndex,
    last_idx: NodeIndex,
}

impl<T> DoublyLinkedList<T> {
    pub fn new(first_val: T) -> DoublyLinkedList<T> {
        let first_node = NodeParentOneChild::new(first_val);
        let mut arena = VecArena::new();
        let first_idx = Some(arena.insert(first_node));
        return DoublyLinkedList { 
            arena, 
            first_idx,
            last_idx: first_idx
        };
    }

    pub fn with_capacity(first_val: T, capacity: usize) -> DoublyLinkedList<T> {
        let first_node = NodeParentOneChild::new(first_val);
        let mut arena = VecArena::with_capacity(capacity);
        let first_idx = Some(arena.insert(first_node));
        return DoublyLinkedList { 
            arena, 
            first_idx,
            last_idx: first_idx
        };
    }

    pub fn get_node_ref<'a>(&'a self, index: NodeIndex) -> Option<&'a NodeParentOneChild<T>> {
        match index {
            Some(idx) => self.arena.get_ref(idx),
            None => None
        }
    }

    pub fn get_node_mut<'a>(&'a mut self, index: NodeIndex) -> Option<&'a NodeParentOneChild<T>> {
        match index {
            Some(idx) => self.arena.get_ref(idx),
            None => None
        }
    }

    #[inline(always)]
    pub fn get_child_node_ref<'a>(&'a self, node: &NodeParentOneChild<T>) -> Option<&'a NodeParentOneChild<T>> {
        self.get_node_ref(node.child_idx)
    }

    #[inline(always)]
    pub fn get_child_node_mut<'a>(&'a mut self, node: &NodeParentOneChild<T>) -> Option<&'a NodeParentOneChild<T>> {
        self.get_node_mut(node.child_idx)
    }

    pub fn get_parent_node_ref<'a>(&'a self, node: &NodeParentOneChild<T>) -> Option<&'a NodeParentOneChild<T>> {
        self.get_node_ref(node.parent_idx)
    }

    #[inline(always)]
    pub fn get_parent_node_mut<'a>(&'a mut self, node: &NodeParentOneChild<T>) -> Option<&'a NodeParentOneChild<T>> {
        self.get_node_mut(node.parent_idx)
    }

    #[inline(always)]
    pub fn get_first_node_ref<'a>(&'a self) -> Option<&'a NodeParentOneChild<T>> {
        self.get_node_ref(self.first_idx)
    }

    #[inline(always)]
    pub fn get_first_node_mut<'a>(&'a mut self) -> Option<&'a NodeParentOneChild<T>> {
        self.get_node_mut(self.first_idx)
    }
}