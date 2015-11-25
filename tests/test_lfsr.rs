use lfsr::fibonacci::FibonacciLFSR;
use lfsr::galois::GaloisLFSR;

use support::eq_mod_rotation;

#[test]
fn iter_works_for_fibonacci() {
    iter_works_for!(FibonacciLFSR);
}

#[test]
fn iter_works_for_galois() {
    iter_works_for!(GaloisLFSR);
}

#[test]
fn same_connection_polynomial_produce_output_equal_mod_rotation() {
    let taps = vec![2, 3];

    let mut galois = GaloisLFSR::from_iter(taps.iter());
    let galois_output: Vec<bool> = galois.iter().take(7).collect();

    let mut fibonacci = FibonacciLFSR::from_iter(taps.iter());
    let fibonacci_output: Vec<bool> = fibonacci.iter().take(7).collect();

    assert!(eq_mod_rotation(&fibonacci_output, &galois_output));
}

#[test]
fn primitive_connection_polynomial_yields_a_maximum_sequence_for_fibonacci() {
    primitive_connection_polynomial_yields_a_maximum_sequence!(FibonacciLFSR);
}

#[test]
fn primitive_connection_polynomial_yields_a_maximum_sequence_for_galois() {
    primitive_connection_polynomial_yields_a_maximum_sequence!(GaloisLFSR);
}

#[test]
fn implements_iter_for_galois() {
    let taps = vec![2, 3];
    let mut lfsr = GaloisLFSR::from_iter(taps.iter());

    let output: Vec<bool> = lfsr.iter().take(8).collect();

    let expected = vec![true, false, false, true, false, true, true, true];

    assert!(eq_mod_rotation(&output, &expected))
}

#[test]
fn implements_into_iterator_for_galois() {
    let taps = vec![2, 3];
    let lfsr = GaloisLFSR::from_iter(taps.iter());

    let mut counter = 0;
    for _b in lfsr {
        if counter == 3 {
            break;
        }
        counter += 1;
    }
}

#[test]
fn implements_into_iterator_for_mut_refs_for_galois() {
    use lfsr::LFSR;

    let taps = vec![2, 3];
    let mut lfsr = GaloisLFSR::from_iter(taps.iter());

    let mut counter = 0;
    for _b in &mut lfsr {
        if counter == 3 {
            break;
        }
        counter += 1;
    }

    lfsr.step();
}

#[test]
fn implements_iter_for_fibonacci() {
    let taps = vec![2, 3];
    let mut lfsr = FibonacciLFSR::from_iter(taps.iter());

    let output: Vec<bool> = lfsr.iter().take(8).collect();

    let expected = vec![true, false, false, true, false, true, true, true];

    assert!(eq_mod_rotation(&output, &expected))
}

#[test]
fn implements_into_iterator_for_fibonacci() {
    let taps = vec![2, 3];
    let lfsr = FibonacciLFSR::from_iter(taps.iter());

    let mut counter = 0;
    for _b in lfsr {
        if counter == 3 {
            break;
        }
        counter += 1;
    }
}

#[test]
fn implements_into_iterator_for_mut_refs_for_fibonacci() {
    use lfsr::LFSR;

    let taps = vec![2, 3];
    let mut lfsr = FibonacciLFSR::from_iter(taps.iter());

    let mut counter = 0;
    for _b in &mut lfsr {
        if counter == 3 {
            break;
        }
        counter += 1;
    }

    lfsr.step();
}
