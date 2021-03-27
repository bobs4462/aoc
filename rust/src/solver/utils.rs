use std::marker::PhantomData;
/// Permuation generator, using Narayana’s next single permutation algorithm
pub(crate) struct Permutator<'a, T: Ord + 'a> {
    // Actually I just wrote it for fun :)
    collection: *mut T,
    len: usize,
    permutaions: u128,
    factorial: u128,
    _marker: PhantomData<&'a mut T>,
}

#[allow(dead_code)]
impl<'a, T: Ord> Permutator<'a, T> {
    pub(crate) fn new(collection: &'a mut [T]) -> Self {
        collection.sort();
        let factorial = fact(collection.len() as u8);
        Permutator {
            collection: collection.as_mut_ptr(),
            len: collection.len(),
            permutaions: 0,
            factorial,
            _marker: PhantomData,
        }
    }

    #[inline]
    fn done(&self) -> bool {
        self.permutaions == self.factorial
    }
}
impl<'a, T: Ord + std::fmt::Debug> Iterator for Permutator<'a, T> {
    type Item = &'a [T];
    fn next(&mut self) -> Option<Self::Item> {
        if self.done() {
            return None;
        }
        let slice = unsafe { std::slice::from_raw_parts_mut(self.collection, self.len) };
        self.permutaions += 1;
        if self.permutaions == 1 {
            return Some(slice);
        }
        let mut k: usize = 0;
        let mut j: usize = 0;
        let mut prev: &T = slice.last().unwrap();
        for (i, t) in slice[..self.len - 1].iter().rev().enumerate() {
            if prev > t {
                k = self.len - i - 2;
                break;
            }
            prev = t
        }

        for (i, t) in slice[k + 1..].iter().enumerate() {
            if slice[k] < *t {
                j = i + k + 1;
            }
        }
        slice.swap(k, j);
        slice[k + 1..].reverse();
        Some(slice)
    }
}

fn fact(val: u8) -> u128 {
    let mut res = 1;
    for i in 1..=val {
        res *= i as u128;
    }
    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_permutator() {
        let mut data = [1, 2, 3];
        const RES: [[i32; 3]; 6] = [
            [1, 2, 3],
            [1, 3, 2],
            [2, 1, 3],
            [2, 3, 1],
            [3, 1, 2],
            [3, 2, 1],
        ];
        let permutator = super::Permutator::new(&mut data);
        for (i, p) in permutator.enumerate() {
            println!("#{}. {:?}", i, p);
            assert_eq!(RES[i], p);
        }
    }
}
