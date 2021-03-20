use super::file::Archive;
use std::{array::TryFromSliceError, convert::TryInto};

const FLAT_LUMP_BEGIN: &str = "F_START";
const FLAT_LUMP_END: &str = "F_END";

pub type Flat<'a> = &'a [u8; 64 * 64];

pub fn parse_flats<'a>(archive: &'a Archive) -> Result<Vec<Flat<'a>>, TryFromSliceError> {
    archive
        .lumps()
        .iter()
        .skip_while(|lump| lump.name() != FLAT_LUMP_BEGIN)
        .filter(|lump| !lump.is_virtual())
        .take_while(|lump| lump.name() != FLAT_LUMP_END)
        .map(|lump| lump.data().try_into())
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_parse_flats_successful() {
        let file = std::fs::read(env!("TEST_WAD")).expect("Error reading wad file");
        let archive = super::Archive::parse(&file).expect("Wad file parser error");

        assert!(super::parse_flats(&archive).is_ok());
    }
}
