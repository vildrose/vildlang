//! Span used to signify positions and ranges in the source code.

/// Represents a unique identifier for a file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileId(pub u32);

/// Represents a position in the source code, including byte offset, line number, and column number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    /// Zero-based UTF-8 byte offset into the source file.
    pub byte: usize,
    /// One-based line number, for user-facing diagnostics.
    pub line: u32,
    /// One-based column number, for user-facing diagnostics.
    pub column: u32,
}

impl Position {
    /// Returns a position at the start of the file (byte 0, line 1, column 1).
    #[must_use]
    pub const fn start() -> Self {
        Self {
            byte: 0,
            line: 1,
            column: 1,
        }
    }
}

/// Represents a span of text in the source code, including a file identifier and start/end positions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    /// The file identifier for this span.
    pub file: FileId,
    /// The start position of this span.
    pub start: Position,
    /// The end position of this span.
    pub end: Position,
}

impl Span {
    /// Returns a new span with the given file identifier and start/end positions.
    #[must_use]
    pub const fn new(file: FileId, start: Position, end: Position) -> Self {
        Self { file, start, end }
    }

    /// Returns an empty span at the given position in the given file.
    #[must_use]
    pub const fn empty(file: FileId, at: Position) -> Self {
        Self {
            file,
            start: at,
            end: at,
        }
    }
}
