use clap::{crate_version, App, Arg};

#[derive(Debug, Clone)]
pub enum OptNames {
    WorkDir,
}

impl OptNames {
    pub fn to_str(&self) -> &str {
        match self {
            Self::WorkDir => "workdir",
        }
    }
}

pub struct Opts {
    pub work_dir: String,
}

impl Opts {
    pub fn new() -> Self {
        let matches = App::new("My Blog")
            .version(crate_version!())
            .author("coimioc. <cocoaffee@gmail.com>")
            .about("My blog CLI")
            .arg(
                Arg::with_name(OptNames::WorkDir.to_str())
                    .short("w")
                    .long("workdir")
                    .value_name("WORK_DIR")
                    .help("Set working directory of blog server.")
                    .takes_value(true),
            )
            .get_matches();

        Opts {
            work_dir: matches
                .value_of(OptNames::WorkDir.to_str())
                .unwrap_or(".")
                .to_string(),
        }
    }
}
