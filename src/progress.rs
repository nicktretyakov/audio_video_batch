use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub struct BatchProgress {
    pub main_bar: ProgressBar,
    pub current_video_bar: ProgressBar,
}

impl BatchProgress {
    pub fn new(total_videos: usize) -> Self {
        let main_bar = ProgressBar::new(total_videos as u64);
        main_bar.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} videos ({eta})")
                .unwrap()
                .progress_chars("#>-"),
        );

        let current_video_bar = ProgressBar::new(100);
        current_video_bar.set_style(
            ProgressStyle::default_bar()
                .template("  {spinner:.green} {msg} [{bar:30.yellow/red}] {percent}%")
                .unwrap()
                .progress_chars("=>-"),
        );

        Self {
            main_bar,
            current_video_bar,
        }
    }

    pub fn start_video(&self, video_name: &str) {
        self.current_video_bar.reset();
        self.current_video_bar
            .set_message(format!("Processing {}", video_name));
    }

    pub fn update_video_progress(&self, step: &str, progress: u64) {
        self.current_video_bar.set_message(step.to_string());
        self.current_video_bar.set_position(progress);
    }

    pub fn finish_video(&self, success: bool) {
        if success {
            self.current_video_bar.finish_with_message("✓ Complete");
        } else {
            self.current_video_bar.finish_with_message("✗ Failed");
        }
        self.main_bar.inc(1);
    }

    pub fn finish(&self) {
        self.current_video_bar.finish_and_clear();
        self.main_bar
            .finish_with_message("Batch processing complete!");
    }
}
