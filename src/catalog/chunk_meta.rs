//chuck_meta.rs

#[derive(Debug, Clone, Copy)]
pub struct SnapshotHeader {
    magic_number: [u8; 5],  //tsvdb
    version: i8,
    timestamp: u64,
    checksum: i32,    
}

pub struct Snapshot {
    header: SnapshotHeader,
    payload: Vec<u8>,
}

impl Snapshot {
    pub fn set_snapshot() {

    }

    pub fn commit_snapshot_to_disk() {
    
    }

    pub fn load_snapshot_from_disk() {


    }
}
