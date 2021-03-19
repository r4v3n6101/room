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

#[test]
fn print_sidedef_textures() {
    let file = std::fs::read(TEST_WAD_PATH).unwrap();
    let archive = file::wad::parser::file::Archive::parse(&file).unwrap();
    let texture1_lump = archive
        .lumps()
        .iter()
        .find(|lump| lump.name() == "TEXTURE1")
        .expect("TEXTURE1 not found");
    let textures = file::wad::parser::texture::Textures::parse(texture1_lump.data()).unwrap();
    textures
        .iter()
        .for_each(|tex| println!("TEXTURE1 name: {}", tex.name()));
}
