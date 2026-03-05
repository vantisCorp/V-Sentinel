//! Micro-segmentation module for Zero Trust
//!
//! This module provides micro-segmentation capabilities:
//! - Network segmentation (Phase 1)
//! - Application-level segmentation (Phase 3)
//! - Data segmentation controls (Phase 3)

pub mod network;
pub mod application;
pub mod data;

pub use network::{NetworkSegmenter, NetworkSegment, SegmentPolicy, Protocol, SegmentType};
pub use application::{ApplicationSegmenter, ApplicationSegment, ServiceMeshConfig, ServiceDependency};
pub use data::{DataSegmenter, DataSegment, DataClassification, UserAttributes, DataAccessResult};