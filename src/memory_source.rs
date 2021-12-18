use rodio::{Decoder, Source};
use std::io::{Read, Seek};
use std::time::Duration;

pub struct MemorySource {
    data: Vec<i16>,
    channels: u16,
    sample_rate: u32,
    current_playback_location: usize,
}

impl MemorySource {
    pub fn from_decoder<R>(decoder: Decoder<R>) -> Self
    where
        R: Read + Seek,
    {
        let channels = decoder.channels();
        let sample_rate = decoder.sample_rate();
        let data = decoder.collect();

        MemorySource {
            data,
            channels,
            sample_rate,
            current_playback_location: 0,
        }
    }
}

impl Iterator for MemorySource {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_playback_location += 1;
        self.data.get(self.current_playback_location - 1).copied()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.data.len(), Some(self.data.len()))
    }
}

impl Source for MemorySource {
    /// Frame length is "Until source end" for an in-memory source, indicated in Rodio by `None`.
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        self.channels
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    /// Always returns Some.
    fn total_duration(&self) -> Option<Duration> {
        let seconds = self.data.len() as f32 / (self.channels as u32 * self.sample_rate) as f32;
        Some(Duration::from_secs_f32(seconds))
    }
}

#[cfg(test)]
mod test {
    use crate::memory_source::MemorySource;
    use rodio::{Decoder, Source};
    use std::fs::File;
    use std::io::BufReader;
    use std::time::Duration;

    #[test]
    fn test_from_decoder() {
        let audio_file = BufReader::new(File::open("test_assets/test_library/noise.ogg").unwrap());
        let decoder = Decoder::new(audio_file).unwrap();
        let memory_source = MemorySource::from_decoder(decoder);

        assert_eq!(
            memory_source.total_duration(),
            Some(Duration::from_secs_f32(2.382_948))
        );
    }

    #[test]
    fn test_size_hint() {
        let source = MemorySource {
            data: vec![10, 12, 0, 13, 56, 11],
            channels: 0,
            sample_rate: 0,
            current_playback_location: 0,
        };

        assert_eq!(source.size_hint(), (6, Some(6)));
    }

    #[test]
    fn test_next() {
        let mut source = MemorySource {
            data: vec![10, 12, 0, 13],
            channels: 0,
            sample_rate: 0,
            current_playback_location: 0,
        };

        assert_eq!(source.next(), Some(10));
        assert_eq!(source.next(), Some(12));
        assert_eq!(source.next(), Some(0));
        assert_eq!(source.next(), Some(13));
        assert_eq!(source.next(), None);
        assert_eq!(source.next(), None);
    }
}
