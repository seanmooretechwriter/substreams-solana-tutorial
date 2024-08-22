mod pb;

use crate::pb::sol::block::v1::BlockMeta;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables as DatabaseChangeTables;
use substreams_solana::pb::sf::solana::r#type::v1::Block;
use crate::pb::sf::solana::dex::trades::v1::Output;

#[substreams::handlers::map]
fn map_block(block: Block) -> Result<BlockMeta, substreams::errors::Error> {
    let mut block_height: Option<u64> = None;
    if let Some(v) = block.block_height.as_ref() {
        block_height = Some(v.block_height)
    }

    Ok(BlockMeta {
        hash: block.blockhash.to_string(),
        parent_hash: block.previous_blockhash.to_string(),
        slot: block.slot,
        parent_slot: block.parent_slot,
        transaction_count: block.transactions.len() as u64,
        block_height,
    })
}

#[substreams::handlers::map]
fn map_trades(trades: Output) -> Result<Output, substreams::errors::Error>{
    Ok(trades)
}
#[substreams::handlers::map]
fn db_out(bm: BlockMeta) -> Result<DatabaseChanges, substreams::errors::Error> {
    // Initialize changes container
    let mut tables = DatabaseChangeTables::new();

    tables
        .create_row("block", [("hash", bm.hash)])
        .set("parent_hash", bm.parent_hash)
        .set("block_height", bm.slot)
        .set("transaction_count", bm.transaction_count);
    Ok(tables.to_database_changes())
}
