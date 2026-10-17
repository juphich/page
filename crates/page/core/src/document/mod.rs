//! PAGE document model: Markdown sections and PAGE blocks in source order.

// The model has no consumer until the parser (pt-0004) and generator (pt-0007) exist.
// Remove this allowance in pt-0007.
#![allow(dead_code)]

pub(crate) mod block;
pub(crate) mod inline;
mod page;
pub(crate) mod url;

// Unused until pt-0007 for the same reason as above.
#[allow(unused_imports)]
pub(crate) use page::{DocumentBlock, PageBlock, PageDocument};
