use cargo::core::features;
use cargo::{self, drop_print, drop_println, CliResult, Config};
use clap::{AppSettings, Arg, ArgMatches};

#[test]
fn a_simple_test() {
    assert_eq!(2+2, 4);
}