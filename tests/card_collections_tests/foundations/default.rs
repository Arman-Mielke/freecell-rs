use freecell::Foundations;

#[test]
fn test_default() {
    let default: Foundations = Foundations::default();
    assert_eq!(Foundations::new(), default)
}
