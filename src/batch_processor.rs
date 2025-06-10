use crate::audio_processor::{extract_audio, transcribe_audio, AudioResult};
use crate::frame_analyzer::{FrameAnalyzer, FrameResult};
use crate::synchronizer::{synchronize_results, SynchronizedResult};
use crate::video_processor::extract_frames;
use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

#[derive(Debug)]
pub struct BatchConfig {
    pub input_dir: PathBuf,
    pub output_dir: PathBuf,
    pub video_extensions: Vec<String>,
    pub max_concurrent: usize,
}

impl Default for BatchConfig {
    fn default() -> Self {
        Self {
            input_dir: PathBuf::from("input_videos"),
            output_dir: PathBuf::from("output_results"),
            video_extensions: vec![
                "mp4".to_string(),
                "avi".to_string(),
                "mov".to_string(),
                "mkv".to_string(),
                "wmv".to_string(),
                "flv".to_string(),
            ],
            max_concurrent: 4,
        }
    }
}

#[derive(Debug)]
pub struct VideoProcessingResult {
    pub video_path: PathBuf,
    pub processing_time: std::time::Duration,
    pub frame_count: usize,
    pub audio_segments: usize,
    pub synchronized_results: Vec<SynchronizedResult>,
    pub success: bool,
    pub error_message: Option<String>,
}

#[derive(Debug)]
pub struct BatchResults {
    pub total_videos: usize,
    pub successful: usize,
    pub failed: usize,
    pub total_processing_time: std::time::Duration,
    pub results: Vec<VideoProcessingResult>,
}

pub struct BatchProcessor {
    config: BatchConfig,
}

impl BatchProcessor {
    pub fn new(config: BatchConfig) -> Self {
        Self { config }
    }

    pub fn find_video_files(&self) -> Result<Vec<PathBuf>> {
        let mut video_files = Vec::new();

        if !self.config.input_dir.exists() {
            return Err(anyhow::anyhow!(
                "Input directory does not exist: {:?}",
                self.config.input_dir
            ));
        }

        for entry in fs::read_dir(&self.config.input_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(extension) = path.extension() {
                    let ext = extension.to_string_lossy().to_lowercase();
                    if self.config.video_extensions.contains(&ext) {
                        video_files.push(path);
                    }
                }
            }
        }

        video_files.sort();
        Ok(video_files)
    }

    pub fn process_single_video(
        &self,
        video_path: &Path,
        analyzer: &FrameAnalyzer,
    ) -> VideoProcessingResult {
        let start_time = Instant::now();
        let video_name = video_path.file_stem().unwrap().to_string_lossy();

        // Create output directories for this video
        let video_output_dir = self.config.output_dir.join(&*video_name);
        let frames_dir = video_output_dir.join("frames");
        let audio_path = video_output_dir.join("audio.aac");

        println!("Processing video: {}", video_name);

        match self.process_video_internal(video_path, &frames_dir, &audio_path, analyzer) {
            Ok((frame_results, audio_results)) => {
                let synchronized_results = synchronize_results(frame_results, audio_results);
                let processing_time = start_time.elapsed();

                // Save results to JSON file
                if let Err(e) = self.save_results(&video_output_dir, &synchronized_results) {
                    eprintln!("Warning: Failed to save results for {}: {}", video_name, e);
                }

                VideoProcessingResult {
                    video_path: video_path.to_path_buf(),
                    processing_time,
                    frame_count: synchronized_results.len(),
                    audio_segments: synchronized_results
                        .iter()
                        .filter(|r| r.audio_text.is_some())
                        .count(),
                    synchronized_results,
                    success: true,
                    error_message: None,
                }
            }
            Err(e) => {
                let processing_time = start_time.elapsed();
                eprintln!("Failed to process {}: {}", video_name, e);

                VideoProcessingResult {
                    video_path: video_path.to_path_buf(),
                    processing_time,
                    frame_count: 0,
                    audio_segments: 0,
                    synchronized_results: Vec::new(),
                    success: false,
                    error_message: Some(e.to_string()),
                }
            }
        }
    }

    fn process_video_internal(
        &self,
        video_path: &Path,
        frames_dir: &Path,
        audio_path: &Path,
        analyzer: &FrameAnalyzer,
    ) -> Result<(Vec<FrameResult>, Vec<AudioResult>)> {
        // Create directories
        fs::create_dir_all(frames_dir)?;
        fs::create_dir_all(audio_path.parent().unwrap())?;

        // Extract frames
        let timestamps = extract_frames(video_path, frames_dir)
            .map_err(|e| anyhow::anyhow!("Frame extraction failed: {}", e))?;

        // Process frames - updated to use new analyzer
        let mut frame_results = Vec::new();
        for (i, ts) in timestamps.into_iter().enumerate() {
            let frame_path = frames_dir.join(format!("frame_{:04}.png", i));
            if frame_path.exists() {
                let analysis = analyzer
                    .process_frame(&frame_path, ts)
                    .map_err(|e| anyhow::anyhow!("Frame processing failed: {}", e))?;
                frame_results.push(analysis.into());
            }
        }

        // Extract and process audio
        extract_audio(video_path, audio_path)
            .map_err(|e| anyhow::anyhow!("Audio extraction failed: {}", e))?;

        let audio_results = transcribe_audio(audio_path)?;

        Ok((frame_results, audio_results))
    }

    fn save_results(&self, output_dir: &Path, results: &[SynchronizedResult]) -> Result<()> {
        use std::io::Write;

        let results_file = output_dir.join("results.json");
        let mut file = fs::File::create(results_file)?;

        // Simple JSON serialization (in production, use serde)
        writeln!(file, "[")?;
        for (i, result) in results.iter().enumerate() {
            writeln!(file, "  {{")?;
            writeln!(file, "    \"timestamp\": {},", result.timestamp)?;
            writeln!(file, "    \"video_objects\": [")?;
            for (j, (label, conf, bbox)) in result.video_objects.iter().enumerate() {
                writeln!(file, "      {{")?;
                writeln!(file, "        \"label\": \"{}\",", label)?;
                writeln!(file, "        \"confidence\": {},", conf)?;
                writeln!(
                    file,
                    "        \"bbox\": [{}, {}, {}, {}]",
                    bbox[0], bbox[1], bbox[2], bbox[3]
                )?;
                writeln!(
                    file,
                    "      }}{}",
                    if j < result.video_objects.len() - 1 {
                        ","
                    } else {
                        ""
                    }
                )?;
            }
            writeln!(file, "    ],")?;
            if let Some(text) = &result.audio_text {
                writeln!(
                    file,
                    "    \"audio_text\": \"{}\"",
                    text.replace('"', "\\\"")
                )?;
            } else {
                writeln!(file, "    \"audio_text\": null")?;
            }
            writeln!(file, "  }}{}", if i < results.len() - 1 { "," } else { "" })?;
        }
        writeln!(file, "]")?;

        Ok(())
    }

    pub fn process_batch(&self) -> Result<BatchResults> {
        let start_time = Instant::now();

        // Create output directory
        fs::create_dir_all(&self.config.output_dir)?;

        // Find all video files
        let video_files = self.find_video_files()?;
        println!("Found {} video files to process", video_files.len());

        if video_files.is_empty() {
            return Ok(BatchResults {
                total_videos: 0,
                successful: 0,
                failed: 0,
                total_processing_time: start_time.elapsed(),
                results: Vec::new(),
            });
        }

        // Load ML model once for all videos - updated
        println!("Loading ML model...");
        let mut analyzer = FrameAnalyzer::new("mock")
            .map_err(|e| anyhow::anyhow!("Failed to create ML analyzer: {}", e))?;
        analyzer
            .load_model(None)
            .map_err(|e| anyhow::anyhow!("Failed to load ML model: {}", e))?;

        println!("Using ML backend: {}", analyzer.backend_name());

        // Process videos
        let mut results = Vec::new();
        let mut successful = 0;
        let mut failed = 0;

        for (i, video_path) in video_files.iter().enumerate() {
            println!(
                "\n[{}/{}] Processing: {:?}",
                i + 1,
                video_files.len(),
                video_path.file_name().unwrap()
            );

            let result = self.process_single_video(video_path, &analyzer);

            if result.success {
                successful += 1;
                println!(
                    "✓ Success - {} frames, {} audio segments, {:.2}s",
                    result.frame_count,
                    result.audio_segments,
                    result.processing_time.as_secs_f64()
                );
            } else {
                failed += 1;
                println!(
                    "✗ Failed - {}",
                    result
                        .error_message
                        .as_ref()
                        .unwrap_or(&"Unknown error".to_string())
                );
            }

            results.push(result);
        }

        let total_processing_time = start_time.elapsed();

        // Generate batch summary
        self.generate_batch_summary(&results, total_processing_time)?;

        Ok(BatchResults {
            total_videos: video_files.len(),
            successful,
            failed,
            total_processing_time,
            results,
        })
    }

    fn generate_batch_summary(
        &self,
        results: &[VideoProcessingResult],
        total_time: std::time::Duration,
    ) -> Result<()> {
        use std::io::Write;

        let summary_file = self.config.output_dir.join("batch_summary.txt");
        let mut file = fs::File::create(summary_file)?;

        writeln!(file, "=== Batch Processing Summary ===")?;
        writeln!(file, "Total videos processed: {}", results.len())?;
        writeln!(
            file,
            "Successful: {}",
            results.iter().filter(|r| r.success).count()
        )?;
        writeln!(
            file,
            "Failed: {}",
            results.iter().filter(|r| !r.success).count()
        )?;
        writeln!(
            file,
            "Total processing time: {:.2}s",
            total_time.as_secs_f64()
        )?;
        writeln!(
            file,
            "Average time per video: {:.2}s",
            total_time.as_secs_f64() / results.len() as f64
        )?;
        writeln!(file)?;

        writeln!(file, "=== Individual Results ===")?;
        for result in results {
            writeln!(file, "Video: {:?}", result.video_path.file_name().unwrap())?;
            writeln!(
                file,
                "  Status: {}",
                if result.success { "SUCCESS" } else { "FAILED" }
            )?;
            writeln!(
                file,
                "  Processing time: {:.2}s",
                result.processing_time.as_secs_f64()
            )?;
            if result.success {
                writeln!(file, "  Frames processed: {}", result.frame_count)?;
                writeln!(file, "  Audio segments: {}", result.audio_segments)?;
            } else if let Some(error) = &result.error_message {
                writeln!(file, "  Error: {}", error)?;
            }
            writeln!(file)?;
        }

        Ok(())
    }
}
