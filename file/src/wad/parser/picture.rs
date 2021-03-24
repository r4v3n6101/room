use nom::{
    bytes::complete::take,
    combinator::{map_res, verify},
    multi::{count, many0},
    number::complete::{le_i16, le_i32, le_u8},
    sequence::tuple,
};

type Input<'a> = &'a [u8];
type ParseError<'a> = nom::error::VerboseError<Input<'a>>;
type ParseResult<'a, O> = nom::IResult<Input<'a>, O, ParseError<'a>>;
type OnlyResult<'a, O> = Result<O, nom::Err<ParseError<'a>>>;

struct Post {
    rowstart: u8,
    pixels: Vec<u8>,
}

impl Post {
    fn parse(i: &[u8]) -> ParseResult<Self> {
        let (i, (rowstart, num_pixels, _)) = tuple((le_u8, le_u8, le_u8))(i)?;
        let (i, (pixels, _)) = tuple((count(le_u8, num_pixels as usize), le_u8))(i)?;
        Ok((i, Self { rowstart, pixels }))
    }
}

pub struct Picture {
    pub width: i16,
    pub height: i16,
    pub left_offset: i16,
    pub top_offset: i16,
    columns: Vec<Vec<Post>>,
}

impl Picture {
    pub fn parse(lump_i: &[u8]) -> OnlyResult<Self> {
        let (i, (width, height, left_offset, top_offset)) =
            tuple((le_i16, le_i16, le_i16, le_i16))(lump_i)?;
        let (_, columns) = count(
            map_res(le_i32, |offset| {
                let (data_i, _) = take(offset as usize)(lump_i)?;
                many0(verify(Post::parse, |post| post.rowstart != 255))(data_i)
                    .map(|(_, post)| post)
            }),
            width as usize,
        )(i)?;
        Ok(Self {
            width,
            height,
            left_offset,
            top_offset,
            columns,
        })
    }

    pub fn into_matrix(self) -> Vec<Vec<u8>> {
        let (width, height) = (self.width as usize, self.height as usize);
        let mut output = vec![vec![!0; height]; width];
        for x in 0..width {
            let out_column = &mut output[x];
            let column = &self.columns[x];
            column.into_iter().for_each(|post| {
                let pixels = post.pixels.iter().cloned();
                let rowstart = post.rowstart as usize;
                out_column.splice(rowstart..rowstart + pixels.len(), pixels);
            })
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    #[test]
    fn save_pictures_to_disk() {
        let file = std::fs::read(env!("TEST_WAD")).expect("Error reading wad file");
        let output_dir = std::path::PathBuf::from_str(env!("IMG_OUTPUT_DIR"))
            .expect("Error getting output dir's path");
        let archive =
            crate::wad::parser::file::Archive::parse(&file).expect("Wad file parser error");
        let sprites = archive
            .iter()
            .skip_while(|(name, _)| name != &"S_START")
            .take_while(|(name, _)| name != &"S_END")
            .filter(|(_, lump)| !lump.is_virtual());

        let playpal_lump = archive.get_by_name("PLAYPAL").expect("PLAYPAL not found");
        let playpal = crate::wad::parser::playpal::parse_playpal(playpal_lump.data)
            .expect("Error parsing PLAYPAL");
        let playpal = playpal[0]; // Use only first pallete

        sprites.into_iter().for_each(|(name, lump)| {
            let image =
                super::Picture::parse(lump.data).expect(format!("Error parsing {}", name).as_str());
            let matrix = image.into_matrix();

            let mut img_buf = image::ImageBuffer::new(matrix.len() as u32, matrix[0].len() as u32);
            for x in 0..matrix.len() {
                for y in 0..matrix[0].len() {
                    let pixel = img_buf.get_pixel_mut(x as u32, y as u32);
                    let index = matrix[x][y];
                    let color = if index == !0 {
                        (0, 0, 0, 0)
                    } else {
                        let color = playpal[index as usize];
                        (color.0, color.1, color.2, 0xFF)
                    };
                    *pixel = image::Rgba([color.0, color.1, color.2, color.3]);
                }
            }

            let output_path = output_dir.join(format!("{}.png", name));
            img_buf
                .save(&output_path)
                .expect(&format!("Error saving {}", name));
            println!(
                "Saved {}",
                output_path.to_str().expect("Error converting path to str")
            );
        });
    }
}
