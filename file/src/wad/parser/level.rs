use super::name::parse_name;
use nom::{
    bytes::complete::take,
    combinator::verify,
    combinator::{map, map_res},
    multi::{count, many0},
    number::complete::{le_i16, le_u16},
    sequence::{delimited, tuple},
};

type Input<'a> = &'a [u8];
type ParseError<'a> = nom::error::VerboseError<Input<'a>>;
type ParseResult<'a, O> = nom::IResult<Input<'a>, O, ParseError<'a>>;
type OnlyResult<'a, O> = Result<O, nom::Err<ParseError<'a>>>;

pub struct BoundingBox {
    pub top: i16,
    pub bottom: i16,
    pub left: i16,
    pub right: i16,
}

impl BoundingBox {
    fn parse(i: &[u8]) -> ParseResult<Self> {
        let (i, (top, bottom, left, right)) = tuple((le_i16, le_i16, le_i16, le_i16))(i)?;
        Ok((
            i,
            Self {
                top,
                bottom,
                left,
                right,
            },
        ))
    }
}

pub struct Thing {
    pub x_pos: i16,
    pub y_pos: i16,
    pub angle: i16,
    pub ttype: i16,
    pub options: i16,
}

impl Thing {
    fn parse(i: &[u8]) -> ParseResult<Self> {
        let (i, (x_pos, y_pos, angle, ttype, options)) =
            tuple((le_i16, le_i16, le_i16, le_i16, le_i16))(i)?;
        Ok((
            i,
            Self {
                x_pos,
                y_pos,
                angle,
                ttype,
                options,
            },
        ))
    }
}

pub struct Things;
impl Things {
    pub fn parse(i: &[u8]) -> OnlyResult<Vec<Thing>> {
        let (_, things) = many0(Thing::parse)(i)?;
        Ok(things)
    }
}

pub struct Linedef {
    pub vertex_start: i16,
    pub vertex_end: i16,
    pub flags: i16,
    pub function: i16,
    pub tag: i16,
    pub sidedef_right: i16,
    pub sidedef_left: i16,
}

impl Linedef {
    fn parse(i: &[u8]) -> ParseResult<Self> {
        let (i, (vertex_start, vertex_end, flags, function, tag, sidedef_right, sidedef_left)) =
            tuple((le_i16, le_i16, le_i16, le_i16, le_i16, le_i16, le_i16))(i)?;
        Ok((
            i,
            Self {
                vertex_start,
                vertex_end,
                flags,
                function,
                tag,
                sidedef_right,
                sidedef_left,
            },
        ))
    }
}

pub struct Linedefs;
impl Linedefs {
    pub fn parse(i: &[u8]) -> OnlyResult<Vec<Linedef>> {
        let (_, linedefs) = many0(Linedef::parse)(i)?;
        Ok(linedefs)
    }
}

pub struct Sidedef<'a> {
    pub x_offset: i16,
    pub y_offset: i16,
    pub upper_texture: &'a str,
    pub lower_texture: &'a str,
    pub mid_texture: &'a str,
    pub sector_ref: i16,
}

impl<'a> Sidedef<'a> {
    fn parse(i: &'a [u8]) -> ParseResult<Self> {
        let (i, (x_offset, y_offset, upper_texture, lower_texture, mid_texture, sector_ref)) =
            tuple((le_i16, le_i16, parse_name, parse_name, parse_name, le_i16))(i)?;
        Ok((
            i,
            Self {
                x_offset,
                y_offset,
                upper_texture,
                lower_texture,
                mid_texture,
                sector_ref,
            },
        ))
    }
}

pub struct Sidedefs;
impl Sidedefs {
    pub fn parse(i: &[u8]) -> OnlyResult<Vec<Sidedef>> {
        let (_, sidedefs) = many0(Sidedef::parse)(i)?;
        Ok(sidedefs)
    }
}

pub type Vertex = (i16, i16);
pub fn parse_vertices(i: &[u8]) -> OnlyResult<Vec<Vertex>> {
    let (_, vertices) = many0(tuple((le_i16, le_i16)))(i)?;
    Ok(vertices)
}

pub struct Segment {
    pub vertex_start: i16,
    pub vertex_end: i16,
    pub bams: i16,
    pub line_num: i16,
    pub segside: i16,
    pub segoffset: i16,
}

impl Segment {
    fn parse(i: &[u8]) -> ParseResult<Self> {
        let (i, (vertex_start, vertex_end, bams, line_num, segside, segoffset)) =
            tuple((le_i16, le_i16, le_i16, le_i16, le_i16, le_i16))(i)?;
        Ok((
            i,
            Self {
                vertex_start,
                vertex_end,
                bams,
                line_num,
                segside,
                segoffset,
            },
        ))
    }
}

pub struct Segments;
impl Segments {
    pub fn parse(i: &[u8]) -> OnlyResult<Vec<Segment>> {
        let (_, segments) = many0(Segment::parse)(i)?;
        Ok(segments)
    }
}

pub struct SubSector {
    pub numsegs: i16,
    pub start_seg: i16,
}

impl SubSector {
    fn parse(i: &[u8]) -> ParseResult<Self> {
        let (i, (numsegs, start_seg)) = tuple((le_i16, le_i16))(i)?;
        Ok((i, Self { numsegs, start_seg }))
    }
}

pub struct SubSectors;
impl SubSectors {
    pub fn parse(i: &[u8]) -> OnlyResult<Vec<SubSector>> {
        let (_, ssectors) = many0(SubSector::parse)(i)?;
        Ok(ssectors)
    }
}

pub struct Node {
    pub x: i16,
    pub y: i16,
    pub dx: i16,
    pub dy: i16,
    pub bbox: [BoundingBox; 2],
    pub children: [u16; 2],
}

impl Node {
    fn parse(i: &[u8]) -> ParseResult<Self> {
        let (i, (x, y, dx, dy, bbox1, bbox2, child1, child2)) = tuple((
            le_i16,
            le_i16,
            le_i16,
            le_i16,
            BoundingBox::parse,
            BoundingBox::parse,
            le_u16,
            le_u16,
        ))(i)?;
        Ok((
            i,
            Self {
                x,
                y,
                dx,
                dy,
                bbox: [bbox1, bbox2],
                children: [child1, child2],
            },
        ))
    }
}

pub struct Nodes;
impl Nodes {
    pub fn parse(i: &[u8]) -> OnlyResult<Vec<Node>> {
        let (_, nodes) = many0(Node::parse)(i)?;
        Ok(nodes)
    }
}

pub struct Sector<'a> {
    pub floor_height: i16,
    pub ceiling_height: i16,
    pub floor_pic: &'a str,
    pub ceiling_pic: &'a str,
    pub light_level: i16,
    pub special_sector: i16,
    pub tag: i16,
}

impl<'a> Sector<'a> {
    fn parse(i: &'a [u8]) -> ParseResult<Self> {
        let (
            i,
            (
                floor_height,
                ceiling_height,
                floor_pic,
                ceiling_pic,
                light_level,
                special_sector,
                tag,
            ),
        ) = tuple((
            le_i16, le_i16, parse_name, parse_name, le_i16, le_i16, le_i16,
        ))(i)?;
        Ok((
            i,
            Self {
                floor_height,
                ceiling_height,
                floor_pic,
                ceiling_pic,
                light_level,
                special_sector,
                tag,
            },
        ))
    }
}

pub struct Sectors;
impl Sectors {
    pub fn parse<'a>(i: &'a [u8]) -> OnlyResult<Vec<Sector<'a>>> {
        let (_, sectors) = many0(Sector::parse)(i)?;
        Ok(sectors)
    }
}

pub struct Blockmap {
    pub x_origin: i16,
    pub y_origin: i16,
    pub x_blocks: i16,
    pub y_blocks: i16,
    pub linedefs_num: Vec<Vec<i16>>,
}

impl Blockmap {
    pub fn parse(blockmap_i: &[u8]) -> OnlyResult<Self> {
        let (i, (x_origin, y_origin, x_blocks, y_blocks)) =
            tuple((le_i16, le_i16, le_i16, le_i16))(blockmap_i)?;
        let (_, linedefs_num) = map(
            count(
                map_res(le_u16, |offset| {
                    let (_, blocklist_i) = take::<_, _, ParseError>(offset as usize)(blockmap_i)?;
                    delimited(
                        verify(le_i16, |&x| x == 0),
                        many0(verify(le_i16, |&x| x != -1)),
                        verify(le_i16, |&x| x == -1),
                    )(blocklist_i)
                    .map(|(_, linedefs_num)| linedefs_num)
                }),
                (x_blocks * y_blocks) as usize,
            ),
            |x| x.into_iter().collect(),
        )(i)?;
        Ok(Self {
            x_origin,
            y_origin,
            x_blocks,
            y_blocks,
            linedefs_num,
        })
    }
}
