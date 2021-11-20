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

    pub fn get_song(&self, id: &SongId) -> Option<&Song> {
        self.songs.get(id)
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

pub struct Playlist {
    songs: Vec<SongId>,
}

impl Playlist {
    pub fn new() -> Self {
        Self { songs: Vec::new() }
    }

    pub fn song_ids(&self) -> core::slice::Iter<'_, SongId> {
        self.songs.iter()
    }

    pub fn add_song(&mut self, id: SongId) {
        self.songs.push(id);
    }

    pub fn add_songs(&mut self, mut ids: Vec<SongId>) {
        self.songs.append(&mut ids)
    }

    pub fn remove_songs_by_indexes(&mut self, indexes: &Vec<usize>) {
        for &idx in indexes {
            self.songs.remove(idx);
        }
    }
}
