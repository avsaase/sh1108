//! Raw mode for coercion into richer driver types
//!
//! A display driver instance without high level functionality used as a return type from the
//! builder. Used as a source to coerce the driver into richer modes like
//! [`GraphicsMode`](../graphics/index.html).

use display_interface::AsyncWriteOnlyDataCommand;

use crate::{mode::displaymode::DisplayModeTrait, properties::DisplayProperties};

/// Raw display mode
pub struct RawMode<DI>
where
    DI: AsyncWriteOnlyDataCommand,
{
    properties: DisplayProperties<DI>,
}

impl<DI> DisplayModeTrait<DI> for RawMode<DI>
where
    DI: AsyncWriteOnlyDataCommand,
{
    /// Create new RawMode instance
    fn new(properties: DisplayProperties<DI>) -> Self {
        RawMode { properties }
    }

    /// Release all resources used by RawMode
    fn release(self) -> DisplayProperties<DI> {
        self.properties
    }
}

impl<DI: AsyncWriteOnlyDataCommand> RawMode<DI> {
    /// Create a new raw display mode
    pub fn new(properties: DisplayProperties<DI>) -> Self {
        RawMode { properties }
    }
}
