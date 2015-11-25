use bit_vec::BitVec;
use std::slice::Iter;
use lfsr;
use lfsr::LFSR;

#[derive(PartialEq, Debug)]
pub struct GaloisLFSR {
    state: BitVec,
    mask: BitVec,
}

impl GaloisLFSR {
    pub fn from_iter(iter: Iter<usize>) -> Self {
        let &len = iter.clone().max().unwrap();

        let state = BitVec::from_elem(len, true);
        let mut mask = BitVec::from_elem(len, false);

        for &t in iter {
            if t < len {
                // Set the _counterpart_ mask index
                mask.set(len - t, true);
            }
        }
        mask.set(0, true);

        GaloisLFSR { state: state, mask: mask, }
    }

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

impl LFSR for GaloisLFSR {
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
