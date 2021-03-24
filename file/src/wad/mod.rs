pub mod parser; // TODO : remove pub as it's inner part of code

use parser::file::{Archive as PArchive, Type as PType};

pub struct Archive;

impl Archive {
    pub fn new<B: AsRef<[u8]>>(wads: &[B]) {
        // TODO : unwrap remove
        let (iwads, pwads): (Vec<_>, Vec<_>) = wads
            .into_iter()
            .map(|wad| PArchive::parse(wad.as_ref()).unwrap())
            .partition(|archive| archive.wtype == PType::IWAD);

        // TODO : check that iwads.len() == 1
        // TODO : make vec of entries of iwad patched by pwads checking order of level-specific
        // entries such as THIGNS, e.t.c.
        // TODO : parse this entries to structures like Vec<Level>, PLAYPAL and so on
    }
}
