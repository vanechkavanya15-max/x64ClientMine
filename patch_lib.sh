#!/bin/bash
# Insert the module declaration before the end of the file, keeping sorting
sed -i '/pub mod wgpu_speedtree_wind_pipeline;/a pub mod wgpu_speedtree_billboard_lod;' crates/m2-client/src/lib.rs
