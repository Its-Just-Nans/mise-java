use eyre::Result;

use crate::{
    config::Conf,
    db::{jvm_repository::JvmRepository, pool::ConnectionPool},
};

#[derive(Debug, clap::Args)]
#[clap(verbatim_doc_comment)]
pub struct Arch {}

impl Arch {
    pub fn run(self) -> Result<()> {
        let conf = Conf::try_get()?;
        if conf.export.path.is_none() {
            return Err(eyre::eyre!("export.path is not configured"));
        }
        let conn_pool = ConnectionPool::get_pool()?;
        let db = JvmRepository::new(conn_pool)?;

        let archs = db.get_distinct("architecture")?;
        for arch in &archs {
            println!("{}", arch);
        }
        Ok(())
    }
}
