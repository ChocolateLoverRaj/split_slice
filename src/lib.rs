#![no_std]
use core::fmt::Debug;

#[derive(Copy)]
pub struct SplitSlice<'a, T>(pub &'a [T], pub &'a [T]);

impl<T> SplitSlice<'_, T> {
    pub fn len(&self) -> usize {
        self.0.len() + self.1.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty() && self.1.is_empty()
    }
}

impl<T> Clone for SplitSlice<'_, T> {
    fn clone(&self) -> Self {
        Self(self.0, self.1)
    }
}

impl<T: Debug> Debug for SplitSlice<'_, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self.clone().into_iter()).finish()
    }
}

impl<'a, T> IntoIterator for SplitSlice<'a, T> {
    type IntoIter = SplitSliceIterator<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Self::IntoIter {
        SplitSliceIterator::new(self)
    }
}

pub struct SplitSliceIterator<'a, T> {
    split_slice: SplitSlice<'a, T>,
    position: usize,
}

impl<'a, T> SplitSliceIterator<'a, T> {
    fn new(split_slice: SplitSlice<'a, T>) -> Self {
        Self {
            split_slice,
            position: 0,
        }
    }
}

impl<'a, T> Iterator for SplitSliceIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position < self.split_slice.0.len() {
            let item = &self.split_slice.0[self.position];
            self.position += 1;
            Some(item)
        } else {
            let position_2 = self.position - self.split_slice.0.len();
            let item = self.split_slice.1.get(position_2)?;
            self.position += 1;
            Some(item)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl<T> ExactSizeIterator for SplitSliceIterator<'_, T> {
    fn len(&self) -> usize {
        self.split_slice.len() - self.position
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterate() {
        assert!(
            SplitSlice(&[1, 2, 3], &[4, 5])
                .into_iter()
                .copied()
                .eq([1, 2, 3, 4, 5])
        );
    }

    #[test]
    fn iterate_first_empty() {
        assert!(SplitSlice(&[], &[1, 2]).into_iter().copied().eq([1, 2]));
    }

    #[test]
    fn iterate_second_empty() {
        assert!(SplitSlice(&[1, 2], &[]).into_iter().copied().eq([1, 2]));
    }

    #[test]
    fn iterate_both_empty() {
        assert!(SplitSlice::<i32>(&[], &[]).into_iter().copied().eq([]));
    }

    #[test]
    fn len() {
        assert_eq!(SplitSlice(&[1, 2, 3, 4], &[5]).len(), 5);
    }
}
