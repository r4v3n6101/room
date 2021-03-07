#[test]
fn abc() {
    let file = std::fs::read("/usr/share/doom/doom1.wad").unwrap();
    let archive = file::wad::Archive::parse(&file).unwrap();
    archive
        .iter_with_indices()
        .for_each(|(i, e)| println!("{}: {}", i, e.name()));
}
