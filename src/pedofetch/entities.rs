use std::{error, fmt};

/// Warning message if terminal size is too small
const WARNING_IF_SMALL: &str = "Term size too small. Desired size is";
/// Default fetch size
const FETCH_SIZE: TermCoord = TermCoord { x: 110, y: 26 };

#[derive(Debug)]
pub enum PedofetchError {
    /// Error message if terminal size is too small
    SmallSizeWarning(String),
}

impl error::Error for PedofetchError {}

impl fmt::Display for PedofetchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PedofetchError::SmallSizeWarning(warning) => write!(f, "{}", warning),
        }
    }
}

struct Block {
    /// right upper corner for block
    ru: TermCoord,
    /// left bottom corner for block
    lb: TermCoord,
    /// optional command for block
    command: Option<String>,
    /// optional contents for block
    contents: Option<Vec<String>>,
    /// whether block should be wrapped
    wrap: bool,
}

/// Coordinate in terminal is represented by this struct
#[derive(Clone, Copy, Debug)]
pub struct TermCoord {
    /// x axis
    pub x: i32,
    /// y axis
    pub y: i32,
}

impl TermCoord {
    /// Calculates the corners of the terminal
    ///
    /// # Arguments
    /// * `self` - `TermCoord` struct
    ///
    /// # Returns
    /// * `Ok([TermCoord; 4])` - Array of `TermCoord` structs
    ///   in such order as [ru, lu, ld, rd]
    pub fn calculate_corners(&self) -> Result<[TermCoord; 4], PedofetchError> {
        let term_center = TermCoord {
            x: self.x / 2,
            y: self.y / 2,
        };
        if self.x <= FETCH_SIZE.x || self.y <= FETCH_SIZE.y {
            return Err(PedofetchError::SmallSizeWarning(format!(
                "{} {} {}",
                WARNING_IF_SMALL, FETCH_SIZE.x, FETCH_SIZE.y
            )));
        }
        Ok([
            TermCoord {
                x: term_center.x - FETCH_SIZE.x / 2,
                y: term_center.y - FETCH_SIZE.y / 2,
            },
            TermCoord {
                x: term_center.x + FETCH_SIZE.x / 2,
                y: term_center.y - FETCH_SIZE.y / 2,
            },
            TermCoord {
                x: term_center.x + FETCH_SIZE.x / 2,
                y: term_center.y + FETCH_SIZE.y / 2,
            },
            TermCoord {
                x: term_center.x - FETCH_SIZE.x / 2,
                y: term_center.y + FETCH_SIZE.y / 2,
            },
        ])
    }
}
