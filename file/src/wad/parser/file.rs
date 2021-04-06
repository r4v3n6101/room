use std::collections::HashMap;

use super::{
    name::parse_name,
    types::{OnlyResult, ParseResult},
};
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
    pub name: &'a str,
    pub data: &'a [u8],
}

impl<'a> Lump<'a> {
    fn parse(i: &'a [u8], file: &'a [u8]) -> ParseResult<'a, Self> {
        let (i, (offset, disk_size, name)) = tuple((le_i32, le_i32, parse_name))(i)?;

        let (data_i, _) = take(offset as usize)(file)?;
        let (_, data) = take(disk_size as usize)(data_i)?;

        Ok((i, Self { name, data }))
    }

    pub const fn is_virtual(&self) -> bool {
        self.data.is_empty()
    }
}

pub struct Archive<'a> {
    pub wtype: Type,
    lumps: Vec<Lump<'a>>,
    named_lumps: HashMap<&'a str, usize>,
}

impl<'a> Archive<'a> {
    fn build_from_lumps(wtype: Type, lumps: Vec<Lump<'a>>) -> Archive<'a> {
        let named_lumps = lumps
            .iter()
            .map(|lump| lump.name)
            .enumerate()
            .map(|(i, name)| (name, i))
            .collect();
        Self {
            wtype,
            lumps,
            named_lumps,
        }
    }

    pub fn parse(file: &'a [u8]) -> OnlyResult<Self> {
        let (_, (wtype, dir_num, dir_offset)) = tuple((
            map(alt((tag(b"PWAD"), tag(b"IWAD"))), Type::from),
            le_i32,
            le_i32,
        ))(file)?;

        let (dir_i, _) = take(dir_offset as usize)(file)?;
        let (_, lumps): (_, Vec<_>) =
            map(count(|i| Lump::parse(i, file), dir_num as usize), |vec| {
                vec.into_iter().collect()
            })(dir_i)?;
        Ok(Self::build_from_lumps(wtype, lumps))
    }

    pub fn iter(&self) -> impl Iterator<Item = &Lump> {
        self.lumps.iter()
    }

    pub fn get_by_index(&self, i: usize) -> Option<&Lump> {
        self.lumps.get(i)
    }

    pub fn get_by_name<S: AsRef<str>>(&self, s: S) -> Option<&Lump> {
        self.named_lumps
            .get(s.as_ref())
            .and_then(|&index| self.get_by_index(index))
    }

    pub fn add_lump(&mut self, lump: Lump<'a>) {
        if let Some(&index) = self.named_lumps.get(lump.name) {
            self.lumps[index] = lump;
        } else {
            self.named_lumps.insert(lump.name, self.lumps.len() - 1);
            self.lumps.push(lump);
        }
    }
}

impl<'a> IntoIterator for Archive<'a> {
    type Item = Lump<'a>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.lumps.into_iter()
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
            .for_each(|(i, lump)| println!("Lump {} named {}", i, lump.name));
    }
}
