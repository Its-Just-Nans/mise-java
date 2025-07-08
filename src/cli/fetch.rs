use crossbeam_channel::{select, unbounded};
use eyre::Result;
use log::{error, info};
use std::{collections::HashMap, sync::Arc};

use crate::{
    db::{jvm_repository::JvmRepository, pool::ConnectionPool},
    jvm::vendor::{VENDORS, Vendor},
};

/// Fetch data from JVM vendors
///
/// Will crawl data from all vendors if none are specified
#[derive(Debug, clap::Args)]
#[clap(verbatim_doc_comment)]
pub struct Fetch {
    /// Vendors to fetch e.g.: openjdk, zulu
    #[clap(value_name = "VENDOR")]
    pub vendors: Vec<String>,
}

impl Fetch {
    pub fn run(self) -> Result<()> {
        if self.vendors.is_empty() {
            info!("fetching all vendors");
        } else {
            info!("fetching vendors: {:?}", self.vendors);
        }

        let start = std::time::Instant::now();
        let conn_pool = ConnectionPool::get_pool()?;
        let pool = rayon::ThreadPoolBuilder::default().build()?;
        pool.scope(|s| {
            let run = |name: String, vendor: Arc<dyn Vendor>| {
                let conn_pool = conn_pool.clone();
                s.spawn(move |_| {
                    let db = match JvmRepository::new(conn_pool) {
                        Ok(db) => db,
                        Err(err) => {
                            error!("[{name}] failed to connect to database: {err}");
                            return;
                        }
                    };

                    info!("[{name}] fetching meta data");
                    let jvm_data = match vendor.fetch() {
                        Ok(data) => data,
                        Err(err) => {
                            error!("[{name}] failed to fetch meta data: {err}");
                            return;
                        }
                    };

                    info!("[{name}] writing to database");
                    match db.insert(&jvm_data) {
                        Ok(result) => {
                            info!("[{name}] inserted/modified {result} records")
                        }
                        Err(err) => {
                            error!("[{name}] failed to write to database: {err}");
                        }
                    };
                });
            };

            let (tx, rx) = unbounded();
            for (name, vendor) in self.get_vendors() {
                tx.send((name, vendor)).unwrap();
            }
            drop(tx);

            loop {
                select! {
                    recv(rx) -> msg => {
                        match msg {
                            Ok((name, vendor)) => run(name, vendor),
                            Err(_) => break,
                        }
                    }
                }
            }
        });

        info!("fetched all vendors in {:.2} seconds", start.elapsed().as_secs_f32());
        Ok(())
    }

    fn get_vendors(&self) -> HashMap<String, Arc<dyn Vendor>> {
        VENDORS
            .iter()
            .map(|v| (v.get_name(), v.to_owned()))
            .filter(|(k, _v)| self.vendors.is_empty() || self.vendors.contains(k))
            .collect()
    }
}
