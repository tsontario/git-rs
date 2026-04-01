use crate::commands::CliConfig;
use crate::objects::object::ObjectType;
use crate::objects::object_hash::ObjectHash;
use crate::objects::store;

#[derive(clap::Args, Clone)]
pub struct HashObjectArgs {
    #[arg(short = 't', long = "type", default_value = "blob")]
    pub obj_type: ObjectType,

    #[arg(short = 'w', long = "write")]
    pub write: bool,

    // File to hash (or stdin if not specified)
    pub file: Option<String>,
}
pub fn call(config: &CliConfig, args: &HashObjectArgs) -> anyhow::Result<()> {
    if args.file.is_none() {
        return Err(anyhow::anyhow!(
            "No file specified and stdin is not yet supported"
        ));
    }

    let work_dir = std::path::Path::new(&config.work_dir);
    let path = std::path::Path::join(work_dir, args.file.as_ref().unwrap());
    let mut file = std::fs::File::open(path)?;
    let size = file.metadata()?.len() as usize;
    let obj_hash: ObjectHash;
    if args.write {
        obj_hash = store::write_object(
            args.obj_type,
            &mut file,
            std::path::Path::new(work_dir)
                .join(store::DEFAULT_OBJ_PATH)
                .as_path(),
            size,
        )?;
    } else {
        obj_hash = ObjectHash::build(&mut file, &mut std::io::sink(), args.obj_type, size)?;
    }

    println!("{}", obj_hash.hash);
    Ok(())
}
