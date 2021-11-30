use hardware_sim::Gate;

#[test]
fn and() {
    let mut and = Gate::new(vec!["a", "b"], vec![false, false, false, true], "and").unwrap();
    assert_eq!(and.eval(), false);

    and.set_input("a", true).unwrap();
    assert_eq!(and.eval(), false);

    and.set_input("a", false).unwrap();
    and.set_input("b", true).unwrap();
    assert_eq!(and.eval(), false);

    and.set_input("a", true).unwrap();
    assert_eq!(and.eval(), true);
}

#[test]
fn or() {
    let mut or = Gate::new(vec!["a", "b"], vec![false, true, true, true], "or").unwrap();
    assert_eq!(or.eval(), false);

    or.set_input("a", true).unwrap();
    assert_eq!(or.eval(), true);

    or.set_input("a", false).unwrap();
    or.set_input("b", true).unwrap();
    assert_eq!(or.eval(), true);

    or.set_input("a", true).unwrap();
    assert_eq!(or.eval(), true);
}

#[test]
fn xor() {
    let mut xor = Gate::new(vec!["a", "b"], vec![false, true, true, false], "xor").unwrap();
    assert_eq!(xor.eval(), false);

    xor.set_input("a", true).unwrap();
    assert_eq!(xor.eval(), true);

    xor.set_input("a", false).unwrap();
    xor.set_input("b", true).unwrap();
    assert_eq!(xor.eval(), true);

    xor.set_input("a", true).unwrap();
    assert_eq!(xor.eval(), false);
}

#[test]
fn not() {
    let mut not = Gate::new(vec!["a"], vec![true, false], "not").unwrap();
    assert_eq!(not.eval(), true);

    not.set_input("a", true).unwrap();
    assert_eq!(not.eval(), false);
}

#[test]
fn imply() {
    let mut imply = Gate::new(vec!["a", "b"], vec![false, true, false, false], "imply").unwrap();
    assert_eq!(imply.eval(), false);

    imply.set_input("a", true).unwrap();
    assert_eq!(imply.eval(), true);

    imply.set_input("a", false).unwrap();
    imply.set_input("b", true).unwrap();
    assert_eq!(imply.eval(), false);

    imply.set_input("a", true).unwrap();
    assert_eq!(imply.eval(), false);
}
