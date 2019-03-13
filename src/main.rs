#![feature(box_patterns)]

#[macro_use]
extern crate structopt;
extern crate pager;

use pager::Pager;
use structopt::StructOpt;

use offregisters_lib::archive::untar_all_in_dir;
use offregisters_lib::download::download;
use offregisters_nodejs::helpers;
use offregisters_nodejs::helpers::{filter_versions, highest_version};

use url::Url;
use std::path::Path;

// env!("CARGO_PKG_NAME")

#[derive(Debug, StructOpt)]
#[structopt(name = "onodejs", about = "Execute node <version> with [args ...]")]
enum NodeJsSub {
    #[structopt(name = "ls")]
    List {
        #[structopt(short = "i")]
        interactive: bool,
    },
    #[structopt(name = "ls-remote")]
    ListRemote {
        #[structopt(long = "lts")]
        lts: Option<String>,
    },
    #[structopt(name = "use")]
    Use { version: String },
}

#[derive(StructOpt, Debug)]
struct Opt {
    /// Disassemble the function at the given address
    #[structopt(short = "p", long = "prefix", default_value = "$HOME/.onvm")]
    prefix: String,
    #[structopt(subcommand)]
    subcommand: Option<NodeJsSub>,
}

fn main() {
    Pager::new().setup();
    let opt: Opt = Opt::from_args();
    println!("{:?}", opt);
    std::fs::create_dir_all(&opt.prefix).unwrap();

    match &opt {
        Opt {
            prefix: _,
            subcommand: Some(NodeJsSub::ListRemote { lts: Some(lts_val) }),
        } => {
            for version in helpers::VERSIONS.iter() {
                if &version.lts == lts_val {
                    println!("{}", version.version)
                }
            }
        }
        Opt {
            prefix: _,
            subcommand: Some(NodeJsSub::ListRemote { lts: None }),
        } => {
            for version in helpers::VERSIONS.iter() {
                println!("{}", version.version)
            }
        }
        Opt {
            prefix: prefix_s,
            subcommand:
            Some(NodeJsSub::Use {
                     version: accepted_version,
                 }),
        } => match match accepted_version.as_str() {
            "lts" => Some(highest_version(filter_versions("lts").collect())),
            _ => helpers::VERSIONS
                .iter()
                .filter(|v| v.version.ends_with(accepted_version.as_str()))
                .next(),
        } {
            Some(version) => {
                let folder_name = &*format!(
                    "node-{ver}-{os}-{arch}.tar.gz",
                    ver = version.version,
                    os = std::env::consts::OS.replace("macos", "darwin"),
                    arch = std::env::consts::ARCH.replace("x86_64", "x64"),
                );
                download(
                    Some(&prefix_s),
                    vec![Url::parse(&*format!(
                        "https://nodejs.org/dist/{ver}/{folder_name}.tar.gz",
                        ver = version.version,
                        folder_name = folder_name,
                    ))
                        .unwrap()],
                    false,
                )
                    .unwrap();
                let full_path = Path::new(prefix_s).join(folder_name);
                if !full_path.exists() {
                    untar_all_in_dir(&prefix_s, Some(&prefix_s)).unwrap();
                }
            }
            None => panic!("Unable to find Node.js version: {}", accepted_version),
        },
        _ => println!("{:?}", opt),
    }

    /*
    match opt {
        Opt{ NodeJsSub::ListRemote { lts: None } } => {
        for version in helpers::VERSIONS.iter() {
        println ! ("{}", version.version)
        }
        },
        /*
        NodeJsSub::ListRemote { lts: s_opt } => {
            let lts = s_opt.unwrap();
            for version in helpers::VERSIONS.iter().filter(|v| v.lts == lts) {
                println!("{}", version.version)
            }
        }
        NodeJsSub::Use {
            version: accepted_version,
        } => match match &*accepted_version {
            "lts" => Some(highest_version(filter_versions("lts").collect())),
            _ => helpers::VERSIONS
                .iter()
                .filter(|v| v.version.ends_with(accepted_version.as_str()))
                .next(),
        } {
            Some(version) => {
                download(
                    Some(&download_dir),
                    vec![Url::parse(&*format!(
                        "https://nodejs.org/dist/{ver}/node-{ver}-{os}-{arch}.tar.gz",
                        ver = version.version,
                        os = std::env::consts::OS.replace("macos", "darwin"),
                        arch = std::env::consts::ARCH.replace("x86_64", "x64"),
                    )).unwrap()],
                    false,
                )
                    .unwrap();
                untar_all_in_dir(&download_dir, Some(&download_dir)).unwrap();
            }
            None => panic!("Unable to find Node.js version: {}", accepted_version),
        },
        */
        _ => println!("{:?}", opt),
    }
    */
}
