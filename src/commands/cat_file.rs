use std::io::{Read};
use flate2::read::ZlibDecoder;
use crate::commands::CliConfig;
use crate::objects::object::Object;

#[derive(clap::Args, Clone)]
#[command(group = clap::ArgGroup::new("mode").required(true))]
pub struct CatFileArgs {
    #[arg(index = 1)]
    pub obj_hash : String,

    #[arg(short = 't', group = "mode")]
    pub show_type: bool,

    #[arg(short = 's', group = "mode")]
    pub show_size: bool,

    #[arg(short = 'p', group = "mode")]
    pub show_content: bool,
}

pub fn call(config: &CliConfig, args : &CatFileArgs) -> anyhow::Result<String> {
    let (prefix, hash) = args.obj_hash.split_at(2);
    let git_path = config.git_dir.as_ref().ok_or_else(|| anyhow::anyhow!("Git directory not found"))?;
    let obj_path = git_path.join("objects").join(prefix).join(hash);

    let mut decoder = ZlibDecoder::new(std::fs::File::open(obj_path)?);
    let mut buf = Vec::new();
    decoder.read_to_end(&mut buf)?;
    let object = Object::build(buf)?;

    if args.show_size {
        Ok(object.size.to_string())
    } else if args.show_type {
        Ok(object.obj_type.to_string())
    } else if args.show_content {
        Ok(String::from_utf8(object.content)?)
    } else {
        Err(anyhow::anyhow!("No mode specified"))
    }
}

