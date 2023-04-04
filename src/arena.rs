
pub struct VecArena<T> {
    data: Vec<T>,
    open_indexes: Vec<usize>,
}

impl<T> VecArena<T> {
    pub fn new() -> VecArena<T> {
        return VecArena { 
            data: Vec::new(), 
            open_indexes: Vec::new(), 
        };
    }

    pub fn with_capacity(capacity: usize) -> VecArena<T> {
        return VecArena { 
            data: Vec::with_capacity(capacity), 
            open_indexes: Vec::new(), 
        };
    }

    pub fn insert(&mut self, value: T) -> usize {
        match self.open_indexes.pop() {
            Some(index) => {
                self.data[index] = value;
                return index
            },
            None => {
                self.data.push(value);
                return self.data.len() - 1;
            }
        }
    }

    #[inline(always)]
    pub fn delete(&mut self, index: usize) {
        self.open_indexes.push(index);
    }

    #[inline(always)]
    pub fn get_ref<'a>(&'a self, index: usize) -> Option<&'a T> {
        if index >= self.data.len() {
            return None
        }
        return Some(&self.data[index]);
    }

    #[inline(always)]
    pub fn get_mut<'a>(&'a mut self, index: usize) -> Option<&'a mut T> {
        if index >= self.data.len() {
            return None
        }
        return Some(&mut self.data[index]);
    }
}