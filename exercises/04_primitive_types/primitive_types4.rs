// primitive_types4.rs
//
// Get a slice out of Array a where the ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand
// for a hint.


#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let (first, second) = a.split_at(1);
    let (k, l) = second.split_at(3);

    let nice_slice = k;

    assert_eq!([2, 3, 4], nice_slice)
}
