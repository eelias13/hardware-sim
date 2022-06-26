/*
use hardware_sim::{Component, LookupTable};

#[test]
fn nand() {
    let mut nand = LookupTable::new(
        vec![vec![true, true, true, false]],
        vec!["a", "b"],
        vec!["out"],
        "Nand",
    )
    .unwrap();

    assert_eq!(nand.get("out"), Ok(true));
    assert_eq!(nand.set("a", true), Ok(()));
    assert_eq!(nand.get("out"), Ok(true));
    assert_eq!(nand.set("a", false), Ok(()));
    assert_eq!(nand.set("b", true), Ok(()));
    assert_eq!(nand.get("out"), Ok(true));
    assert_eq!(nand.set("a", true), Ok(()));
    assert_eq!(nand.get("out"), Ok(false));
}

#[test]
fn common_3_in_1() {
    let mut common = LookupTable::new(
        vec![
            vec![false, false, false, true],
            vec![false, true, true, true],
            vec![false, true, true, false],
        ],
        vec!["a", "b"],
        vec!["and", "or", "xor"],
        "Common",
    )
    .unwrap();

    assert_eq!(common.get("and"), Ok(false));
    assert_eq!(common.get("or"), Ok(false));
    assert_eq!(common.get("xor"), Ok(false));
    //
    assert_eq!(common.set("a", true), Ok(()));
    //
    assert_eq!(common.get("and"), Ok(false));
    assert_eq!(common.get("or"), Ok(true));
    assert_eq!(common.get("xor"), Ok(true));
    //
    assert_eq!(common.set("a", false), Ok(()));
    assert_eq!(common.set("b", true), Ok(()));
    //
    assert_eq!(common.get("and"), Ok(false));
    assert_eq!(common.get("or"), Ok(true));
    assert_eq!(common.get("xor"), Ok(true));
    //
    assert_eq!(common.set("a", true), Ok(()));
    //
    assert_eq!(common.get("and"), Ok(true));
    assert_eq!(common.get("or"), Ok(true));
    assert_eq!(common.get("xor"), Ok(false));
}
*/
