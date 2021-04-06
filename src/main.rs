mod model;

use file::wad::{
    parser::{
        file::Archive,
        level::{Level, Levels},
    },
    utils::merge,
};
use model::Model;
use std::{
    fs::{create_dir_all as mkdir, read as fread, write as fwrite},
    path::{Path, PathBuf},
};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "wad2obj", about = "Convert doom levels into .obj models")]
struct Opt {
    #[structopt(short, long = "iwad", parse(from_os_str), help = "Path to main IWAD")]
    iwad: PathBuf,

    #[structopt(
        short,
        long = "out",
        default_value = "out",
        parse(from_os_str),
        help = "Output directory where everything will be saved"
    )]
    output_dir: PathBuf,

    #[structopt(
        short,
        long = "pwad",
        parse(from_os_str),
        help = "Path to one or many PWAD-s"
    )]
    pwads: Vec<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();

    let iwad_data = fread(&opt.iwad).expect("fread: error reading iwad");
    let mut iwad = Archive::parse(&iwad_data).expect("parser: error processing iwad");
    println!("IWAD: parsed");

    let pwads_data: Vec<_> = opt
        .pwads
        .iter()
        .map(|pwad_path| fread(pwad_path).expect("fread: error reading pwad"))
        .collect();
    if !pwads_data.is_empty() {
        let pwads: Vec<_> = pwads_data
            .iter()
            .map(|data| Archive::parse(data).expect("parser: error processing pwad"))
            .collect();
        println!("PWADs: parsed");

        merge(&mut iwad, pwads);
        println!("IWAD: merged");
    } else {
        println!("PWADs: skip...");
    }

    mkdir(&opt.output_dir).expect("mkdir: error creating output directory");

    let levels = Levels::parse(iwad.iter()).expect("parser: error processing levels");
    println!("IWAD: {} levels parsed", levels.len());

    levels
        .iter()
        .for_each(|level| save_model(&opt.output_dir, level));
}

fn save_model<P: AsRef<Path>>(output_dir: P, level: &Level) {
    let full_path = output_dir.as_ref().join(format!("{}.obj", level.name));
    let model = Model::from_level(&level);
    let obj_data = model.into_obj_str();
    fwrite(full_path, obj_data).expect("fwrite: error writing .obj model");
    println!("{} done", level.name);
}
