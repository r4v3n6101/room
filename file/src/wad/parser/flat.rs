use std::{array::TryFromSliceError, convert::TryInto};

pub type Flat<'a> = &'a [u8; 64 * 64];

pub fn parse_flat<'a>(i: &'a [u8]) -> Result<Flat<'a>, TryFromSliceError> {
    i.try_into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_parse_flats_successful() {
        let file = std::fs::read(env!("TEST_WAD")).expect("Error reading wad file");
        let archive =
            crate::wad::parser::file::Archive::parse(&file).expect("Wad file parser error");
        let flat = archive
            .iter()
            .skip_while(|(name, _)| name != &"F_START")
            .filter(|(_, lump)| !lump.is_virtual())
            .next()
            .map(|(_, l)| l)
            .expect("Next flat to F_START not found");

        assert!(super::parse_flat(flat.data()).is_ok());
    }
}
