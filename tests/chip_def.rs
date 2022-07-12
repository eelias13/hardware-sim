use hardware_sim::{ChipDef, ComponentDef, LookupTable};

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
        vec![ComponentDef::new(
            vec![("a", "a"), ("b", "a")],
            vec![("out", "out")],
            "Nand",
        )],
    );

    let or_def = ChipDef::new(
        "Or",
        vec!["a", "b"],
        vec!["out"],
        vec![
            ComponentDef::new(vec![("a", "a")], vec![("out", "not_a")], "Not"),
            ComponentDef::new(vec![("a", "b")], vec![("out", "not_b")], "Not"),
            ComponentDef::new(
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
            ComponentDef::new(vec![("a", "a")], vec![("out", "not_a")], "Not"),
            ComponentDef::new(vec![("a", "b")], vec![("out", "not_b")], "Not"),
            ComponentDef::new(
                vec![("a", "a"), ("b", "not_b")],
                vec![("out", "and_a")],
                "And",
            ),
            ComponentDef::new(
                vec![("a", "not_a"), ("b", "b")],
                vec![("out", "and_b")],
                "And",
            ),
            ComponentDef::new(
                vec![("a", "and_a"), ("b", "and_b")],
                vec![("out", "out")],
                "Or",
            ),
        ],
    );
}
