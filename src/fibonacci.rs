use bit_vec::BitVec;
use std::slice::Iter;
use lfsr;

#[derive(PartialEq, Debug)]
pub struct FibonacciLFSR {
    state: BitVec,
    taps: Vec<usize>,
}

impl FibonacciLFSR {
    pub fn from_iter(iter: Iter<usize>) -> Self {
        let &len = iter.clone().max().unwrap();

        let state = BitVec::from_elem(len, true);

        let mut taps: Vec<usize> = iter.map(|&t| t).collect();
        taps.sort();
        taps.dedup();

        FibonacciLFSR { state: state, taps: taps, }
    }

    fn shift(&mut self) {
        lfsr::shift(&mut self.state);
    }

    fn feedback_bit(&mut self) -> bool {
        self.taps
            .iter()
            .map(|&i| self.state[i - 1])
            .fold(false, |acc, b| acc ^ b)
    }

    pub fn iter(&mut self) -> lfsr::LFSRIter<FibonacciLFSR> {
        lfsr::LFSRIter { lfsr: self }
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
