use std::{
    iter::{Enumerate, Peekable, chain, repeat_n},
    slice,
};

use crate::{ExtraAdapters, iterators::Compress};

pub struct Permutations<'s, T: Copy> {
    data: &'s [T],
    select: usize,
    iterators: Vec<Peekable<Compress<Enumerate<slice::Iter<'s, T>>, Vec<bool>>>>,
}

impl<'s, T: Copy> Permutations<'s, T> {
    pub(crate) fn new(data: &'s [T], select: Option<usize>) -> Self {
        let select = select.unwrap_or(data.len());

        assert!(select <= data.len());

        let iterators = (0..select)
            .map(|i| {
                data.iter()
                    .enumerate()
                    .compress(chain(repeat_n(false, i), repeat_n(true, data.len() - i)).collect())
                    .peekable()
            })
            .collect();

        Self {
            data,
            select,
            iterators,
        }
    }
}

impl<'s, T: Copy> Iterator for Permutations<'s, T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self
            .iterators
            .iter_mut()
            .map(|it| it.peek().map(|&(_, &v)| v))
            .collect();

        while let Some(_) = self.iterators.pop_if(|it| {
            it.next();
            it.peek().is_none()
        }) {}

        if self.iterators.is_empty() {
            self.iterators.push(
                self.data[..0]
                    .iter()
                    .enumerate()
                    .compress(vec![])
                    .peekable(),
            );
        } else {
            let mut mask = vec![true; self.data.len()];
            for it in &mut self.iterators {
                if let Some(&(i, _)) = it.peek() {
                    mask[i] = false;
                }
            }

            for _ in self.iterators.len()..self.select {
                let mut it = self
                    .data
                    .iter()
                    .enumerate()
                    .compress(mask.clone())
                    .peekable();

                if let Some(&(i, _)) = it.peek() {
                    mask[i] = false;
                }

                self.iterators.push(it);
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perm_4() {
        let data = [1, 2, 3, 4];

        [
            [1, 2, 3, 4],
            [1, 2, 4, 3],
            [1, 3, 2, 4],
            [1, 3, 4, 2],
            [1, 4, 2, 3],
            [1, 4, 3, 2],
            [2, 1, 3, 4],
            [2, 1, 4, 3],
            [2, 3, 1, 4],
            [2, 3, 4, 1],
            [2, 4, 1, 3],
            [2, 4, 3, 1],
            [3, 1, 2, 4],
            [3, 1, 4, 2],
            [3, 2, 1, 4],
            [3, 2, 4, 1],
            [3, 4, 1, 2],
            [3, 4, 2, 1],
            [4, 1, 2, 3],
            [4, 1, 3, 2],
            [4, 2, 1, 3],
            [4, 2, 3, 1],
            [4, 3, 1, 2],
            [4, 3, 2, 1],
        ]
        .iter()
        .zip(Permutations::new(&data, None))
        .for_each(|(expected, generated)| assert_eq!(generated.as_slice(), expected));
    }

    #[test]
    fn perm_5_3() {
        let data = [1, 2, 3, 4, 5];

        [
            [1, 2, 3],
            [1, 2, 4],
            [1, 2, 5],
            [1, 3, 2],
            [1, 3, 4],
            [1, 3, 5],
            [1, 4, 2],
            [1, 4, 3],
            [1, 4, 5],
            [1, 5, 2],
            [1, 5, 3],
            [1, 5, 4],
            [2, 1, 3],
            [2, 1, 4],
            [2, 1, 5],
            [2, 3, 1],
            [2, 3, 4],
            [2, 3, 5],
            [2, 4, 1],
            [2, 4, 3],
            [2, 4, 5],
            [2, 5, 1],
            [2, 5, 3],
            [2, 5, 4],
            [3, 1, 2],
            [3, 1, 4],
            [3, 1, 5],
            [3, 2, 1],
            [3, 2, 4],
            [3, 2, 5],
            [3, 4, 1],
            [3, 4, 2],
            [3, 4, 5],
            [3, 5, 1],
            [3, 5, 2],
            [3, 5, 4],
            [4, 1, 2],
            [4, 1, 3],
            [4, 1, 5],
            [4, 2, 1],
            [4, 2, 3],
            [4, 2, 5],
            [4, 3, 1],
            [4, 3, 2],
            [4, 3, 5],
            [4, 5, 1],
            [4, 5, 2],
            [4, 5, 3],
            [5, 1, 2],
            [5, 1, 3],
            [5, 1, 4],
            [5, 2, 1],
            [5, 2, 3],
            [5, 2, 4],
            [5, 3, 1],
            [5, 3, 2],
            [5, 3, 4],
            [5, 4, 1],
            [5, 4, 2],
            [5, 4, 3],
        ]
        .iter()
        .zip(Permutations::new(&data, Some(3)))
        .for_each(|(expected, generated)| assert_eq!(generated.as_slice(), expected));
    }
}
