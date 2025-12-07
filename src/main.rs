use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};

mod song_config;
use song_config::SongConfig;

#[derive(Debug, Clone)]
pub struct LyricLine {
    pub text: String,
    pub start_time: f64,
    pub end_time: f64,
}

struct KaraokeApp {
    song_title: String,
    lyrics: Vec<LyricLine>,
    start_time: Instant,
    paused: bool,
    current_position: f64,
    song_duration: f64,
    line_delay: f64,
}

impl KaraokeApp {
    fn new() -> Self {
        let config = SongConfig::load();
        
        Self {
            song_title: config.title,
            lyrics: config.lyrics,
            start_time: Instant::now(),
            paused: false,
            current_position: config.start_position,
            song_duration: config.duration,
            line_delay: 0.0,
        }
    }

    fn get_current_time(&self) -> f64 {
        if self.paused {
            self.current_position
        } else {
            let time = self.current_position + self.start_time.elapsed().as_secs_f64();
            time.min(self.song_duration)
        }
    }

    fn is_song_ended(&self) -> bool {
        self.get_current_time() >= self.song_duration
    }

    fn toggle_pause(&mut self) {
        if self.is_song_ended() {
            return;
        }
        
        self.paused = !self.paused;
        if self.paused {
            self.current_position = self.get_current_time();
        } else {
            self.start_time = Instant::now();
        }
    }

    fn get_current_line_index(&self, current_time: f64) -> Option<usize> {
        for (i, line) in self.lyrics.iter().enumerate() {
            let adjusted_start = line.start_time + self.line_delay;
            let adjusted_end = line.end_time + self.line_delay;
            if current_time >= adjusted_start && current_time < adjusted_end {
                return Some(i);
            }
        }
        
        if current_time >= self.lyrics.last().unwrap().end_time + self.line_delay {
            return Some(self.lyrics.len() - 1);
        }
        
        for i in (0..self.lyrics.len()).rev() {
            let adjusted_end = self.lyrics[i].end_time + self.line_delay;
            if current_time >= adjusted_end {
                return Some(i);
            }
        }
        
        None
    }

    fn get_line_progress(&self, current_time: f64, line_idx: usize) -> f64 {
        if line_idx >= self.lyrics.len() {
            return 0.0;
        }
        let line = &self.lyrics[line_idx];
        let adjusted_start = line.start_time + self.line_delay;
        let adjusted_end = line.end_time + self.line_delay;
        
        if current_time < adjusted_start {
            return 0.0;
        }
        if current_time >= adjusted_end {
            return 1.0;
        }
        (current_time - adjusted_start) / (adjusted_end - adjusted_start)
    }

    fn is_line_completed(&self, current_time: f64, line_idx: usize) -> bool {
        if line_idx >= self.lyrics.len() {
            return false;
        }
        let line = &self.lyrics[line_idx];
        current_time >= line.end_time + self.line_delay
    }
}

fn render_lyric_content(text: &str, progress: f64, is_active: bool, is_completed: bool) -> Vec<Span> {
    if is_active {
        let chars: Vec<char> = text.chars().collect();
        let split_pos = (chars.len() as f64 * progress) as usize;

        let sung_part: String = chars.iter().take(split_pos).collect();
        let unsung_part: String = chars.iter().skip(split_pos).collect();

        let mut spans = Vec::new();

        if !sung_part.is_empty() {
            spans.push(Span::styled(
                sung_part,
                Style::default().fg(Color::Rgb(0, 255, 0)).add_modifier(Modifier::BOLD) // Green color for sung/completed lyrics
            ));
        }

        if !unsung_part.is_empty() {
            spans.push(Span::styled(
                unsung_part,
                Style::default()
                    .fg(Color::White) // White color for unsung part of current line
                    .add_modifier(Modifier::BOLD) 
            ));
        }

        spans
    } else if is_completed {
        vec![Span::styled(
            text,
            Style::default().fg(Color::Rgb(0, 255, 0)) // Green color for completed lines
        )]
    } else {
        vec![Span::styled(
            text,
            Style::default().fg(Color::White) // White color for upcoming/unplayed lines
        )]
    }
}

fn create_progress_bar(progress: f64, width: usize) -> Line<'static> {
    let total_sub_blocks = (width * 100) as f64;
    let filled_sub_blocks = (total_sub_blocks * progress) as usize;
    
    let dot_position = ((width as f64 * progress) as usize).min(width.saturating_sub(1));
    
    let mut spans = Vec::new();
    
    for i in 0..width {
        let start_block = i * 100;
        let blocks_in_this_char = filled_sub_blocks.saturating_sub(start_block).min(100);
        
        if i == dot_position {
            spans.push(Span::styled(
                "●",
                Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
            ));
        } else {
            let color = if blocks_in_this_char > 0 {
                Color::White // White for played portion
            } else {
                Color::Rgb(80, 80, 80) // Gray for unplayed portion of progress bar
            };
            
            spans.push(Span::styled(
                "━",
                Style::default().fg(color)
            ));
        }
    }
    
    Line::from(spans)
}

fn format_time(seconds: f64) -> String {
    let mins = (seconds as i32) / 60;
    let secs = (seconds as i32) % 60;
    format!("{}:{:02}", mins, secs)
}

fn ui(f: &mut ratatui::Frame, app: &KaraokeApp) {
    let size = f.size();
    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(10),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(size);

    let current_time = app.get_current_time();
    let current_idx = app.get_current_line_index(current_time);
    
    const VISIBLE_LINES: usize = 5;
    const CENTER_LINE: usize = 2;
    
    let lyrics_height = chunks[0].height as usize;
    let mut lines = Vec::new();
    
    let top_padding = (lyrics_height.saturating_sub(VISIBLE_LINES)) / 2;
    
    for display_row in 0..lyrics_height {
        if display_row >= top_padding && display_row < top_padding + VISIBLE_LINES {
            let visible_row = display_row - top_padding;
            
            if visible_row == CENTER_LINE {
                if let Some(curr_idx) = current_idx {
                    let line = &app.lyrics[curr_idx];
                    let progress = app.get_line_progress(current_time, curr_idx);
                    let is_completed = app.is_line_completed(current_time, curr_idx);
                    
                    let lyric_spans = render_lyric_content(&line.text, progress, true, is_completed);
                    
                    let mut full_spans = vec![
                        Span::styled(">     ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
                    ];
                    full_spans.extend(lyric_spans);
                    full_spans.push(Span::styled("     <", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)));
                    
                    lines.push(Line::from(full_spans));
                } else {
                    lines.push(Line::from(""));
                }
            } else {
                if let Some(curr_idx) = current_idx {
                    let offset = visible_row as i32 - CENTER_LINE as i32;
                    let lyric_idx = curr_idx as i32 + offset;
                    
                    if lyric_idx >= 0 && (lyric_idx as usize) < app.lyrics.len() {
                        let lyric_idx = lyric_idx as usize;
                        let line = &app.lyrics[lyric_idx];
                        let is_completed = app.is_line_completed(current_time, lyric_idx);
                        
                        let lyric_spans = render_lyric_content(&line.text, 0.0, false, is_completed);
                        lines.push(Line::from(lyric_spans));
                    } else {
                        lines.push(Line::from(""));
                    }
                } else {
                    lines.push(Line::from(""));
                }
            }
        } else {
            lines.push(Line::from(""));
        }
    }

    let lyrics_widget = Paragraph::new(lines)
        .alignment(Alignment::Center)
        .style(Style::default().bg(Color::Rgb(20, 24, 40))); // Background color
    f.render_widget(lyrics_widget, chunks[0]);

    let progress_ratio = (current_time / app.song_duration).min(1.0);
    let current_time_str = format_time(current_time);
    let duration_str = format_time(app.song_duration);
    
    let progress_bar_width = 30;
    let progress_bar = create_progress_bar(progress_ratio, progress_bar_width);
    
    let mut time_spans = vec![
        Span::styled(format!("{}  ", current_time_str), Style::default().fg(Color::White))
    ];
    time_spans.extend(progress_bar.spans);
    time_spans.push(Span::styled(format!("  {}", duration_str), Style::default().fg(Color::White)));
    
    let time_widget = Paragraph::new(Line::from(time_spans))
        .alignment(Alignment::Center);
    f.render_widget(time_widget, chunks[1]);

    let song_title = Paragraph::new(app.song_title.as_str())
        .style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center);
    f.render_widget(song_title, chunks[2]);

    let controls = if app.is_song_ended() {
        "♫ Song Ended - Press R to Restart ♫"
    } else {
        "⇄  ◀  ‖  ▶  ⟲"
    };
    
    let controls_widget = Paragraph::new(controls)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center);
    f.render_widget(controls_widget, chunks[3]);
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = KaraokeApp::new();
    let tick_rate = Duration::from_millis(16);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        if app.is_song_ended() && !app.paused {
            app.paused = true;
            app.current_position = app.song_duration;
        }

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => break,
                    KeyCode::Char(' ') => app.toggle_pause(),
                    KeyCode::Char('r') | KeyCode::Char('R') => {
                        app.current_position = 0.0;
                        app.start_time = Instant::now();
                        app.paused = false;
                    },
                    KeyCode::Up => {
                        app.line_delay += 0.1;
                    },
                    KeyCode::Down => {
                        app.line_delay = (app.line_delay - 0.1).max(0.0);
                    },
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
