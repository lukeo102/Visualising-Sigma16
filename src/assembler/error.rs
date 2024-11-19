#[derive(Clone)]
pub struct AssemblingError {
    pub message: String,
    pub line: usize,
    pub resolution: String,
}
