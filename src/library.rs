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

/// The ListEntryId is needed because we need a unique identifier for entries in the playlist.
/// If we don't have those, it is hard to refer to a specific playlist entry after
/// the order of the entries changed.
#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub struct ListEntryId(usize);

impl ListEntryId {
    fn next(&self) -> Self {
        ListEntryId(self.0 + 1)
    }
}

pub struct Song {
    pub title: String,
}

pub struct Playlist {
    songs: Vec<(ListEntryId, SongId)>,
    next_entry_id: ListEntryId,
}

impl Playlist {
    pub fn new() -> Self {
        Self {
            songs: Vec::new(),
            next_entry_id: ListEntryId(0),
        }
    }

    pub fn song_ids(&self) -> core::slice::Iter<'_, (ListEntryId, SongId)> {
        self.songs.iter()
    }

    pub fn add_song(&mut self, song_id: SongId) {
        self.songs.push((self.next_entry_id, song_id));
        self.next_entry_id = self.next_entry_id.next();
    }

    pub fn add_songs(&mut self, mut song_ids: Vec<SongId>) {
        for song_id in song_ids {
            self.add_song(song_id);
        }
    }

    pub fn remove_songs_by_indexes(&mut self, indexes: &Vec<usize>) {
        for &idx in indexes {
            self.songs.remove(idx);
        }
    }

    pub fn move_from_index_to_target_index(&mut self, from: usize, mut target: usize) {
        if from >= self.songs.len() {
            return;
        }
        if target >= from {
            target -= 1;
        }

        let song_id = self.songs.remove(from);
        self.songs.insert(target, song_id);
    }
}
