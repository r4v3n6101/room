use super::{
    name::parse_name,
    types::{OnlyResult, ParseResult},
};
use indexmap::IndexMap;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::map,
    multi::count,
    number::complete::le_i32,
    sequence::tuple,
};

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
    pub data: &'a [u8],
}

impl<'a> Lump<'a> {
    fn parse(i: &'a [u8], file: &'a [u8]) -> ParseResult<'a, (&'a str, Self)> {
        let (i, (offset, disk_size, name)) = tuple((le_i32, le_i32, parse_name))(i)?;

        let (data_i, _) = take(offset as usize)(file)?;
        let (_, data) = take(disk_size as usize)(data_i)?;

        Ok((i, (name, Self { data })))
    }

    pub const fn is_virtual(&self) -> bool {
        self.data.is_empty()
    }
}

pub struct Archive<'a> {
    pub wtype: Type,
    lumps: IndexMap<&'a str, Lump<'a>>,
}

impl<'a> Archive<'a> {
    pub fn parse(file: &'a [u8]) -> OnlyResult<Self> {
        let (_, (wtype, dir_num, dir_offset)) = tuple((
            map(alt((tag(b"PWAD"), tag(b"IWAD"))), Type::from),
            le_i32,
            le_i32,
        ))(file)?;

        let (dir_i, _) = take(dir_offset as usize)(file)?;
        let (_, lumps) = map(count(|i| Lump::parse(i, file), dir_num as usize), |vec| {
            vec.into_iter().collect()
        })(dir_i)?;
        Ok(Self { wtype, lumps })
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &Lump)> {
        self.lumps.iter().map(|(&s, l)| (s, l))
    }

    pub fn get_by_index(&self, i: usize) -> Option<(&str, &Lump)> {
        self.lumps.get_index(i).map(|(&s, l)| (s, l))
    }

    pub fn get_by_name<S: AsRef<str>>(&self, s: S) -> Option<&Lump> {
        self.lumps.get(s.as_ref())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn print_lump_names() {
        let file = std::fs::read(env!("TEST_WAD")).expect("Error reading wad file");
        let archive = super::Archive::parse(&file).expect("Wad file parser error");

        println!("Wad type: {:?}", archive.wtype);
        archive
            .iter()
            .enumerate()
            .for_each(|(i, (name, _))| println!("Lump {} named {}", i, name));
    }
}
