use bit_vec::BitVec;
use num::pow;
use std::slice::Iter;
use lfsr;

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

pub struct LFSRIter<'a> {
    lfsr: &'a mut GaloisLFSR,
}

impl<'a> Iterator for LFSRIter<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        use lfsr::LFSR;

        let o = self.lfsr.output();
        self.lfsr.step();
        Some(o)
    }
}

pub struct LFSRByteIter<'a> {
    lfsr: &'a mut GaloisLFSR,
}

impl<'a> Iterator for LFSRByteIter<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        use lfsr::LFSR;

        let byte = self.lfsr.lsbyte();
        self.lfsr.step();
        Some(byte)
    }
}
