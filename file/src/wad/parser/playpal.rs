use super::types::{OnlyResult, ParseResult};
use nom::{combinator::map_res, multi::count, number::complete::le_u8, sequence::tuple};
use std::convert::TryInto;

pub type Rgb = (u8, u8, u8);
pub type Pallete = [Rgb; 256];
pub type PlayPal = [Pallete; 14];

fn parse_pallete(i: &[u8]) -> ParseResult<Pallete> {
    map_res(count(tuple((le_u8, le_u8, le_u8)), 256), |res| {
        res.try_into()
    })(i)
}

pub fn parse_playpal(i: &[u8]) -> OnlyResult<PlayPal> {
    let (_, out) = map_res(count(parse_pallete, 14), |res| res.try_into())(i)?;
    Ok(out)
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_parse_playpal_successful() {
        let file = std::fs::read(env!("TEST_WAD")).expect("Error reading wad file");
        let archive =
            crate::wad::parser::file::Archive::parse(&file).expect("Wad file parser error");
        let playpal_lump = archive.get_by_name("PLAYPAL").expect("PLAYPAL not found");
        assert!(super::parse_pallete(playpal_lump.data).is_ok());
    }
}
