use log::{debug, info};
use osmpbfreader::{OsmObj, OsmPbfReader};
use rusqlite::{Connection, NO_PARAMS};
use std::fs::File;
use std::path::Path;
use structopt::StructOpt;

const MIGRATIONS: [&'static str; 2] = [
    //========= 1 create nodes table =========
    "
        CREATE TABLE IF NOT EXISTS nodes (
            id INTEGER PRIMARY KEY,
            lat INTEGER NOT NULL,
            lon INTEGER NOT NULL
        )
    ",
    //========= 2 create ways_to_nodes table =========
    "
        CREATE TABLE IF NOT EXISTS ways_to_nodes (
            way INTEGER,
            node INTEGER,
        )
    ",
];

pub fn init(path: impl AsRef<Path>) -> Result<Connection, rusqlite::Error> {
    info!("opening connection to sqlite db at {:?}", path.as_ref());
    let mut conn = rusqlite::Connection::open(path)?;

    let ver = conn.pragma_query_value::<i64, _>(None, "user_version", |row| row.get(0))? as usize;
    info!("database is at version {} (0 is freshly-created)", ver);

    if (ver as usize) < MIGRATIONS.len() {
        info!(
            "need to run migrations! currently at {}, migrations list is at {}",
            ver,
            MIGRATIONS.len()
        );

        for (i, m) in MIGRATIONS[ver..].iter().enumerate() {
            let tx = conn.transaction()?;
            debug!("executing migration {}", i + 1);
            tx.execute(m, NO_PARAMS)?;
            tx.commit()?;

            debug!(
                "migration committed! rewriting database version to {}",
                ver + i + 1
            );
            conn.pragma_update(None, "user_version", &((ver + i) as i64 + 1))?;
        }
    }

    Ok(conn)
}

#[derive(Debug, StructOpt)]
struct Opts {
    db: String,
    osm: String,
    interested: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opts::from_args();
    env_logger::init();
    let mut conn = init(opt.db)?;

    let mut pbf = {
        let r = File::open(opt.osm)?;
        OsmPbfReader::new(r)
    };

    let file_contents = std::fs::read(opt.interested)?;
    let interested: Vec<OsmObj> = ron::de::from_bytes(&file_contents)?;

    for relation in interested.iter().filter(|o|o.is_relation()) {
        // relation.

    }

    // let tx = conn.transaction()?;
    // for obj in pbf.iter().map(Result::unwrap) {
    //     match obj {
    //         OsmObj::Node(node) => {
    //             tx.execute(
    //                 "INSERT INTO nodes (id, lat, lon) VALUES (?, ?, ?)",
    //                 &[
    //                     node.id.0,
    //                     node.decimicro_lat as i64,
    //                     node.decimicro_lon as i64,
    //                 ],
    //             )?;
    //         }
    //         _ => {}
    //     }
    // }
    // tx.commit()?;

    Ok(())
}
