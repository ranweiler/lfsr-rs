use bit_vec::BitVec;
use num::pow;
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

    pub fn output(&self) -> bool {
        let len = self.state.len();
        self.state[len - 1]
    }

    pub fn step(&mut self) {
        let b = self.feedback_bit();

        self.shift();

        self.state.set(0, b);
    }

    fn shift(&mut self) {
        lfsr::shift(&mut self.state);
    }

    fn feedback_bit(&mut self) -> bool {
        self.taps
            .iter()
            .map(|&i| self.state[i-1])
            .fold(false, |acc, b| acc ^ b)
    }

    pub fn iter(&mut self) -> LFSRIter {
        LFSRIter { lfsr: self }
    }

    fn lsbyte(&self) -> u8 {
        self.state
            .iter()
            .take(8)
            .enumerate()
            .fold(0, |acc, (i, b)| acc + (b as u8) * pow(2, i))
    }

    pub fn bytes(&mut self) -> LFSRByteIter {
        if self.state.len() < 8 {
            panic!("LFSR must have length at least 8 to iterate bytes");
        }

        LFSRByteIter { lfsr: self }
    }
}

pub struct LFSRIter<'a> {
    lfsr: &'a mut FibonacciLFSR,
}

impl<'a> Iterator for LFSRIter<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        let r = self.lfsr.state.get(0);
        self.lfsr.step();
        r
    }
}

pub struct LFSRByteIter<'a> {
    lfsr: &'a mut FibonacciLFSR,
}

impl<'a> Iterator for LFSRByteIter<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        let byte = self.lfsr.lsbyte();
        self.lfsr.step();
        Some(byte)
    }
}
