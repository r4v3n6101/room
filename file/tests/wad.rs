const TEST_WAD_PATH: &str = "/usr/share/doom/doom1.wad";

#[test]
fn print_lump_names() {
    let file = std::fs::read(TEST_WAD_PATH).unwrap();
    let archive = file::wad::parser::file::Archive::parse(&file).unwrap();

    println!("Wad type: {:?}", archive.wtype());
    archive
        .lumps()
        .iter()
        .enumerate()
        .for_each(|(i, e)| println!("{}: {}", i, e.name()));
}
