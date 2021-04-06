use crate::wad::parser::file::Archive;

pub fn is_level_name(name: &str) -> bool {
    let chars: Vec<_> = name.chars().collect();
    match chars.as_slice() {
        ['E', x, 'M', y] | ['M', 'A', 'P', x, y] => {
            (('1'..='2').contains(x) && ('1'..='9').contains(y)) || (*x == '3' && *y == '0')
        }
        _ => false,
    }
}

pub fn merge<'a, I>(iwad: &mut Archive<'a>, pwads: I)
where
    I: IntoIterator<Item = Archive<'a>>,
{
    pwads
        .into_iter()
        .flat_map(Archive::into_iter)
        .for_each(|lump| iwad.add_lump(lump));
}
