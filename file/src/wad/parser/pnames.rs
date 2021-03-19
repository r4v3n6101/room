// TODO : merge into another parser
use super::name::parse_name;
use nom::{combinator::map, multi::length_count, number::complete::le_i32};

type Input<'a> = &'a [u8];
type ParseError<'a> = nom::error::VerboseError<Input<'a>>;
type OnlyResult<'a, O> = Result<O, nom::Err<ParseError<'a>>>;

pub fn parse_pnames(i: &[u8]) -> OnlyResult<Vec<&str>> {
    let (_, pnames) = length_count(map(le_i32, |x| x as usize), parse_name)(i)?;
    Ok(pnames)
}

#[cfg(test)]
mod tests {
    #[test]
    fn print_indexed_pnames() {
        let file = std::fs::read(env!("TEST_WAD")).expect("Error reading wad file");
        let archive =
            crate::wad::parser::file::Archive::parse(&file).expect("Wad File parser error");
        let pnames_lump = archive.get_by_name("PNAMES").expect("PNAMES not found");
        let pnames = super::parse_pnames(pnames_lump.data()).expect("Error parsing PNAMES");

        pnames
            .into_iter()
            .enumerate()
            .for_each(|(i, pname)| println!("PNAME {} named {}", i, pname));
    }
}