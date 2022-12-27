use std::collections::VecDeque;
use std::ops::Deref;

#[derive(Default, Clone)]
pub struct SortedVec<T: Ord>(Vec<T>);

impl<T: Ord> Deref for SortedVec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Ord> SortedVec<T> {
    pub fn insert(&mut self, elem: T) {
        let idx = match self.0.binary_search(&elem) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };
        self.0.insert(idx, elem);
    }

    pub fn remove(&mut self, elem: &T) {
        while let Ok(idx) = self.0.binary_search(elem) {
            self.0.remove(idx);
        }
    }
}

impl<T: Ord> FromIterator<T> for SortedVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut v: Vec<_> = iter.into_iter().collect();
        v.sort();
        Self(v)
    }
}

#[derive(Default, Clone)]
pub struct SortedVecDeque<T: Ord>(VecDeque<T>);

impl<T: Ord> Deref for SortedVecDeque<T> {
    type Target = VecDeque<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Ord> SortedVecDeque<T> {
    pub fn insert(&mut self, elem: T) {
        let idx = match self.0.binary_search(&elem) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };
        self.0.insert(idx, elem);
    }

    pub fn remove(&mut self, elem: &T) {
        while let Ok(idx) = self.0.binary_search(elem) {
            self.0.remove(idx);
        }
    }
}

impl<T: Ord> FromIterator<T> for SortedVecDeque<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut v: Vec<_> = iter.into_iter().collect();
        v.sort();
        Self(VecDeque::from(v))
    }
}
