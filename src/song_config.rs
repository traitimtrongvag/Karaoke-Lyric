use crate::LyricLine;

pub struct SongConfig {
    pub title: String,
    pub duration: f64,
    pub start_position: f64,
    pub lyrics: Vec<LyricLine>,
}

impl SongConfig {
    pub fn load() -> Self {
        // Song metadata - modify these values for different songs
        let title = "Title here".to_string();
        let duration = 21.0;  // Total song duration in seconds (0:21)
        let start_position = 0.0;  // Starting position in seconds (0:0)

        // Lyrics with timing - modify or add lines as needed
        // Format: text, start_time (seconds), end_time (seconds)
        let lyrics = vec![
            LyricLine {
                text: "Example line 1".to_string(),
                start_time: 0.0, // Start time
                end_time: 3.0, // End time 
            },
            LyricLine {
                text: "Example line 2".to_string(),
                start_time: 3.0,
                end_time: 6.0,
            },
            LyricLine {
                text: "Example line 3".to_string(),
                start_time: 6.0,
                end_time: 9.0,
            },
            LyricLine {
                text: "Example line 4".to_string(),
                start_time: 9.0,
                end_time: 12.0,
            },
            LyricLine {
                text: "Example line 5".to_string(),
                start_time: 12.0,
                end_time: 15.0,
            },
            LyricLine {
                text: "Example line 6".to_string(),
                start_time: 15.0,
                end_time: 18.0,
            },
            LyricLine {
                text: "Example line 7".to_string(),
                start_time: 18.0,
                end_time: 21.0,
            },

        ];

        Self {
            title,
            duration,
            start_position,
            lyrics,
        }
    }
}

