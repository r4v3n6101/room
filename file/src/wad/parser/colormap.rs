use nom::{combinator::map_res, multi::count, number::complete::le_u8};
use std::convert::TryInto;

type Input<'a> = &'a [u8];
type ParseError<'a> = nom::error::VerboseError<Input<'a>>;
type ParseResult<'a, O> = nom::IResult<Input<'a>, O, ParseError<'a>>;
type OnlyResult<'a, O> = Result<O, nom::Err<ParseError<'a>>>;

pub type Colormap = [u8; 256];
pub type Colormaps = [Colormap; 34];

fn parse_colormap(i: &[u8]) -> ParseResult<Colormap> {
    map_res(count(le_u8, 256), |res| res.try_into())(i)
}

pub fn parse_colormaps(i: &[u8]) -> OnlyResult<Colormaps> {
    let (_, out) = map_res(count(parse_colormap, 34), |res| res.try_into())(i)?;
    Ok(out)
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_parse_colormaps_successful() {
        let file = std::fs::read(env!("TEST_WAD")).expect("Error reading wad file");
        let archive =
            crate::wad::parser::file::Archive::parse(&file).expect("Wad file parser error");
        let colormap_lump = archive.get_by_name("COLORMAP").expect("COLORMAP not found");
        assert!(super::parse_colormaps(colormap_lump.data).is_ok());
    }
}
