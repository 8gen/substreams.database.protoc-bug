mod pb;

use substreams::errors::Error;
use substreams::Hex;
use substreams_database_change::pb::database::table_change::Operation;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_ethereum::pb::eth as ethpb;

substreams_ethereum::init!();


#[substreams::handlers::map]
pub fn db_out(
    block: ethpb::v2::Block,
) -> Result<DatabaseChanges, Error> {
    let mut database_changes: DatabaseChanges = Default::default();
    database_changes
        .push_change("block", block.number.to_string().as_str(), 0, Operation::Create)
        .change("id", (None, block.number))
        .change("hash", (None, Hex(block.hash.clone())))
        .change("timestamp", (None, block.timestamp_seconds()));
    Ok(database_changes)
}
