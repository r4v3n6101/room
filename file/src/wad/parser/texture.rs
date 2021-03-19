use super::name::parse_name;
use nom::{
    bytes::complete::take,
    combinator::{map, map_res},
    multi::length_count,
    number::complete::{le_i16, le_i32},
    sequence::tuple,
};

type Input<'a> = &'a [u8];
type ParseError<'a> = nom::error::VerboseError<Input<'a>>;
type ParseResult<'a, O> = nom::IResult<Input<'a>, O, ParseError<'a>>;
type OnlyResult<'a, O> = Result<O, nom::Err<ParseError<'a>>>;

pub struct PatchDescriptor {
    x_offset: i16,
    y_offset: i16,
    id: i16,
    stepdir: i16,
    colormap: i16,
}

impl PatchDescriptor {
    fn parse(i: &[u8]) -> ParseResult<Self> {
        let (i, (x_offset, y_offset, id, stepdir, colormap)) =
            tuple((le_i16, le_i16, le_i16, le_i16, le_i16))(i)?;
        Ok((
            i,
            Self {
                x_offset,
                y_offset,
                id,
                stepdir,
                colormap,
            },
        ))
    }

    pub const fn x_offset(&self) -> i16 {
        self.x_offset
    }

    pub const fn y_offset(&self) -> i16 {
        self.y_offset
    }

    pub const fn id(&self) -> i16 {
        self.id
    }

    pub const fn stepdir(&self) -> i16 {
        self.stepdir
    }

    pub const fn colormap(&self) -> i16 {
        self.colormap
    }
}

pub struct Texture<'a> {
    name: &'a str,
    width: i16,
    height: i16,
    patch_descriptors: Vec<PatchDescriptor>,
}

impl<'a> Texture<'a> {
    fn parse(i: &'a [u8]) -> OnlyResult<Self> {
        let (_, (name, _, _, width, height, _, _, patch_descriptors)) = tuple((
            parse_name,
            le_i16,
            le_i16,
            le_i16,
            le_i16,
            le_i16,
            le_i16,
            length_count(map(le_i16, |x| x as usize), PatchDescriptor::parse),
        ))(i)?;
        Ok(Self {
            name,
            width,
            height,
            patch_descriptors,
        })
    }

    pub const fn name(&self) -> &str {
        self.name
    }

    pub const fn width(&self) -> i16 {
        self.width
    }

    pub const fn height(&self) -> i16 {
        self.height
    }

    pub fn patch_descriptors(&self) -> &[PatchDescriptor] {
        &self.patch_descriptors
    }
}

pub struct Textures;

impl Textures {
    pub fn parse<'a>(i: &'a [u8]) -> OnlyResult<Vec<Texture<'a>>> {
        let (_, textures) = length_count(
            map(le_i32, |x| x as usize),
            map_res(map(le_i32, |x| x as usize), |offset| {
                let (tex_i, _) = take(offset)(i)?;
                Texture::parse(tex_i)
            }),
        )(i)?;
        Ok(textures)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn print_textures() {
        let file = std::fs::read(env!("TEST_WAD")).unwrap();
        let archive = crate::wad::parser::file::Archive::parse(&file).unwrap();
        let texture1_lump = archive
            .lumps()
            .iter()
            .find(|lump| lump.name() == "TEXTURE1")
            .expect("TEXTURE1 not found");
        let textures = super::Textures::parse(texture1_lump.data()).unwrap();
        textures
            .iter()
            .for_each(|tex| println!("TEXTURE1 name: {}", tex.name()));
    }
}
