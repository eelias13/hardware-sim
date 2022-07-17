use hardware_sim::{ChipDef, Circuit, LookupTable};
use std::collections::HashMap;

#[test]
fn nand() {
    let lut = LookupTable::new(
        vec![vec![true, true, true, false]],
        vec!["a", "b"],
        vec!["out"],
        "Nand",
    )
    .unwrap();

    let mut lut_map = HashMap::new();
    lut_map.insert("Nand".to_string(), lut);

    let def = ChipDef::new(
        "Nand",
        vec!["a", "b"],
        vec!["out"],
        vec![(vec![("a", "a"), ("b", "b")], vec![("out", "out")], "Nand")],
    );

    let mut circuit = Circuit::new(def, lut_map).unwrap();

    assert_eq!(circuit.get("out"), Ok(false));
    assert_eq!(circuit.tick(), Ok(()));
    assert_eq!(circuit.get("out"), Ok(true));

    assert_eq!(circuit.set("a", true), Ok(()));
    assert_eq!(circuit.tick(), Ok(()));
    assert_eq!(circuit.get("out"), Ok(true));

    assert_eq!(circuit.set("b", true), Ok(()));
    assert_eq!(circuit.tick(), Ok(()));
    assert_eq!(circuit.get("out"), Ok(false));

    assert_eq!(circuit.set("a", false), Ok(()));
    assert_eq!(circuit.tick(), Ok(()));
    assert_eq!(circuit.get("out"), Ok(true));
}

#[test]
fn not_from_nand() {
    let lut = LookupTable::new(
        vec![vec![true, true, true, false]],
        vec!["a", "b"],
        vec!["out"],
        "Nand",
    )
    .unwrap();

    let mut lut_map = HashMap::new();
    lut_map.insert("Nand".to_string(), lut);

    let def = ChipDef::new(
        "Not",
        vec!["input"],
        vec!["output"],
        vec![(
            vec![("a", "input"), ("b", "input")],
            vec![("out", "output")],
            "Nand",
        )],
    );

    let mut circuit = Circuit::new(def, lut_map).unwrap();

    assert_eq!(circuit.tick(), Ok(()));
    assert_eq!(circuit.get("output"), Ok(true));

    assert_eq!(circuit.set("input", true), Ok(()));
    assert_eq!(circuit.tick(), Ok(()));
    assert_eq!(circuit.get("output"), Ok(false));
}

#[test]
fn and_from_nand() {
    let lut = LookupTable::new(
        vec![vec![true, true, true, false]],
        vec!["a", "b"],
        vec!["out"],
        "Nand",
    )
    .unwrap();

    let mut lut_map = HashMap::new();
    lut_map.insert("Nand".to_string(), lut);

    let def = ChipDef::new(
        "And",
        vec!["a", "b"],
        vec!["out"],
        vec![
            (
                vec![("a", "nand"), ("b", "nand")],
                vec![("out", "out")],
                "Nand",
            ),
            (vec![("a", "a"), ("b", "b")], vec![("out", "nand")], "Nand"),
        ],
    );

    let mut circuit = Circuit::new(def, lut_map).unwrap();

    assert_eq!(circuit.tick(), Ok(()));
    assert_eq!(circuit.tick(), Ok(()));
    assert_eq!(circuit.get("out"), Ok(false));
    assert_eq!(circuit.set("a", true), Ok(()));
    assert_eq!(circuit.tick(), Ok(()));
    assert_eq!(circuit.get("out"), Ok(false));
    assert_eq!(circuit.set("b", true), Ok(()));
    assert_eq!(circuit.tick(), Ok(()));
    assert_eq!(circuit.get("out"), Ok(true));
    assert_eq!(circuit.set("a", false), Ok(()));
    assert_eq!(circuit.tick(), Ok(()));
    assert_eq!(circuit.get("out"), Ok(false));
}
