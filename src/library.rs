use std::collections::hash_map::Iter;
use std::collections::HashMap;

#[derive(Debug)]
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

    pub fn add_song(&mut self, song: Song) -> SongId {
        let id = self.next_id;
        self.songs.insert(id, song);
        self.next_id = self.next_id.next();

        id
    }

    pub fn get_song(&self, id: &SongId) -> Option<&Song> {
        self.songs.get(id)
    }

    pub fn song_count(&self) -> usize {
        self.songs.len()
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
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

#[derive(Debug)]
pub struct Song {
    pub title: String,
}

#[derive(Debug)]
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

#[cfg(test)]
mod test {
    use crate::library::SongId;
    use crate::{Library, Song};

    #[test]
    fn library_new_library_is_empty() {
        let library = Library::new();
        assert_eq!(library.song_count(), 0);

        assert!(library.get_song(&SongId(0)).is_none());
    }

    #[test]
    fn library_add_song_gives_unique_id() {
        let mut library = Library::new();

        let id1 = library.add_song(Song {
            title: String::new(),
        });
        let id2 = library.add_song(Song {
            title: String::new(),
        });

        assert_ne!(id1, id2);
    }

    #[test]
    fn library_get_song() {
        let mut library = Library::new();

        let song_title1 = "title!";
        let song_title2 = "another title";

        let id1 = library.add_song(Song {
            title: song_title1.to_owned(),
        });
        let id2 = library.add_song(Song {
            title: song_title2.to_owned(),
        });

        let song1 = library.get_song(&id1).unwrap();
        let song2 = library.get_song(&id2).unwrap();

        assert_eq!(song1.title, song_title1);
        assert_eq!(song2.title, song_title2);
    }
}
