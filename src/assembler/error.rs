#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct AssemblingError {
    pub message: String,
    pub line: usize,
    pub resolution: String,
}
