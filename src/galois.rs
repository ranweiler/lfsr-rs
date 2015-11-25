use bit_vec::BitVec;
use std::slice::Iter;
use std::iter::FromIterator;
use lfsr;

#[derive(PartialEq, Debug)]
pub struct GaloisLFSR {
    state: BitVec,
    mask: BitVec,
}

impl GaloisLFSR {
    fn shift(&mut self) {
        lfsr::shift(&mut self.state);
    }

    fn feedback(&mut self) {
        for (i, mb) in self.mask.iter().enumerate() {
            let sb = self.state[i];
            self.state.set(i, mb ^ sb);
        }
    }

    pub fn iter(&mut self) -> lfsr::Iter<GaloisLFSR> {
        lfsr::Iter { lfsr: self }
    }
}

impl lfsr::LFSR for GaloisLFSR {
    fn output(&self) -> bool {
        let len = self.state.len();
        self.state[len - 1]
    }

    fn step(&mut self) {
        let output = self.output();

        self.shift();

        if output {
            self.feedback();
        }
    }
}

impl IntoIterator for GaloisLFSR {
    type Item = bool;
    type IntoIter = lfsr::IntoIter<GaloisLFSR>;

    fn into_iter(self) -> Self::IntoIter {
        lfsr::IntoIter { lfsr: self }
    }
}

impl<'a> IntoIterator for &'a mut GaloisLFSR {
    type Item = bool;
    type IntoIter = lfsr::Iter<'a, GaloisLFSR>;

    fn into_iter(self) -> Self::IntoIter {
        lfsr::Iter { lfsr: self }
    }
}

impl FromIterator<usize> for GaloisLFSR {
    fn from_iter<T>(source: T) -> Self where T: IntoIterator<Item=usize> {
        let mut taps: Vec<usize> = source.into_iter().map(|t| t).collect();
        taps.sort();
        taps.dedup();

        let &len = taps.iter().max().unwrap();

        let state = BitVec::from_elem(len, true);
        let mut mask = BitVec::from_elem(len, false);

        for &t in &taps {
            if t < len {
                // Set the _counterpart_ mask index
                mask.set(len - t, true);
            }
        }
        mask.set(0, true);

        GaloisLFSR { state: state, mask: mask, }
    }
}
