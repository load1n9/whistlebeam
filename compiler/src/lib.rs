use beam_file::RawBeamFile;
use beam_file::chunk::{Chunk, RawChunk};

pub fn test() {
    let chunk = RawChunk{id: *b"Atom", data: Vec::new()};
    let beam = RawBeamFile{chunks: vec![chunk]};
    beam.to_file("my.beam").unwrap();
}
