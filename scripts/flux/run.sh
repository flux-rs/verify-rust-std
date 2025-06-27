#!/bin/bash

time RUSTC_BOOTSTRAP=1 FLUXFLAGS="-Fcatch-bugs -Ftimings" cargo flux -p core > ../scripts/flux/log.txt 2>&1
python3 ../scripts/flux/errors.py           ../scripts/flux/log.txt > ../scripts/flux/errors.txt
python3 ../scripts/flux/errors_by_module.py ../scripts/flux/log.txt > ../scripts/flux/errors_by_module.txt

# RUSTC_BOOTSTRAP=1 FLUXFLAGS="-Fdump-constraint -Fdump-checker-trace -Fcheck-files=/Users/rjhala/research/verify-rust-std/library/core/src/ascii/ascii_char.rs" cargo flux -p core
# RUSTC_BOOTSTRAP=1 FLUXFLAGS="-Fcatch-bugs" cargo flux -p core 
# RUSTC_BOOTSTRAP=1 FLUXFLAGS="-Fcatch-bugs -Ftimings -Fcheck-files=/Users/rjhala/research/verify-rust-std/library/core/src/pat.rs" cargo flux -p core
# time RUSTC_BOOTSTRAP=1 FLUXFLAGS="-Fcatch-bugs -Ftimings -Fcheck-def=parse_inf_nan" cargo flux -p core
# time RUST_BACKTRACE=1 RUSTC_BOOTSTRAP=1 FLUXFLAGS="-Fcatch-bugs" cargo flux -p core > log.txt 2>&1
# time RUSTC_BOOTSTRAP=1 FLUXFLAGS="-Fcatch-bugs -Fcheck-files=decimal_seq.rs" cargo flux -p core > log.txt 2>&1
# time RUSTC_BOOTSTRAP=1 FLUXFLAGS="-Fcatch-bugs -Fcheck-def=parse_decimal_seq" cargo flux -p core > log.txt 2>&1
# time RUST_BACKTRACE=1 RUSTC_BOOTSTRAP=1 FLUXFLAGS="-Fcatch-bugs -Fcheck-files=decimal_seq.rs" cargo flux -p core
