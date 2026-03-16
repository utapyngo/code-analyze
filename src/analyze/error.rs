// SPDX-License-Identifier: Apache-2.0

#[derive(Debug, thiserror::Error)]
pub enum AnalyzeError {
    #[error("{0}")]
    Parse(String),

    #[error("{0}")]
    TreeSitter(String),

    #[error("{0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Other(String),
}
