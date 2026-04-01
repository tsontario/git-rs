use crate::commands::CliConfig;
use crate::objects::object::Object;
use flate2::read::ZlibDecoder;
use std::io::Read;

#[derive(clap::Args, Clone)]
#[command(group = clap::ArgGroup::new("mode").required(true))]
pub struct CatFileArgs {
    #[arg(index = 1)]
    pub obj_hash: String,

    #[arg(short = 't', group = "mode")]
    pub show_type: bool,

    #[arg(short = 's', group = "mode")]
    pub show_size: bool,

    #[arg(short = 'p', group = "mode")]
    pub show_content: bool,
}

pub fn call(config: &CliConfig, args: &CatFileArgs) -> anyhow::Result<()> {
    let (prefix, hash) = args.obj_hash.split_at(2);
    let git_path = config
        .git_dir
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Git directory not found"))?;
    let obj_path = git_path.join("objects").join(prefix).join(hash);

    let mut decoder = ZlibDecoder::new(std::fs::File::open(obj_path)?);
    let mut buf = Vec::new();
    decoder.read_to_end(&mut buf)?;
    let object = Object::build(buf)?;

    if args.show_size {
        println!("{}", object.size);
    } else if args.show_type {
        println!("{}", object.obj_type);
    } else if args.show_content {
        println!("{}", String::from_utf8(object.content)?)
    } else {
        return Err(anyhow::anyhow!("No mode specified"));
    }
    Ok(())
}
