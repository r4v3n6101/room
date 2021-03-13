use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_till},
    combinator::{map, map_res},
    multi::count,
    number::complete::le_i32,
    sequence::tuple,
};
use std::iter::Iterator;

const NAME_LEN: usize = 8;

type Input<'a> = &'a [u8];
type ParseError<'a> = nom::error::VerboseError<Input<'a>>;
type ParseResult<'a, O> = nom::IResult<Input<'a>, O, ParseError<'a>>;
type OnlyResult<'a, O> = Result<O, nom::Err<ParseError<'a>>>;

fn take_cstr(i: &[u8], size: usize) -> ParseResult<&str> {
    let (i, cstr) = take(size)(i)?;
    let (_, cstr) = map_res(take_till(|x| x == 0), std::str::from_utf8)(cstr)?;
    Ok((i, cstr))
}

#[derive(Clone, Copy, Debug)]
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

pub struct Entry<'a> {
    name: &'a str,
    data: &'a [u8],
}

impl<'a> Entry<'a> {
    fn parse(i: &'a [u8], file: &'a [u8]) -> ParseResult<'a, Self> {
        let (i, (offset, disk_size, name)) = tuple((
            map(le_i32, |x| x as usize),
            map(le_i32, |x| x as usize),
            |i| take_cstr(i, NAME_LEN),
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
}

pub struct Archive<'a> {
    wtype: Type,
    entries: Vec<Entry<'a>>,
}

impl<'a> Archive<'a> {
    pub fn parse(file: &'a [u8]) -> OnlyResult<Self> {
        let (_, (wtype, dir_num, dir_offset)) = tuple((
            map(alt((tag(b"PWAD"), tag(b"IWAD"))), Type::from),
            map(le_i32, |x| x as usize),
            map(le_i32, |x| x as usize),
        ))(file)?;

        let (dir_i, _) = take(dir_offset)(file)?;
        let (_, entries) = count(|i| Entry::parse(i, file), dir_num)(dir_i)?;
        Ok(Self { wtype, entries })
    }

    pub const fn wtype(&self) -> Type {
        self.wtype
    }

    pub fn iter_with_indices(&self) -> impl Iterator<Item = (usize, &Entry)> {
        self.entries.iter().enumerate()
    }

    pub fn get_by_index(&self, index: usize) -> Option<&Entry> {
        self.entries.get(index)
    }

    pub fn get_by_name<S: AsRef<str>>(&self, name: S) -> Option<&Entry> {
        self.entries.iter().find(|e| e.name() == name.as_ref())
    }
}
