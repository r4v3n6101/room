pub mod colormap;
pub mod file;
pub mod flat;
pub mod level;
pub mod name;
pub mod picture;
pub mod playpal;
pub mod pnames;
pub mod texture;

mod types {
    pub type Input<'a> = &'a [u8];
    pub type ParseError<'a> = nom::error::VerboseError<Input<'a>>;
    pub type ParseResult<'a, O> = nom::IResult<Input<'a>, O, ParseError<'a>>;
    pub type OnlyResult<'a, O> = Result<O, nom::Err<ParseError<'a>>>;
}
