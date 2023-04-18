#!/usr/bin/env sh 

# accepts a argument which is passed to cargo run and the application
# run this script with './run_with_log.sh debuglayout' to debug the iced gui layout.
# or just run this script without additional parameters to run the application just with 
# info logging enabled
RUST_LOG=sorpho=info cargo run $1
