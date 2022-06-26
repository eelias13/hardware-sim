/*
use hardware_sim::{Circuit, Component, Connection, LookupTable};
use std::collections::HashMap;

#[test]
#[ignore]
fn and_from_nand() {
    let nand = LookupTable::new(
        vec![vec![true, true, true, false]],
        vec!["a", "b"],
        vec!["out"],
        "Nand",
    )
    .unwrap();
    let mut lut_map = HashMap::new();
    lut_map.insert("Nand".to_string(), nand);
    let circuit_map = HashMap::new();

    let mut and = Circuit::new(
        vec!["a", "b"],
        vec!["out"],
        "And",
        vec![
            Connection::new("Nand", vec![("a", "a"), ("b", "b")], vec![("out", "nand")]),
            Connection::new(
                "Nand",
                vec![("nand", "a"), ("nand", "b")],
                vec![("out", "out")],
            ),
        ],
        &lut_map,
        &circuit_map,
    )
    .unwrap();

    assert_eq!(and.get("out"), Ok(false));
}



#[test]
fn common_from_nand() {


    Chip::new(
        "Not",
        vec!["a"],
        vec!["out"],
        vec![Component::new(
            vec![("a", "a"), ("b", "a"), ("out", "out")],
            "Nand"
        )]
    ),
    Chip::new(
        "Or",
        vec!["a", "b"],
        vec!["out"],
        vec![
            Component::new(vec![("a", "a"), ("out", "not_a")], "Not"),
            Component::new(vec![("a", "b"), ("out", "not_b")], "Not"),
            Component::new(vec![("a", "not_a"), ("b", "not_b"), ("out", "out")], "Nand"),
        ]
    ),
    Chip::new(
        "Xor",
        vec!["a", "b"],
        vec!["out"],
        vec![
            Component::new(vec![("a", "a"), ("out", "not_a")], "Not"),
            Component::new(vec![("a", "b"), ("out", "not_b")], "Not"),
            Component::new(vec![("a", "a"), ("b", "not_b"), ("out", "and_a")], "And"),
            Component::new(vec![("a", "not_a"), ("b", "b"), ("out", "and_b")], "And"),
            Component::new(vec![("a", "and_a"), ("b", "and_b"), ("out", "out")], "Or"),
        ]
    ),
])}
*/
