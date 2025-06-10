use crate::ml_backend::{create_ml_backend, FrameAnalysis, MLBackend};
use anyhow::Result;
use std::path::Path;

pub struct FrameAnalyzer {
    backend: Box<dyn MLBackend>,
}

impl FrameAnalyzer {
    pub fn new(backend_type: &str) -> Result<Self> {
        let backend = create_ml_backend(backend_type)?;
        Ok(Self { backend })
    }

    pub fn load_model(&mut self, model_path: Option<&Path>) -> Result<()> {
        println!("Loading ML model using {}", self.backend.backend_name());
        self.backend.load_model(model_path)
    }

    pub fn process_frame(&self, frame_path: &Path, timestamp: f64) -> Result<FrameAnalysis> {
        self.backend.process_frame(frame_path, timestamp)
    }

    pub fn backend_name(&self) -> &str {
        self.backend.backend_name()
    }
}

// Legacy compatibility functions
pub fn load_model() -> Result<FrameAnalyzer> {
    let mut analyzer = FrameAnalyzer::new("mock")?;
    analyzer.load_model(None)?;
    Ok(analyzer)
}

pub fn process_frame(
    frame_path: &Path,
    analyzer: &FrameAnalyzer,
    timestamp: f64,
) -> Result<FrameAnalysis> {
    analyzer.process_frame(frame_path, timestamp)
}

// Legacy types for compatibility
#[derive(Debug, Clone)]
pub struct FrameResult {
    pub timestamp: f64,
    pub objects: Vec<(String, f32, [f32; 4])>,
}

impl From<FrameAnalysis> for FrameResult {
    fn from(analysis: FrameAnalysis) -> Self {
        Self {
            timestamp: analysis.timestamp,
            objects: analysis
                .detections
                .into_iter()
                .map(|d| (d.label, d.confidence, d.bbox))
                .collect(),
        }
    }
}
