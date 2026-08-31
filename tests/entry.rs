fn resolve(path: &str) -> std::path::PathBuf {
    let ui = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../tests/ui")
        .canonicalize()
        .unwrap();
    ui.join(path)
}

#[test]
fn empty() {
    let t = trybuild::TestCases::new();
    t.pass(resolve("empty.rs"));
}

#[test]
fn duplicate_grant() {
    let t = trybuild::TestCases::new();
    t.compile_fail(resolve("duplicate-grant.rs"));
}

#[test]
fn grant() {
    let t = trybuild::TestCases::new();
    t.pass(resolve("grant.rs"));
}

#[test]
fn grant_struct() {
    let t = trybuild::TestCases::new();
    t.pass(resolve("grant-struct.rs"));
}

#[test]
fn grant_construct() {
    let t = trybuild::TestCases::new();
    t.compile_fail(resolve("grant-construct.rs"));
}
