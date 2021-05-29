use file::wad::{
    parser::{
        file::Archive,
        level::{Level, Levels},
    },
    utils::merge,
};
use std::{
    fs::{create_dir_all as mkdir, read as fread, write as fwrite},
    path::{Path, PathBuf},
};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "wad2svg", about = "Convert doom levels into .svg maps")]
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
        .for_each(|level| save_svg(&opt.output_dir, level));
}

fn save_svg<P: AsRef<Path>>(output_dir: P, level: &Level) {
    let full_path = output_dir.as_ref().join(format!("{}.svg", level.name));
    let svg_data = level2svg(level);
    fwrite(full_path, svg_data).expect("fwrite: error writing .svg map");
    println!("{} done", level.name);
}

fn level2svg(level: &Level) -> String {
    const MARGIN: i16 = 50;
    const EXPECTED_WIDTH: i32 = 1024;

    let mut svg = String::new();

    let vertices = &level.vertices;
    let (mut min_x, mut min_y, mut max_x, mut max_y) =
        (vertices[0].0, vertices[0].1, vertices[0].0, vertices[0].1);
    for (x, y) in vertices {
        min_x = min_x.min(*x);
        min_y = min_y.min(*y);
        max_x = max_x.max(*x);
        max_y = max_y.max(*y);
    }
    let (sx, sy) = (max_x - min_x + 2 * MARGIN, max_y - min_y + 2 * MARGIN);
    let (w, h) = (
        EXPECTED_WIDTH as f32,
        EXPECTED_WIDTH as f32 / sx as f32 * sy as f32,
    ); // Scale to aspect ratio

    svg += &format!("<svg width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\">", w,h,sx, sy);
    svg += &format!(
        "<rect x=\"0\" y=\"0\" width=\"{}\" height=\"{}\" fill=\"white\"/>",
        sx, sy
    );

    level.linedefs.iter().for_each(|linedef| {
        let (x1, y1) = vertices[linedef.vertex_start as usize];
        let (x2, y2) = vertices[linedef.vertex_end as usize];
        svg +=
            &format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"black\" stroke-width=\"7\"/>",
            MARGIN + x1 - min_x, MARGIN + y1 - min_y, MARGIN + x2 - min_x, MARGIN + y2 - min_y
        );
    });

    svg += "</svg>";
    svg
}
