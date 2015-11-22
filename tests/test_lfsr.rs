use lfsr::fibonacci::LFSR as FibonacciLFSR;
use lfsr::galois::LFSR as GaloisLFSR;

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
