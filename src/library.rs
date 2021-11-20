use std::collections::hash_map::Iter;
use std::collections::HashMap;

pub struct Library {
    songs: HashMap<SongId, Song>,
    next_id: SongId,
}

impl Library {
    pub fn new() -> Self {
        Self {
            songs: HashMap::new(),
            next_id: SongId(0),
        }
    }

    pub fn songs(&self) -> Iter<'_, SongId, Song> {
        self.songs.iter()
    }

    pub fn add_song(&mut self, song: Song) {
        self.songs.insert(self.next_id, song);
        self.next_id = self.next_id.next();
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct SongId(usize);

impl SongId {
    fn next(&self) -> Self {
        SongId(self.0 + 1)
    }
}

pub struct Song {
    pub title: String,
}
