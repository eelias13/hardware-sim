use hardware_sim::{ChipDef, LookupTable};

#[test]
fn common_from_nand() {
    let nand = LookupTable::new(
        vec![vec![true, true, true, false]],
        vec!["a", "b"],
        vec!["out"],
        "Nand",
    )
    .unwrap();

    let not_def = ChipDef::new(
        "Not",
        vec!["a"],
        vec!["out"],
        vec![(vec![("a", "a"), ("b", "a")], vec![("out", "out")], "Nand")],
    );

    let or_def = ChipDef::new(
        "Or",
        vec!["a", "b"],
        vec!["out"],
        vec![
            (vec![("a", "a")], vec![("out", "not_a")], "Not"),
            (vec![("a", "b")], vec![("out", "not_b")], "Not"),
            (
                vec![("a", "not_a")],
                vec![("b", "not_b"), ("out", "out")],
                "Nand",
            ),
        ],
    );

    let xor_def = ChipDef::new(
        "Xor",
        vec!["a", "b"],
        vec!["out"],
        vec![
            (vec![("a", "a")], vec![("out", "not_a")], "Not"),
            (vec![("a", "b")], vec![("out", "not_b")], "Not"),
            (
                vec![("a", "a"), ("b", "not_b")],
                vec![("out", "and_a")],
                "And",
            ),
            (
                vec![("a", "not_a"), ("b", "b")],
                vec![("out", "and_b")],
                "And",
            ),
            (
                vec![("a", "and_a"), ("b", "and_b")],
                vec![("out", "out")],
                "Or",
            ),
        ],
    );
}
