//! This module provides anything required to deal with different *factors* that
//! can influence the partitioning.

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Factors {
    speed: f64,
}
