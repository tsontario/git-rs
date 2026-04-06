use crate::commands::CliConfig;
use crate::objects::object::Object;
use crate::objects::store;

#[derive(clap::Args, Clone)]
pub struct LsTreeArgs {
    #[arg(index = 1)]
    pub obj_hash: String,

    #[arg(short = 'r')]
    pub recursive: bool,
}

pub fn call(config: &CliConfig, args: &LsTreeArgs) -> anyhow::Result<()> {
    let store = store::Store::new(config.git_dir.to_path_buf())?;

    let object = store.load_object(&args.obj_hash)?;
    match object {
        Object::Tree(tree) => {
            if args.recursive {
                store.load_tree_recursive(&args.obj_hash, std::path::PathBuf::new())?;
            }
            for entry in tree.entries {
                println!("{}", entry);
            }
        }
        _ => return Err(anyhow::anyhow!("Expected tree object")),
    }

    Ok(())
}
