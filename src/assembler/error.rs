#[derive(Clone, serde::Serialize, serde::Deserialize)]
/// Simple struct to hold information of errors during assembly
pub struct AssemblingError {
    pub message: String,
    pub line: usize,
    pub resolution: String,
}
