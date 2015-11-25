use bit_vec::BitVec;

pub trait LFSR {
    fn output(&self) -> bool;
    fn step(&mut self);
}

pub fn shift(state: &mut BitVec) {
    let range = 1 .. state.len();

    for i in range.rev() {
        let pred = state[i - 1];
        state.set(i, pred);
    };

    state.set(0, false);
}

pub struct Iter<'a, T: 'a + LFSR> {
    pub lfsr: &'a mut T,
}

impl<'a, T: LFSR> Iterator for Iter<'a, T> {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        let o = self.lfsr.output();

        self.lfsr.step();

        Some(o)
    }
}

pub struct IntoIter<L: LFSR> {
    pub lfsr: L,
}

impl<T: LFSR> Iterator for IntoIter<T> {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        let o = self.lfsr.output();

        self.lfsr.step();

        Some(o)
    }
}

#[cfg(test)]
mod test {
    use bit_vec::BitVec;
    use super::shift;

    #[test]
    fn shift_works() {
        let ref mut bv = BitVec::from_elem(4, true);

        shift(bv);
        assert!(bv.eq_vec(&[false, true, true, true]));

        shift(bv);
        assert!(bv.eq_vec(&[false, false, true, true]));

        shift(bv);
        assert!(bv.eq_vec(&[false, false, false, true]));

        shift(bv);
        assert!(bv.eq_vec(&[false, false, false, false]));

        shift(bv);
        assert!(bv.eq_vec(&[false, false, false, false]));
    }
}
