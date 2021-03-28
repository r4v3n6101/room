use super::{
    file::Archive,
    name::parse_name,
    types::{OnlyResult, ParseResult},
};
use nom::{
    multi::many0,
    number::complete::{le_i16, le_u16},
    sequence::tuple,
};

const THINGS_OFFSET: usize = 1;
const LINEDEFS_OFFSET: usize = 2;
const SIDEDEFS_OFFSET: usize = 3;
const VERTICES_OFFSET: usize = 4;
const SEGMENTS_OFFSET: usize = 5;
const SUBSECTORS_OFFSET: usize = 6;
const NODES_OFFSET: usize = 7;
const SECTORS_OFFSET: usize = 8;
// unused
// const REJECT_OFFSET: usize = 9;
// const BLOCKMAP_OFFSET: usize = 10;

fn is_level(name: &str) -> bool {
    let chars: Vec<_> = name.chars().collect();
    match chars.as_slice() {
        ['E', x, 'M', y] | ['M', 'A', 'P', x, y] => {
            (('1'..='2').contains(x) && ('1'..='9').contains(y)) || (*x == '3' && *y == '0')
        }
        _ => false,
    }
}

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
    fn parse(i: &[u8]) -> OnlyResult<Vec<Thing>> {
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
    fn parse(i: &[u8]) -> OnlyResult<Vec<Linedef>> {
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
    fn parse(i: &[u8]) -> OnlyResult<Vec<Sidedef>> {
        let (_, sidedefs) = many0(Sidedef::parse)(i)?;
        Ok(sidedefs)
    }
}

pub type Vertex = (i16, i16);
fn parse_vertices(i: &[u8]) -> OnlyResult<Vec<Vertex>> {
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
    fn parse(i: &[u8]) -> OnlyResult<Vec<Segment>> {
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
    fn parse(i: &[u8]) -> OnlyResult<Vec<SubSector>> {
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
    fn parse(i: &[u8]) -> OnlyResult<Vec<Node>> {
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
    fn parse<'a>(i: &'a [u8]) -> OnlyResult<Vec<Sector<'a>>> {
        let (_, sectors) = many0(Sector::parse)(i)?;
        Ok(sectors)
    }
}

pub struct Level<'a> {
    pub name: &'a str,
    pub things: Vec<Thing>,
    pub linedefs: Vec<Linedef>,
    pub sidedefs: Vec<Sidedef<'a>>,
    pub vertices: Vec<Vertex>,
    pub segments: Vec<Segment>,
    pub subsectors: Vec<SubSector>,
    pub nodes: Vec<Node>,
    pub sectors: Vec<Sector<'a>>,
}

impl<'a> Level<'a> {
    fn parse(i: usize, archive: &'a Archive) -> OnlyResult<'a, Self> {
        Ok(Self {
            name: archive.get_by_index(i).unwrap().name,
            things: Things::parse(archive.get_by_index(i + THINGS_OFFSET).unwrap().data)?,
            linedefs: Linedefs::parse(archive.get_by_index(i + LINEDEFS_OFFSET).unwrap().data)?,
            sidedefs: Sidedefs::parse(archive.get_by_index(i + SIDEDEFS_OFFSET).unwrap().data)?,
            vertices: parse_vertices(archive.get_by_index(i + VERTICES_OFFSET).unwrap().data)?,
            segments: Segments::parse(archive.get_by_index(i + SEGMENTS_OFFSET).unwrap().data)?,
            subsectors: SubSectors::parse(
                archive.get_by_index(i + SUBSECTORS_OFFSET).unwrap().data,
            )?,
            nodes: Nodes::parse(archive.get_by_index(i + NODES_OFFSET).unwrap().data)?,
            sectors: Sectors::parse(archive.get_by_index(i + SECTORS_OFFSET).unwrap().data)?,
        })
    }
}

pub struct Levels;
impl Levels {
    pub fn parse<'a>(archive: &'a Archive) -> OnlyResult<'a, Vec<Level<'a>>> {
        archive
            .iter()
            .enumerate()
            .filter_map(|(i, lump)| {
                if is_level(lump.name) {
                    Some(Level::parse(i, archive))
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_levels_print_names() {
        let file = std::fs::read(env!("TEST_WAD")).expect("Error reading wad file");
        let archive =
            crate::wad::parser::file::Archive::parse(&file).expect("Wad file parser error");
        let levels = super::Levels::parse(&archive).expect("Error parsing levels");
        levels.into_iter().for_each(|level| {
            println!("Level: {}", level.name);
            println!("    {:4} things", level.things.len());
            println!("    {:4} linedefs", level.linedefs.len());
            println!("    {:4} sidedefs", level.sidedefs.len());
            println!("    {:4} vertices", level.vertices.len());
            println!("    {:4} segs", level.segments.len());
            println!("    {:4} subsectors", level.subsectors.len());
            println!("    {:4} nodes", level.nodes.len());
            println!("    {:4} sectors", level.sectors.len());
        });
    }
}
