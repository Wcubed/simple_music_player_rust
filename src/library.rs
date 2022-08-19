use anyhow::Result;
use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use log::warn;

const SONG_EXTENSION: &str = ".ogg";

#[derive(Debug)]
pub struct Library {
    songs: HashMap<SongId, Song>,
    /// Next id to use when inserting a new entry.
    next_id: SongId,
}

impl Library {
    pub fn new() -> Self {
        Self {
            songs: HashMap::new(),
            next_id: SongId(0),
        }
    }

    pub fn clear(&mut self) {
        self.songs.clear();
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

    pub fn add_songs(&mut self, songs: Vec<Song>) {
        for song in songs {
            self.add_song(song);
        }
    }

    pub fn song_count(&self) -> usize {
        self.songs.len()
    }

    pub fn get_song(&self, id: &SongId) -> Option<&Song> {
        self.songs.get(id)
    }

    pub fn get_random_song_id(&self) -> Option<&SongId> {
        // TODO: This is not the fastest implementation,
        //  since it needs to iter over a lot of the library.
        //  But since it is only called every now and then, for now it is fine.
        let random_index = rand::random::<usize>() % self.songs.len();
        self.songs.keys().nth(random_index)
    }
}

impl Default for Library {
    fn default() -> Self {
        Self::new()
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

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Song {
    pub title: String,
    pub path: PathBuf,
}

#[derive(Debug)]
pub struct Playlist {
    songs: Vec<(ListEntryId, SongId)>,
    /// Next id to use when inserting a new entry.
    next_entry_id: ListEntryId,
}

impl Playlist {
    pub fn new() -> Self {
        Self {
            songs: Vec::new(),
            next_entry_id: ListEntryId(0),
        }
    }

    pub fn clear(&mut self) {
        self.songs.clear();
    }

    pub fn song_ids(&self) -> core::slice::Iter<'_, (ListEntryId, SongId)> {
        self.songs.iter()
    }

    pub fn length(&self) -> usize {
        self.songs.len()
    }

    pub fn get_at_index(&self, index: usize) -> Option<&(ListEntryId, SongId)> {
        self.songs.get(index)
    }

    pub fn get_song_ids(&self) -> Vec<SongId> {
        self.songs
            .iter()
            .map(|(_entry, song_id)| *song_id)
            .collect()
    }

    pub fn add_song(&mut self, song_id: SongId) {
        self.songs.push((self.next_entry_id, song_id));
        self.next_entry_id = self.next_entry_id.next();
    }

    pub fn get_song_index(&self, entry_id: ListEntryId) -> Option<usize> {
        self.songs.iter().position(|(id, _)| id == &entry_id)
    }

    pub fn song_count(&self) -> usize {
        self.songs.len()
    }

    pub fn add_songs(&mut self, song_ids: Vec<SongId>) {
        for song_id in song_ids {
            self.add_song(song_id);
        }
    }

    pub fn remove_song(&mut self, entry_id: ListEntryId) {
        if let Some(idx) = self.songs.iter().position(|(id, _)| id == &entry_id) {
            self.songs.remove(idx);
        }
    }

    pub fn remove_song_by_index(&mut self, index: usize) {
        self.songs.remove(index);
    }

    pub fn move_from_index_to_target_index(&mut self, from: usize, target: usize) {
        if from >= self.songs.len() {
            return;
        }

        let song_id = self.songs.remove(from);
        self.songs.insert(target, song_id);
    }

    /// Returns None if there is no next entry, or if the given entry is not in the playlist.
    /// Loops to the first song if the last song is given.
    /// Also gives the index of the song in the playlist.
    pub fn get_next_entry(
        &self,
        current_entry: ListEntryId,
    ) -> Option<(ListEntryId, SongId, usize)> {
        if let Some(idx) = self.get_song_index(current_entry) {
            if idx < self.songs.len() - 1 {
                let next_idx = idx + 1;
                self.songs
                    .get(next_idx)
                    .copied()
                    .map(|(entry_id, song_id)| (entry_id, song_id, next_idx))
            } else {
                self.get_first_entry()
            }
        } else {
            None
        }
    }

    /// Returns None if there is no previous entry, or if the given entry is not in the playlist.
    /// Loops to the last song if the first song is given.
    pub fn get_previous_entry(
        &self,
        current_entry: ListEntryId,
    ) -> Option<(ListEntryId, SongId, usize)> {
        if let Some(idx) = self.songs.iter().position(|(id, _)| id == &current_entry) {
            if idx != 0 {
                let prev_entry = idx - 1;
                self.songs
                    .get(prev_entry)
                    .copied()
                    .map(|(entry_id, song_id)| (entry_id, song_id, prev_entry))
            } else {
                self.get_last_entry()
            }
        } else {
            None
        }
    }

    pub fn get_first_entry(&self) -> Option<(ListEntryId, SongId, usize)> {
        self.songs
            .first()
            .copied()
            .map(|(entry_id, song_id)| (entry_id, song_id, 0))
    }

    pub fn get_last_entry(&self) -> Option<(ListEntryId, SongId, usize)> {
        self.songs
            .last()
            .copied()
            .map(|(entry_id, song_id)| (entry_id, song_id, self.songs.len() - 1))
    }
}

impl Default for Playlist {
    fn default() -> Self {
        Self::new()
    }
}

pub fn scan_directory_for_songs<P: AsRef<Path>>(dir: P) -> Result<Vec<Song>> {
    let path = dir.as_ref();

    if !path.exists() {
        anyhow::bail!(
            "Could not scan directory '{}', it does not exist.",
            path.display()
        );
    }
    if !path.is_dir() {
        anyhow::bail!(
            "Could not scan directory '{}', it is not a directory.",
            path.display()
        );
    }

    let glob_path = path.join("**").join("*".to_owned() + SONG_EXTENSION);

    let mut songs = Vec::new();

    for entry in glob::glob(glob_path.to_str().unwrap())
        .expect("Failed to read glob pattern for scanning a directory for songs.")
    {
        match entry {
            Ok(path) => {
                if let Some(song) = song_from_file_path(path) {
                    songs.push(song);
                }
            }
            Err(e) => warn!("{}", e),
        }
    }

    Ok(songs)
}

fn song_from_file_path<P: AsRef<Path>>(file_path: P) -> Option<Song> {
    let path = PathBuf::from(file_path.as_ref());
    match path.file_stem() {
        Some(title) => Some(Song {
            title: title.to_string_lossy().to_string(),
            path,
        }),
        None => {
            warn!(
                "Could not extract song title from path '{}'.",
                path.display()
            );
            None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::library::{Library, ListEntryId, Playlist, Song, SongId};
    use std::path::PathBuf;

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
            path: PathBuf::new(),
        });
        let id2 = library.add_song(Song {
            title: String::new(),
            path: PathBuf::new(),
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
            path: PathBuf::new(),
        });
        let id2 = library.add_song(Song {
            title: song_title2.to_owned(),
            path: PathBuf::new(),
        });

        let song1 = library.get_song(&id1).unwrap();
        let song2 = library.get_song(&id2).unwrap();

        assert_eq!(song1.title, song_title1);
        assert_eq!(song2.title, song_title2);
    }

    #[test]
    fn playlist_adds_songs_to_end() {
        let mut list = Playlist::new();

        assert_eq!(list.song_count(), 0);

        let id1 = SongId(1);
        let id2 = SongId(2);

        list.add_song(id1);
        list.add_song(id2);

        assert_eq!(list.song_count(), 2);

        assert_eq!(list.get_at_index(0).unwrap().1, id1);
        assert_eq!(list.get_at_index(1).unwrap().1, id2);
        assert_eq!(list.get_at_index(2), None);

        let id3 = SongId(3);
        list.add_song(id3);

        assert_eq!(list.get_at_index(2).unwrap().1, id3);
    }

    #[test]
    fn playlist_entry_ids_are_unique() {
        let mut list = Playlist::new();
        let id1 = SongId(1);
        let id2 = SongId(2);

        list.add_song(id1);
        list.add_song(id2);

        assert_ne!(
            list.get_at_index(0).unwrap().0,
            list.get_at_index(1).unwrap().0
        );
    }

    #[test]
    fn playlist_remove_song() {
        let mut list = Playlist::new();

        let id1 = SongId(1);
        let id2 = SongId(2);

        list.add_song(id1);
        list.add_song(id2);

        assert_eq!(list.song_count(), 2);

        list.remove_song(list.get_last_entry().unwrap().0);

        assert_eq!(list.song_count(), 1);

        assert_eq!(list.get_first_entry().unwrap().1, id1);
    }

    #[test]
    fn playlist_get_last_entry() {
        let mut list = Playlist::new();

        let id1 = SongId(1);
        let id2 = SongId(2);

        list.add_song(id1);
        list.add_song(id2);

        assert_eq!(list.get_last_entry().unwrap().1, id2);
    }

    #[test]
    fn playlist_move_from_index_to_target_index() {
        let mut list = Playlist::new();

        let id1 = SongId(1);
        let id2 = SongId(2);
        let id3 = SongId(3);
        let id4 = SongId(4);

        list.add_song(id1);
        list.add_song(id2);
        list.add_song(id3);
        list.add_song(id4);

        assert_eq!(list.get_song_ids(), vec![id1, id2, id3, id4]);

        list.move_from_index_to_target_index(3, 1);
        assert_eq!(list.get_song_ids(), vec![id1, id4, id2, id3]);

        list.move_from_index_to_target_index(17, 1);
        assert_eq!(list.get_song_ids(), vec![id1, id4, id2, id3]);

        list.move_from_index_to_target_index(0, 1);
        assert_eq!(list.get_song_ids(), vec![id4, id1, id2, id3]);

        list.move_from_index_to_target_index(0, 2);
        assert_eq!(list.get_song_ids(), vec![id1, id2, id4, id3]);
    }

    #[test]
    fn playlist_get_next_entry_returns_none_when_no_songs() {
        let list = Playlist::new();

        assert_eq!(list.get_next_entry(ListEntryId(0)), None);
    }

    #[test]
    fn playlist_get_next_entry_returns_next_entry() {
        let mut list = Playlist::new();

        let id1 = SongId(1);
        let id2 = SongId(2);
        let id3 = SongId(3);
        let id4 = SongId(4);

        list.add_song(id1);
        list.add_song(id2);
        list.add_song(id3);
        list.add_song(id4);

        let first = list.get_first_entry().unwrap();
        assert_eq!(first.1, id1);

        let next = list.get_next_entry(first.0).unwrap();
        assert_eq!(next.1, id2);
        let next = list.get_next_entry(next.0).unwrap();
        assert_eq!(next.1, id3);
        let next = list.get_next_entry(next.0).unwrap();
        assert_eq!(next.1, id4);

        // `get_next_entry` wraps when at the end.
        let should_be_first = list.get_next_entry(next.0).unwrap();
        assert_eq!(should_be_first.1, id1);
    }

    #[test]
    fn playlist_get_previous_entry_returns_previous_entry() {
        let mut list = Playlist::new();

        let id1 = SongId(1);
        let id2 = SongId(2);
        let id3 = SongId(3);
        let id4 = SongId(4);

        list.add_song(id1);
        list.add_song(id2);
        list.add_song(id3);
        list.add_song(id4);

        let last = list.get_last_entry().unwrap();
        assert_eq!(last.1, id4);

        let prev = list.get_previous_entry(last.0).unwrap();
        assert_eq!(prev.1, id3);
        let prev = list.get_previous_entry(prev.0).unwrap();
        assert_eq!(prev.1, id2);
        let prev = list.get_previous_entry(prev.0).unwrap();
        assert_eq!(prev.1, id1);

        // `get_previous_entry` wraps when at the start.
        assert_eq!(list.get_previous_entry(prev.0).unwrap().1, id4);
    }
}
