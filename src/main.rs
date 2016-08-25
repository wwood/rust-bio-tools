#[macro_use]
extern crate log;
extern crate fern;
#[macro_use]
extern crate clap;
extern crate bio;
extern crate itertools;
extern crate rustc_serialize;
extern crate csv;
extern crate rust_htslib;
#[macro_use]
extern crate quick_error;

use clap::{App,AppSettings};
use itertools::Itertools;

pub mod fastq;
pub mod bam;
pub mod bcf;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml)
                      .version(env!("CARGO_PKG_VERSION"))
                      .global_settings(&[AppSettings::SubcommandRequired,
                                         AppSettings::ColoredHelp])
                      .get_matches();

    fern::init_global_logger(
        fern::DispatchConfig {
            format: Box::new(|msg, _, _| msg.to_owned()),
            output: vec![fern::OutputConfig::stderr()],
            level: log::LogLevelFilter::Info
        },
        log::LogLevelFilter::Trace
    ).unwrap();

    if let Some(matches) = matches.subcommand_matches("fastq-split") {
        if let Err(e) = fastq::split::split(
            &matches.values_of("chunks").unwrap().collect_vec()
        ) {
            error!("{}", e);
        }
    }
    else if let Some(matches) = matches.subcommand_matches("bam-depth") {
        if let Err(e) = bam::depth::depth(
            &matches.value_of("bam-path").unwrap(),
            value_t!(matches, "max-read-length", u32).unwrap_or(1000),
            value_t!(matches, "include-flags", u16).unwrap_or(0),
            value_t!(matches, "exclude-flags", u16).unwrap_or(4 | 256 | 512 | 1024),
            value_t!(matches, "min-mapq", u8).unwrap_or(0)
        ) {
            error!("{}", e);
        }
    } else if let Some(matches) = matches.subcommand_matches("vcf-to-txt") {
        if let Err(e) = bcf::to_txt::to_txt(
            &matches.values_of("info").map(|values| values.collect_vec()).unwrap_or(vec![]),
            &matches.values_of("format").map(|values| values.collect_vec()).unwrap_or(vec![])
        ) {
            error!("{}", e);
        }
    }
}