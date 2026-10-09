//! PAGE document model: Markdown sections and PAGE blocks in source order.

// The model has no consumer until the parser (pt-0004) and generator (pt-0007) exist.
// Remove this allowance in pt-0007.
#![allow(dead_code)]

mod block;
mod inline;
mod url;
