use super::arena::VecArena;


pub type NodeIndex = Option<usize>;

pub struct NodeOneChild<T> {
    val: T,
    pub child_idx: NodeIndex
}

impl<T> NodeOneChild<T> {
    #[inline(always)]
    pub fn new(val: T) -> Self {
        return NodeOneChild { val, child_idx: None }
    }

    #[inline(always)]
    pub fn new_full(val: T, child_idx: NodeIndex) -> Self {
        return NodeOneChild { val, child_idx }
    }

    #[inline(always)]
    pub fn val_ref<'a>(&'a self) -> &'a T {
        return &self.val
    }

    #[inline(always)]
    pub fn val_mut<'a>(&'a mut self) -> &'a mut T {
        return &mut self.val
    }

    pub fn child_mut<'a>(&self, arena: &'a mut VecArena<Self>) -> Option<&'a mut Self> {
        match self.child_idx {
            Some(idx) => arena.get_mut(idx),
            None => None
        }
    }

    pub fn child_ref<'a>(&self, arena: &'a VecArena<Self>) -> Option<&'a Self> {
        match self.child_idx {
            Some(idx) => arena.get_ref(idx),
            None => None
        }
    }
}

pub struct NodeParentOneChild<T> {
    pub val: T,
    pub parent_idx: NodeIndex, 
    pub child_idx: NodeIndex
}

impl<T> NodeParentOneChild<T> {
    #[inline(always)]
    pub fn new(val: T) -> NodeParentOneChild<T> {
        return NodeParentOneChild { val, child_idx: None, parent_idx: None }
    }

    #[inline(always)]
    pub fn new_full(val: T, child_idx: NodeIndex, parent_idx: NodeIndex) -> NodeParentOneChild<T> {
        return NodeParentOneChild { val, child_idx, parent_idx }
    }

    #[inline(always)]
    pub fn val_ref<'a>(&'a self) -> &'a T {
        return &self.val
    }

    #[inline(always)]
    pub fn val_mut<'a>(&'a mut self) -> &'a mut T {
        return &mut self.val
    }

    pub fn child_mut<'a>(&self, arena: &'a mut VecArena<Self>) -> Option<&'a mut Self> {
        match self.child_idx {
            Some(idx) => arena.get_mut(idx),
            None => None
        }
    }

    pub fn child_ref<'a>(&self, arena: &'a VecArena<Self>) -> Option<&'a Self> {
        match self.child_idx {
            Some(idx) => arena.get_ref(idx),
            None => None
        }
    }

    pub fn parent_mut<'a>(&self, arena: &'a mut VecArena<Self>) -> Option<&'a mut Self> {
        match self.parent_idx {
            Some(idx) => arena.get_mut(idx),
            None => None
        }
    }

    pub fn parent_ref<'a>(&self, arena: &'a VecArena<Self>) -> Option<&'a Self> {
        match self.parent_idx {
            Some(idx) => arena.get_ref(idx),
            None => None
        }
    }
}

pub struct NodeTwoChildren<T> {
    pub val: T,
    pub left_child_idx: NodeIndex,
    pub right_child_idx: NodeIndex
}

impl<T> NodeTwoChildren<T> {
    #[inline(always)]
    pub fn new(val: T) -> NodeTwoChildren<T> {
        return NodeTwoChildren { val, left_child_idx: None, right_child_idx: None }
    }

    #[inline(always)]
    pub fn new_full(val: T, left_child_idx: NodeIndex, right_child_idx: NodeIndex) -> NodeTwoChildren<T> {
        return NodeTwoChildren { val, left_child_idx, right_child_idx }
    }

    #[inline(always)]
    pub fn val_ref<'a>(&'a self) -> &'a T {
        return &self.val
    }

    #[inline(always)]
    pub fn val_mut<'a>(&'a mut self) -> &'a mut T {
        return &mut self.val
    }

    pub fn left_child_mut<'a>(&self, arena: &'a mut VecArena<Self>) -> Option<&'a mut Self> {
        match self.left_child_idx {
            Some(idx) => arena.get_mut(idx),
            None => None
        }
    }

    pub fn left_child_ref<'a>(&self, arena: &'a VecArena<Self>) -> Option<&'a Self> {
        match self.left_child_idx {
            Some(idx) => arena.get_ref(idx),
            None => None
        }
    }

    pub fn right_child_mut<'a>(&self, arena: &'a mut VecArena<Self>) -> Option<&'a mut Self> {
        match self.right_child_idx {
            Some(idx) => arena.get_mut(idx),
            None => None
        }
    }

    pub fn right_child_ref<'a>(&self, arena: &'a VecArena<Self>) -> Option<&'a Self> {
        match self.right_child_idx {
            Some(idx) => arena.get_ref(idx),
            None => None
        }
    }
}

pub struct NodeParentTwoChildren<T> {
    pub val: T,
    pub parent_idx: NodeIndex,
    pub left_child_idx: NodeIndex,
    pub right_child_idx: NodeIndex
}

impl<T> NodeParentTwoChildren<T> {
    #[inline(always)]
    pub fn new(val: T) -> NodeParentTwoChildren<T> {
        return NodeParentTwoChildren { val, left_child_idx: None, right_child_idx: None, parent_idx: None }
    }

    #[inline(always)]
    pub fn new_full(val: T, parent: NodeIndex, left_child: NodeIndex, right_child: NodeIndex) -> NodeParentTwoChildren<T> {
        return NodeParentTwoChildren { val, left_child_idx: left_child, right_child_idx: right_child, parent_idx: parent }
    }

    #[inline(always)]
    pub fn val_ref<'a>(&'a self) -> &'a T {
        return &self.val
    }

    #[inline(always)]
    pub fn val_mut<'a>(&'a mut self) -> &'a mut T {
        return &mut self.val
    }

    pub fn left_child_mut<'a>(&self, arena: &'a mut VecArena<Self>) -> Option<&'a mut Self> {
        match self.left_child_idx {
            Some(idx) => arena.get_mut(idx),
            None => None
        }
    }

    pub fn left_child_ref<'a>(&self, arena: &'a VecArena<Self>) -> Option<&'a Self> {
        match self.left_child_idx {
            Some(idx) => arena.get_ref(idx),
            None => None
        }
    }

    pub fn right_child_mut<'a>(&self, arena: &'a mut VecArena<Self>) -> Option<&'a mut Self> {
        match self.right_child_idx {
            Some(idx) => arena.get_mut(idx),
            None => None
        }
    }

    pub fn right_child_ref<'a>(&self, arena: &'a VecArena<Self>) -> Option<&'a Self> {
        match self.right_child_idx {
            Some(idx) => arena.get_ref(idx),
            None => None
        }
    }

    pub fn parent_mut<'a>(&self, arena: &'a mut VecArena<Self>) -> Option<&'a mut Self> {
        match self.parent_idx {
            Some(idx) => arena.get_mut(idx),
            None => None
        }
    }

    pub fn parent_ref<'a>(&self, arena: &'a VecArena<Self>) -> Option<&'a Self> {
        match self.parent_idx {
            Some(idx) => arena.get_ref(idx),
            None => None
        }
    }
}
