use crate::commands::CliConfig;
use crate::objects::tree;
use flate2::read::ZlibDecoder;
use std::io::Read;

#[derive(clap::Args, Clone)]
pub struct LsTreeArgs {
    #[arg(index = 1)]
    pub obj_hash: String,

    #[arg(short = 'r')]
    pub recursive: bool,
}

pub fn call(config: &CliConfig, args: &LsTreeArgs) -> anyhow::Result<()> {
    let (prefix, hash) = args.obj_hash.split_at(2);
    let git_path = &config.git_dir;
    let obj_path = git_path.join("objects").join(prefix).join(hash);

    let mut decoder = ZlibDecoder::new(std::fs::File::open(obj_path)?);
    let mut buf = Vec::new();
    decoder.read_to_end(&mut buf)?;

    let mut entries: Vec<tree::TreeEntry> = Vec::new();
    if args.recursive {
        // entries = tree::TreeEntry::parse_recursive(&buf, PathBuf::from(""))?;
    } else {
        entries = tree::TreeEntry::parse(&buf)?;
    }

    for entry in entries {
        println!("{}", entry);
    }

    Ok(())
}
