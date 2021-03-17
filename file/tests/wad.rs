const TEST_WAD_PATH: &str = "/usr/share/doom/doom1.wad";

#[test]
fn print_lump_names() {
    let file = std::fs::read(TEST_WAD_PATH).unwrap();
    let archive = file::wad::Archive::parse(&file).unwrap();

    println!("Wad type: {:?}", archive.wtype());
    archive
        .iter_with_indices()
        .for_each(|(i, e)| println!("{}: {}", i, e.name()));
}
