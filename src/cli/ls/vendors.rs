use std::{collections::HashMap, sync::Arc};

use eyre::Result;

use crate::{
    config::Conf,
    db::{jvm_repository::JvmRepository, pool::ConnectionPool},
    jvm::vendor::VENDORS,
};

#[derive(Debug, clap::Args)]
#[clap(verbatim_doc_comment)]
pub struct Vendor {}

impl Vendor {
    pub fn run(self) -> Result<()> {
        let conf = Conf::try_get()?;
        if conf.export.path.is_none() {
            return Err(eyre::eyre!("export.path is not configured"));
        }
        let conn_pool = ConnectionPool::get_pool()?;
        let db = JvmRepository::new(conn_pool)?;

        let vendors = db.get_distinct("vendor")?;
        let vendors_map = self.get_vendors();
        for vendor in &vendors {
            // skip vendors that are not supported
            if !vendors_map.contains_key(vendor) {
                continue;
            }
            println!("{}", vendor);
        }
        Ok(())
    }

    fn get_vendors(&self) -> HashMap<String, Arc<dyn crate::jvm::vendor::Vendor>> {
        VENDORS.iter().map(|v| (v.get_name(), v.to_owned())).collect()
    }
}
