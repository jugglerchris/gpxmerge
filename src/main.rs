use gpx::Gpx;
use structopt::StructOpt;

#[derive(Debug,StructOpt)]
#[structopt(name = "gpxmerge", about = "GPX merging tool")]
pub struct Opts {
    #[structopt(short = "R", long = "rename")]
    rename: Option<String>,

    #[structopt(short = "o", long = "output")]
    output: String,

    files: Vec<String>,
}

fn open_gpx(s: &str) -> Gpx {
    let f = std::fs::File::open(s).unwrap();
    let reader = std::io::BufReader::new(f);
    gpx::read(reader).unwrap()
}

fn main() {
    let opt = Opts::from_args();
    let mut gpx = open_gpx(&opt.files[0]);
    println!("Opened a gpx");
    assert!(gpx.tracks.len() == 1);

    if let Some(new_name) = opt.rename.as_ref() {
        match gpx.metadata {
            None => {
                gpx.metadata = Some(gpx::Metadata {
                    name: Some(new_name.into()),
                    ..Default::default()
                });
            }
            Some(ref mut metadata) => {
                metadata.name = Some(new_name.into());
            }
        }
    }

    for filename in &opt.files[1..] {
        let new_gpx = open_gpx(&filename);
        for track in new_gpx.tracks {
            gpx.tracks[0].segments.extend(track.segments);
        }
    }
    gpx::write(&gpx, std::fs::File::create(&opt.output).unwrap()).unwrap();
}
