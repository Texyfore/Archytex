use std::iter::Flatten;

pub struct PinVec<T> {
    slots: Vec<Option<T>>,
}

impl<T> Default for PinVec<T> {
    fn default() -> Self {
        Self { slots: Vec::new() }
    }
}

impl<T> PinVec<T> {
    pub fn push(&mut self, value: T) -> usize {
        for (index, slot) in self.slots.iter_mut().enumerate() {
            if slot.is_none() {
                *slot = Some(value);
                return index;
            }
        }

        self.slots.push(Some(value));
        self.slots.len() - 1
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index < self.slots.len() {
            self.slots[index].take()
        } else {
            None
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if let Some(slot) = self.slots.get(index) {
            slot.as_ref()
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if let Some(slot) = self.slots.get_mut(index) {
            slot.as_mut()
        } else {
            None
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            flatten: self.slots.iter().flatten(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            flatten: self.slots.iter_mut().flatten(),
        }
    }
}

impl<T> IntoIterator for PinVec<T> {
    type Item = T;

    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            slots: self.slots,
            ptr: 0,
        }
    }
}

pub struct Iter<'a, T> {
    flatten: Flatten<std::slice::Iter<'a, Option<T>>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.flatten.next()
    }
}

pub struct IterMut<'a, T> {
    flatten: Flatten<std::slice::IterMut<'a, Option<T>>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.flatten.next()
    }
}

pub struct IntoIter<T> {
    slots: Vec<Option<T>>,
    ptr: usize,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        while self.ptr < self.slots.len() {
            if self.slots[self.ptr].is_none() {
                self.ptr += 1;
            } else {
                return self.slots[self.ptr].take();
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::PinVec;

    #[test]
    fn push() {
        let mut vec = PinVec::default();
        vec.push(());
        assert_eq!(vec.slots, vec![Some(())])
    }

    #[test]
    fn remove() {
        let mut vec = PinVec {
            slots: vec![Some(())],
        };

        assert_eq!(vec.remove(0), Some(()));
        assert_eq!(vec.remove(0), None);
    }

    #[test]
    fn get() {
        let vec1 = PinVec::<()>::default();
        assert_eq!(vec1.get(0), None);

        let vec2 = PinVec {
            slots: vec![Some(())],
        };

        assert_eq!(vec2.get(0), Some(&()));
    }

    #[test]
    fn get_mut() {
        let mut vec1 = PinVec::<()>::default();
        assert_eq!(vec1.get_mut(0), None);

        let mut vec2 = PinVec {
            slots: vec![Some(())],
        };

        assert_eq!(vec2.get_mut(0), Some(&mut ()));
    }

    #[test]
    fn iter() {
        let vec = PinVec {
            slots: vec![
                Some(0),
                None,
                Some(1),
                None,
                None,
                Some(2),
                Some(3),
                Some(4),
                None,
                None,
            ],
        };

        assert_eq!(vec.iter().collect::<Vec<_>>(), vec![&0, &1, &2, &3, &4]);
    }

    #[test]
    fn iter_mut() {
        let mut vec = PinVec {
            slots: vec![
                Some(0),
                None,
                Some(1),
                None,
                None,
                Some(2),
                Some(3),
                Some(4),
                None,
                None,
            ],
        };

        vec.iter_mut().for_each(|x| *x += 1);

        assert_eq!(
            vec.slots,
            vec![
                Some(1),
                None,
                Some(2),
                None,
                None,
                Some(3),
                Some(4),
                Some(5),
                None,
                None
            ]
        );
    }

    #[test]
    fn into_iter() {
        let vec = PinVec {
            slots: vec![Some(())],
        };

        assert_eq!(vec.into_iter().collect::<Vec<_>>(), vec![()]);
    }
}
