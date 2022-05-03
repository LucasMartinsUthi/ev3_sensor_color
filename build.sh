#!/bin/bash -x
cargo build --release 
scp ./target/armv5te-unknown-linux-musleabi/release/ev3dev-color-sensor ./programas