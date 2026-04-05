use crate::commands::CliConfig;
use crate::objects::object::{Object, ObjectMeta};
use flate2::read::ZlibDecoder;
use std::io::Read;
use crate::objects;

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
    let store = objects::store::Store::new(config.git_dir.clone())?;
    let object = store.load_object(&args.obj_hash)?;

    if args.show_size {
        println!("{}", object.size());
    } else if args.show_type {
        println!("{}", object.obj_type());
    } else if args.show_content {
        println!("{}", String::from_utf8(object.content())?)
    } else {
        return Err(anyhow::anyhow!("No mode specified"));
    }
    Ok(())
}
