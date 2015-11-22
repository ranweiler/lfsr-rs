use std::slice::Iter;
use bit_vec::BitVec;

#[cfg(test)] mod test;

#[derive(Clone, Debug, PartialEq)]
pub struct BinPoly {
    _coeff: BitVec,
}

impl BinPoly {
    pub fn from_coeff(iter: Iter<bool>) -> BinPoly {
        let coeff = iter.cloned().collect();

        BinPoly { _coeff: coeff }
    }

    pub fn degree(&self) -> usize {
        self._coeff
            .iter()
            .enumerate()
            .filter(|&(_i, b)| b)
            .map(|(i, _b)| i)
            .max()
            .unwrap_or(0)
    }

    pub fn coeff(&self, i: usize) -> bool {
        self._coeff.get(i).unwrap_or(false)
    }

    pub fn add(&mut self, other: &BinPoly) {
        if self._coeff.len() < other._coeff.len() {
            let diff = other._coeff.len() - self._coeff.len();
            self._coeff.grow(diff, false);
        }

        for i in 0 .. other._coeff.len() {
            let c = self.coeff(i);
            self._coeff.set(i, c ^ other.coeff(i));
        }
    }

    pub fn shift(&mut self, n: usize) {
        let d = self._coeff.len();
        self._coeff.grow(n, false);

        for i in (n .. (d + n)).rev() {
            let c = self._coeff[i - n];
            self._coeff.set(i, c);
        }

        for i in 0 .. n {
            self._coeff.set(i, false);
        }
    }

    pub fn into_taps(&self) -> Vec<usize> {
        let d = self.degree() + 1;

        self._coeff
            .iter()
            .skip(1)
            .take(d)
            .enumerate()
            .filter(|&(_i, b)| b)
            .map(|(i, _b)| i + 1)
            .collect()
    }
}
