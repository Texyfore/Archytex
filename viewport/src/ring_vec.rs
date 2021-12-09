use std::ops::Index;

pub struct RingVec<T> {
    items: Vec<Option<T>>,
}

impl<T> RingVec<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            items: (0..capacity).map(|_| None).collect(),
        }
    }

    pub fn push(&mut self, item: T) -> usize {
        let mut i = 0;
        while self.items[i].is_some() {
            i += 1;
        }
        self.items[i] = Some(item);
        i
    }

    pub fn insert(&mut self, index: usize, item: T) -> Option<T> {
        let old = self.items[index].take();
        self.items[index] = Some(item);
        old
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        self.items[index].take()
    }

    pub fn has_element_at(&self, index: usize) -> bool {
        self.items[index].is_some()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.items[index].as_ref()
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.items[index].as_mut()
    }

    pub fn iter(&self) -> Iter<T> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.into_iter()
    }
}

impl<T> Index<usize> for RingVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.items[index].as_ref().unwrap()
    }
}

pub struct Iter<'a, T> {
    items: Vec<(usize, &'a T)>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = (usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.items.pop()
    }
}

pub struct IterMut<'a, T> {
    items: Vec<(usize, &'a mut T)>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = (usize, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        self.items.pop()
    }
}

impl<'a, T> IntoIterator for &'a RingVec<T> {
    type Item = (usize, &'a T);

    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            items: self
                .items
                .iter()
                .enumerate()
                .map(|(i, o)| o.as_ref().map(|o| (i, o)))
                .flatten()
                .rev()
                .collect(),
        }
    }
}

impl<'a, T> IntoIterator for &'a mut RingVec<T> {
    type Item = (usize, &'a mut T);

    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut {
            items: self
                .items
                .iter_mut()
                .enumerate()
                .map(|(i, o)| o.as_mut().map(|o| (i, o)))
                .flatten()
                .rev()
                .collect(),
        }
    }
}
