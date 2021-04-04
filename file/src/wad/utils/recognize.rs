use crate::wad::parser::file::Lump;

const LEVEL_LUMPS: [&str; 10] = [
    "THINGS", "LINEDEFS", "SIDEDEFS", "VERTEXES", "SEGS", "SSECTORS", "NODES", "SECTORS", "REJECT",
    "BLOCKMAP",
];

pub fn is_level_name(name: &str) -> bool {
    let chars: Vec<_> = name.chars().collect();
    match chars.as_slice() {
        ['E', x, 'M', y] | ['M', 'A', 'P', x, y] => {
            (('1'..='2').contains(x) && ('1'..='9').contains(y)) || (*x == '3' && *y == '0')
        }
        _ => false,
    }
}

pub fn recognize_levels<'a, I>(lumps: I) -> Vec<Vec<&'a Lump<'a>>>
where
    I: IntoIterator<Item = &'a Lump<'a>>,
{
    let level_lumps: Vec<_> = lumps
        .into_iter()
        .filter(|lump| is_level_name(lump.name) || LEVEL_LUMPS.contains(&lump.name))
        .collect();
    level_lumps
        .chunks(LEVEL_LUMPS.len() + 1)
        .map(|lumps| lumps.to_owned())
        .collect()
}

pub fn recognize_block<'a, I>(
    lumps: I,
    start: &'a str,
    end: &'a str,
) -> impl Iterator<Item = &'a Lump<'a>>
where
    I: IntoIterator<Item = &'a Lump<'a>>,
{
    lumps
        .into_iter()
        .skip_while(move |lump| lump.name != start)
        .take_while(move |lump| lump.name != end)
        .filter(|lump| !lump.is_virtual())
}
