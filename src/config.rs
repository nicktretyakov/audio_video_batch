use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingConfig {
    pub batch: BatchConfig,
    pub ml_models: MLConfig,
    pub output: OutputConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchConfig {
    pub input_directory: PathBuf,
    pub output_directory: PathBuf,
    pub video_extensions: Vec<String>,
    pub max_concurrent_videos: usize,
    pub skip_existing: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MLConfig {
    pub video_model_path: Option<PathBuf>,
    pub audio_model_path: Option<PathBuf>,
    pub confidence_threshold: f32,
    pub use_gpu: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputConfig {
    pub save_frames: bool,
    pub save_audio: bool,
    pub output_format: String, // "json", "csv", "txt"
    pub include_timestamps: bool,
}

impl Default for ProcessingConfig {
    fn default() -> Self {
        Self {
            batch: BatchConfig {
                input_directory: PathBuf::from("input_videos"),
                output_directory: PathBuf::from("output_results"),
                video_extensions: vec![
                    "mp4".to_string(),
                    "avi".to_string(),
                    "mov".to_string(),
                    "mkv".to_string(),
                    "wmv".to_string(),
                    "flv".to_string(),
                    "webm".to_string(),
                ],
                max_concurrent_videos: 4,
                skip_existing: true,
            },
            ml_models: MLConfig {
                video_model_path: None,
                audio_model_path: None,
                confidence_threshold: 0.5,
                use_gpu: true,
            },
            output: OutputConfig {
                save_frames: false,
                save_audio: false,
                output_format: "json".to_string(),
                include_timestamps: true,
            },
        }
    }
}

impl ProcessingConfig {
    pub fn load_from_file(path: &std::path::Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: ProcessingConfig = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save_to_file(&self, path: &std::path::Path) -> anyhow::Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}
