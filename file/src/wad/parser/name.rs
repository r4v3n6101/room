use nom::{
    bytes::complete::{take, take_till},
    combinator::map_res,
};

const NAME_LEN: usize = 8;

type Input<'a> = &'a [u8];
type ParseError<'a> = nom::error::VerboseError<Input<'a>>;
type ParseResult<'a, O> = nom::IResult<Input<'a>, O, ParseError<'a>>;

fn take_cstr(i: &[u8], size: usize) -> ParseResult<&str> {
    let (i, cstr) = take(size)(i)?;
    let (_, cstr) = map_res(take_till(|x| x == 0), std::str::from_utf8)(cstr)?;
    Ok((i, cstr))
}

pub fn parse_name(i: &[u8]) -> ParseResult<&str> {
    take_cstr(i, NAME_LEN)
}
