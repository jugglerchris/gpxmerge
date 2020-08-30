use gpx::Gpx;
use std::fs::File;
use std::io::BufReader;

fn open_gpx(s: &str) -> Gpx {
    let f = std::fs::File::open(s).unwrap();
    let reader = std::io::BufReader::new(f);
    gpx::read(reader).unwrap()
}

fn main() {
    let mut filenames = std::env::args().skip(1);
    let mut gpx = open_gpx(&filenames.next().unwrap());
    println!("Opened a gpx");
    assert!(gpx.tracks.len() == 1);

    for filename in filenames {
        let new_gpx = open_gpx(&filename);
        for track in new_gpx.tracks {
            gpx.tracks[0].segments.extend(track.segments);
        }
    }
    gpx::write(&gpx, std::fs::File::create("test.gpx").unwrap()).unwrap();
}
