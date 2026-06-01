#!/bin/bash

cargo doc --workspace --no-deps --features builds,general_renderer,repl,comp,parser,clap --open
