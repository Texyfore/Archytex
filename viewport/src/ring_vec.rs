pub struct RingVec<T> {
    items: Vec<Option<T>>,
}

impl<T> RingVec<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, item: T) -> usize {
        let mut i = 0;
        while self.items[i].is_none() {
            i += 1;
        }
        self.items[i] = Some(item);
        i
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        self.items[index].take()
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
