use assert_approx_eq::assert_approx_eq;

#[test]
fn simple_password() {
    let password = "kq4zpz13";

    let analyzed = passwords::analyzer::analyze(password);

    assert_approx_eq!(62f64, passwords::scorer::score(&analyzed));
}

#[test]
fn strong_password() {
    let password = "ZYX[$BCkQB中文}%A_3456]  H(\rg";

    let analyzed = passwords::analyzer::analyze(password);

    assert_approx_eq!(100f64, passwords::scorer::score(&analyzed));
}

#[test]
fn common_password() {
    let password = "abc123";

    let analyzed = passwords::analyzer::analyze(password);

    if cfg!(feature = "common-password") {
        assert_approx_eq!(4.8f64, passwords::scorer::score(&analyzed)); // "abc123" is common, so the score is punitively the original divided by 5
    } else {
        assert_approx_eq!(24f64, passwords::scorer::score(&analyzed));
    }
}
