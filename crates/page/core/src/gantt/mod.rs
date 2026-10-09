//! Gantt block model: groups, tasks, and their value types.

// The model has no consumer until the parser (pt-0005) and generator (pt-0007) exist.
// Remove this allowance in pt-0007.
#![allow(dead_code)]

mod date;
mod id;
mod status;
