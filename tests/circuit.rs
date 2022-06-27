use hardware_sim::{ChipDef, Circuit, ComponentDef, LookupTable};
use std::collections::HashMap;

#[test]
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

    let and_def = ChipDef::new(
        "And",
        vec!["a", "b"],
        vec!["out"],
        vec![
            ComponentDef::new(vec![("a", "a"), ("b", "b")], vec![("out", "nand")], "Nand"),
            ComponentDef::new(
                vec![("a", "nand"), ("b", "nand")],
                vec![("out", "out")],
                "Nand",
            ),
        ],
    );

    let mut and = Circuit::new(and_def, lut_map).unwrap();

    assert_eq!(and.get("out"), Ok(false));
    assert_eq!(and.set("a", true), Ok(()));
    assert_eq!(and.get("out"), Ok(false));
    assert_eq!(and.set("b", true), Ok(()));
    assert_eq!(and.get("out"), Ok(true));
    assert_eq!(and.set("a", false), Ok(()));
    assert_eq!(and.get("out"), Ok(false));
}

/*
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
    );


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
