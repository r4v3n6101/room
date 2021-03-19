use super::name::parse_name;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::map,
    multi::count,
    number::complete::le_i32,
    sequence::tuple,
};

type Input<'a> = &'a [u8];
type ParseError<'a> = nom::error::VerboseError<Input<'a>>;
type ParseResult<'a, O> = nom::IResult<Input<'a>, O, ParseError<'a>>;
type OnlyResult<'a, O> = Result<O, nom::Err<ParseError<'a>>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Type {
    IWAD,
    PWAD,
}

impl From<&[u8]> for Type {
    /// It's assumed that `i` is always correct due it's called from parser with pre-check of value
    fn from(i: &[u8]) -> Self {
        match i {
            b"PWAD" => Self::PWAD,
            b"IWAD" => Self::IWAD,
            _ => unreachable!(),
        }
    }
}

pub struct Lump<'a> {
    name: &'a str,
    data: &'a [u8],
}

impl<'a> Lump<'a> {
    fn parse(i: &'a [u8], file: &'a [u8]) -> ParseResult<'a, Self> {
        let (i, (offset, disk_size, name)) = tuple((
            map(le_i32, |x| x as usize),
            map(le_i32, |x| x as usize),
            parse_name,
        ))(i)?;

        let (data_i, _) = take(offset)(file)?;
        let (_, data) = take(disk_size)(data_i)?;

        Ok((i, Self { name, data }))
    }

    pub const fn name(&self) -> &str {
        self.name
    }

    pub const fn data(&self) -> &[u8] {
        self.data
    }

    pub const fn is_virtual(&self) -> bool {
        self.data.is_empty()
    }
}

pub struct Archive<'a> {
    wtype: Type,
    lumps: Vec<Lump<'a>>,
}

impl<'a> Archive<'a> {
    pub fn parse(file: &'a [u8]) -> OnlyResult<Self> {
        let (_, (wtype, dir_num, dir_offset)) = tuple((
            map(alt((tag(b"PWAD"), tag(b"IWAD"))), Type::from),
            map(le_i32, |x| x as usize),
            map(le_i32, |x| x as usize),
        ))(file)?;

        let (dir_i, _) = take(dir_offset)(file)?;
        let (_, lumps) = count(|i| Lump::parse(i, file), dir_num)(dir_i)?;
        Ok(Self { wtype, lumps })
    }

    pub const fn wtype(&self) -> Type {
        self.wtype
    }

    pub fn lumps(&self) -> &[Lump] {
        &self.lumps
    }

    pub fn get_by_name<S: AsRef<str>>(&self, s: S) -> Option<&Lump> {
        self.lumps.iter().find(|lump| lump.name() == s.as_ref())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn print_lump_names() {
        let file = std::fs::read(env!("TEST_WAD")).expect("Error reading wad file");
        let archive = super::Archive::parse(&file).expect("Wad file parser error");

        println!("Wad type: {:?}", archive.wtype());
        archive
            .lumps()
            .iter()
            .enumerate()
            .for_each(|(i, e)| println!("Lump {} named {}", i, e.name()));
    }
}
