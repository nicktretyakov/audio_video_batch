mod audio_processor;
mod batch_processor;
mod frame_analyzer;
mod ml_backend;
mod synchronizer;
mod video_processor;

use anyhow::Result;
use std::path::Path;

use audio_processor::{extract_audio, transcribe_audio};
use frame_analyzer::FrameAnalyzer;
use std::env;
use synchronizer::{print_results, synchronize_results};
use video_processor::extract_frames;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "batch" {
        run_batch_processing()
    } else if args.len() > 1 && args[1] == "single" {
        run_single_video_processing()
    } else {
        println!("Usage:");
        println!("  {} single    - Process single video (input.mp4)", args[0]);
        println!("  {} batch     - Process multiple videos in batch", args[0]);
        println!(
            "  {} batch --config - Show batch configuration options",
            args[0]
        );

        if args.len() > 2 && args[2] == "--config" {
            show_batch_config();
        }

        Ok(())
    }
}

fn run_single_video_processing() -> Result<()> {
    println!("Starting single video processing...\n");

    let video_path = Path::new("input.mp4");
    let output_dir = Path::new("frames");
    let audio_path = Path::new("output.aac");

    // Create output directory
    std::fs::create_dir_all(output_dir)?;

    // Step 1: Extract frames from video
    println!("1. Extracting frames from video...");
    let timestamps = extract_frames(video_path, output_dir)
        .map_err(|e| anyhow::anyhow!("Failed to extract frames: {}", e))?;
    println!("   Extracted {} frames", timestamps.len());

    // Step 2: Load ML analyzer
    println!("2. Loading ML analyzer...");
    let mut analyzer = FrameAnalyzer::new("mock")
        .map_err(|e| anyhow::anyhow!("Failed to create analyzer: {}", e))?;
    analyzer
        .load_model(None)
        .map_err(|e| anyhow::anyhow!("Failed to load model: {}", e))?;
    println!("   Using: {}", analyzer.backend_name());

    // Step 3: Process each frame
    println!("3. Processing frames with ML model...");
    let mut frame_results = Vec::new();
    for (i, ts) in timestamps.into_iter().enumerate() {
        let frame_path = output_dir.join(format!("frame_{:04}.png", i));
        if frame_path.exists() {
            let analysis = analyzer
                .process_frame(&frame_path, ts)
                .map_err(|e| anyhow::anyhow!("Failed to process frame {}: {}", i, e))?;
            frame_results.push(analysis.into());
        }
    }
    println!("   Processed {} frames", frame_results.len());

    // Step 4: Extract audio from video
    println!("4. Extracting audio from video...");
    extract_audio(video_path, audio_path)
        .map_err(|e| anyhow::anyhow!("Failed to extract audio: {}", e))?;

    // Step 5: Transcribe audio
    println!("5. Transcribing audio...");
    let audio_results = transcribe_audio(audio_path)?;
    println!("   Generated {} audio segments", audio_results.len());

    // Step 6: Synchronize results
    println!("6. Synchronizing video and audio results...");
    let synchronized_results = synchronize_results(frame_results, audio_results);

    // Step 7: Display results
    print_results(&synchronized_results);

    println!("Processing completed successfully!");
    Ok(())
}

fn run_batch_processing() -> Result<()> {
    use crate::batch_processor::{BatchConfig, BatchProcessor};

    println!("Starting batch video processing...\n");

    let config = BatchConfig::default();
    println!("Batch Configuration:");
    println!("  Input directory: {:?}", config.input_dir);
    println!("  Output directory: {:?}", config.output_dir);
    println!("  Supported extensions: {:?}", config.video_extensions);
    println!("  Max concurrent: {}\n", config.max_concurrent);

    let processor = BatchProcessor::new(config);

    match processor.process_batch() {
        Ok(batch_results) => {
            println!("\n=== Batch Processing Complete ===");
            println!("Total videos: {}", batch_results.total_videos);
            println!("Successful: {}", batch_results.successful);
            println!("Failed: {}", batch_results.failed);
            println!(
                "Total time: {:.2}s",
                batch_results.total_processing_time.as_secs_f64()
            );

            if batch_results.successful > 0 {
                let avg_time = batch_results.total_processing_time.as_secs_f64()
                    / batch_results.successful as f64;
                println!("Average time per successful video: {:.2}s", avg_time);
            }

            println!("\nResults saved to output directory.");
            println!("Check batch_summary.txt for detailed report.");
        }
        Err(e) => {
            eprintln!("Batch processing failed: {}", e);
            return Err(e);
        }
    }

    Ok(())
}

fn show_batch_config() {
    println!("\nBatch Processing Configuration:");
    println!("  Create 'input_videos/' directory and place your video files there");
    println!("  Supported formats: MP4, AVI, MOV, MKV, WMV, FLV");
    println!("  Results will be saved to 'output_results/' directory");
    println!("  Each video gets its own subdirectory with:");
    println!("    - frames/ (extracted frames)");
    println!("    - audio.aac (extracted audio)");
    println!("    - results.json (analysis results)");
    println!("  batch_summary.txt contains overall statistics");
}
