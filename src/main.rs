#[macro_use]
extern crate structopt;
extern crate pager;

use std::path::{Path, PathBuf};

use pager::Pager;
use structopt::StructOpt;

use url::Url;

use offregisters_lib::archive::untar_all_in_dir;
use offregisters_lib::download::download;
use offregisters_nodejs::helpers;
use offregisters_nodejs::helpers::{filter_versions, highest_version, Version, VERSIONS};
use dirs::home_dir;

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
    #[structopt(short = "p", long = "prefix", default_value = "default_prefix()")]
    prefix: String,
    #[structopt(subcommand)]
    subcommand: Option<NodeJsSub>,
}

fn default_prefix() -> String {
    home_dir().unwrap().join(".onvm").into_os_string().into_string().unwrap()
}

// TODO: Use this function in the `match` statement of
//  `helpers::VERSIONS.iter()
//                    .filter(|v| v.version.ends_with(accepted_version.as_str()))
//                    .next()`
fn get_version_locally<T>(prefix: T, accepted_version: T) -> Option<Version> where T: Into<String> {
    let prefix_s = prefix.into();
    let prefix: &Path = Path::new(&prefix_s);
    let accepted_version_s: String = accepted_version.into();
    let accepted_version_static: &str = &*accepted_version_s;
    let without_v: PathBuf = prefix.join(accepted_version_static);
    let ver_with_v: &str = &*format!("v{}", accepted_version_static);
    let with_v: PathBuf = prefix.join(ver_with_v);
    if without_v.exists() {
        Some(Version::ver_only(accepted_version_static))
    } else if with_v.exists() {
        Some(Version::ver_only(ver_with_v))
    } else {
        None
    }
}

fn main() {
    Pager::new().setup();
    let opt: Opt = Opt::from_args();
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
            "latest" => Some(highest_version(VERSIONS.iter().collect())),
            _ =>
                helpers::VERSIONS
                    .iter()
                    .filter(|v| v.version.ends_with(accepted_version.as_str()))
                    .next()
        } {
            Some(version) => {
                let nodejs_basename: &str = &*format!(
                    "node-{ver}-{os}-{arch}",
                    ver = version.version,
                    os = std::env::consts::OS.replace("macos", "darwin"),
                    arch = std::env::consts::ARCH.replace("x86_64", "x64"),
                );
                download(
                    Some(&prefix_s),
                    vec![Url::parse(&*format!(
                        "https://nodejs.org/dist/{ver}/{nodejs_basename}.tar.gz",
                        ver = version.version,
                        nodejs_basename = nodejs_basename,
                    ))
                        .unwrap()],
                    false,
                )
                    .unwrap();
                let full_path: PathBuf = Path::new(prefix_s).join(nodejs_basename);
                if !full_path.exists() {
                    untar_all_in_dir(&prefix_s, Some(&prefix_s)).unwrap();
                }
            }
            None => panic!("Unable to find Node.js version: {}", accepted_version),
        },
        _ => println!("{:?}", opt),
    }
}
