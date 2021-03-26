use super::{
    name::parse_name,
    types::{OnlyResult, ParseResult},
};
use nom::{
    bytes::complete::take,
    combinator::{map, map_res},
    multi::length_count,
    number::complete::{le_i16, le_i32},
    sequence::tuple,
};

pub struct PatchDescriptor {
    pub x_offset: i16,
    pub y_offset: i16,
    pub id: i16,
    pub stepdir: i16,
    pub colormap: i16,
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
}

pub struct Texture<'a> {
    pub name: &'a str,
    pub width: i16,
    pub height: i16,
    pub patch_descriptors: Vec<PatchDescriptor>,
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
}

pub struct Textures;

impl Textures {
    pub fn parse<'a>(i: &'a [u8]) -> OnlyResult<Vec<Texture<'a>>> {
        let (_, textures) = length_count(
            map(le_i32, |x| x as usize),
            map_res(le_i32, |offset| {
                let (tex_i, _) = take(offset as usize)(i)?;
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
        let file = std::fs::read(env!("TEST_WAD")).expect("Error reading wad file");
        let archive =
            crate::wad::parser::file::Archive::parse(&file).expect("Wad file parser error");
        let texture1_lump = archive.get_by_name("TEXTURE1").expect("TEXTURE1 not found");
        let textures = super::Textures::parse(texture1_lump.data).expect("Error parsing TEXTURE1");
        textures
            .iter()
            .for_each(|tex| println!("TEXTURE1 name: {}", tex.name));
    }
}
