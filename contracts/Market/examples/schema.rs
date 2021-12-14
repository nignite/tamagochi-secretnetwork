use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use market::msg::{HandleMessage, InitMsg, QueryMessage};
use market::state::State;

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InitMsg), &out_dir);
    export_schema(&schema_for!(HandleMessage), &out_dir);
    export_schema(&schema_for!(QueryMessage), &out_dir);
    export_schema(&schema_for!(State), &out_dir);
}
