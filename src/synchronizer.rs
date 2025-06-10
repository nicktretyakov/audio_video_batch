use crate::audio_processor::AudioResult;
use crate::frame_analyzer::FrameResult;

#[derive(Debug)]
pub struct SynchronizedResult {
    pub timestamp: f64,
    pub video_objects: Vec<(String, f32, [f32; 4])>,
    pub audio_text: Option<String>,
}

pub fn synchronize_results(
    frame_results: Vec<FrameResult>,
    audio_results: Vec<AudioResult>,
) -> Vec<SynchronizedResult> {
    let mut synchronized = Vec::new();

    for frame_result in frame_results {
        let timestamp = frame_result.timestamp;

        // Find corresponding audio segment
        let audio_text = audio_results
            .iter()
            .find(|audio| audio.start_time <= timestamp && timestamp <= audio.end_time)
            .map(|audio| audio.text.clone());

        synchronized.push(SynchronizedResult {
            timestamp,
            video_objects: frame_result.objects,
            audio_text,
        });
    }

    synchronized
}

pub fn print_results(results: &[SynchronizedResult]) {
    println!("\n=== Synchronized Video and Audio Analysis Results ===\n");

    for result in results {
        println!("Timestamp: {:.2}s", result.timestamp);

        if !result.video_objects.is_empty() {
            println!("  Video Objects:");
            for (label, confidence, bbox) in &result.video_objects {
                println!(
                    "    - {}: {:.2}% confidence at [{:.1}, {:.1}, {:.1}, {:.1}]",
                    label,
                    confidence * 100.0,
                    bbox[0],
                    bbox[1],
                    bbox[2],
                    bbox[3]
                );
            }
        }

        if let Some(text) = &result.audio_text {
            println!("  Audio: \"{}\"", text);
        }

        println!();
    }
}
