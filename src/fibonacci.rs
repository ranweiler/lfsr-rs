use bit_vec::BitVec;
use std::slice::Iter;
use std::iter::FromIterator;
use lfsr;

#[derive(PartialEq, Debug)]
pub struct FibonacciLFSR {
    state: BitVec,
    taps: Vec<usize>,
}

impl FibonacciLFSR {
    fn shift(&mut self) {
        lfsr::shift(&mut self.state);
    }

    fn feedback_bit(&mut self) -> bool {
        self.taps
            .iter()
            .map(|&i| self.state[i - 1])
            .fold(false, |acc, b| acc ^ b)
    }

    pub fn iter(&mut self) -> lfsr::Iter<FibonacciLFSR> {
        lfsr::Iter { lfsr: self }
    }
}

impl lfsr::LFSR for FibonacciLFSR {
    fn output(&self) -> bool {
        let len = self.state.len();
        self.state[len - 1]
    }

    fn step(&mut self) {
        let b = self.feedback_bit();

        self.shift();

        self.state.set(0, b);
    }
}

impl IntoIterator for FibonacciLFSR {
    type Item = bool;
    type IntoIter = lfsr::IntoIter<FibonacciLFSR>;

    fn into_iter(self) -> Self::IntoIter {
        lfsr::IntoIter { lfsr: self }
    }
}

impl<'a> IntoIterator for &'a mut FibonacciLFSR {
    type Item = bool;
    type IntoIter = lfsr::Iter<'a, FibonacciLFSR>;

    fn into_iter(self) -> Self::IntoIter {
        lfsr::Iter { lfsr: self }
    }
}

impl FromIterator<usize> for FibonacciLFSR {
    fn from_iter<T>(source: T) -> Self where T: IntoIterator<Item=usize> {
        let mut taps: Vec<usize> = source.into_iter().map(|t| t).collect();
        taps.sort();
        taps.dedup();

        let &len = taps.iter().max().unwrap();
        let state = BitVec::from_elem(len, true);

        FibonacciLFSR { state: state, taps: taps, }
    }
}
