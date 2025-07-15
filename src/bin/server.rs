use actix_web::{
    App, HttpResponse, HttpServer, Responder, Result,
    error::ResponseError,
    http::{StatusCode, header::ContentType},
    post, web,
};
use diesel::{ConnectionError, result::Error as dieselError};
use evmnodetooling::dieselsqlite::{models::*, *};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Error as jsonError;
use std::fmt::{self, Display, Formatter};

#[derive(Deserialize)]
struct Sqlquery {
    name: String,
    params: serde_json::Value,
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum SqlResponse {
    Blueprint {
        payload: Vec<u8>,
        timestamp: i32,
    },
    BlueprintRange {
        idandpayloads: Vec<(i32, Vec<u8>)>,
    },
    Block {
        block: Vec<u8>,
    },
    BlockHash {
        hash: Vec<u8>,
    },
    BlockId {
        id: i32,
    },
    PendingConfirmation {
        hash: Vec<u8>,
    },
    PendingConfirmationCount {
        count: i64,
    },
    TransactionReceipt {
        block_hash: Vec<u8>,
        block_number: i32,
        index_: i32,
        hash: Vec<u8>,
        from_: Vec<u8>,
        to_: Option<Vec<u8>>,
        receipt_fields: Vec<u8>,
    },
    TransactionReceipts {
        receipts: Vec<(Vec<u8>, i32, Vec<u8>, Vec<u8>, Option<Vec<u8>>, Vec<u8>)>,
    },
    TransactionObject {
        block_hash: Vec<u8>,
        block_number: i32,
        index_: i32,
        hash: Vec<u8>,
        from_: Vec<u8>,
        to_: Option<Vec<u8>>,
        object_fields: Vec<u8>,
    },
    TransactionObjects {
        objects: Vec<(i32, Vec<u8>, Vec<u8>, Option<Vec<u8>>, Vec<u8>)>,
    },
    ContextHash {
        context_hash: Vec<u8>,
    },
    ContextHashGet {
        id: i32,
        context_hash: Vec<u8>,
    },
    MetadataSmartRollupAddress {
        address: String,
    },
    MetadataHistoryMode {
        history_mode: String,
    },
    ActivationLevels {
        activation_levels: Vec<i32>,
    },
    KernelUpgradeLatestUnapplied {
        injected_before: i32,
        root_hash: Vec<u8>,
        activation_timestamp: i32,
    },
    KernelUpgradeInjected {
        root_hash: Vec<u8>,
        activation_timestamp: i32,
    },
    SequencerUpgradeLatestUnapplied {
        injected_before: i32,
        sequencer: Vec<u8>,
        pool_address: Vec<u8>,
        activation_timestamp: i32,
    },
    SequencerUpgradeInjected {
        sequencer: Vec<u8>,
        pool_address: Vec<u8>,
        activation_timestamp: i32,
    },
    DelayedTransactionSelect {
        payload: Vec<u8>,
    },
    L1L2LevelRelationshipGet {
        latest_l2_level: i32,
        l1_level: i32,
    },
    L1L2FinalizedLevelGet {
        start_l2_level: i32,
        end_l2_level: i32,
    },
    L1L2FinalizedLevel {
        level: i32,
    },
    L1L2FinalizedLevelLast {
        l1_level: i32,
        start_l2_level: i32,
        end_l2_level: i32,
    },
    L1L2FinalizedLevelList {
        levels: Vec<(i32, i32, i32)>,
    },
    IrminChunk {
        level: i32,
        timestamp: i32,
    },
    BlockStorageMode {
        legacy: i32,
    },
    CurrentMigrationId {
        id: i32,
    },
}

#[derive(Debug)]
enum ServerError {
    InternalDatabaseError { error: dieselError },
    ConnectionError { error: ConnectionError },
    UnknownMethod { method_name: String },
    BadParameterFormat { error: jsonError },
}

impl ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ServerError::InternalDatabaseError { error: _ } => StatusCode::INTERNAL_SERVER_ERROR,
            ServerError::ConnectionError { error: _ } => StatusCode::NOT_FOUND,
            ServerError::UnknownMethod { method_name: _ } => StatusCode::BAD_REQUEST,
            ServerError::BadParameterFormat { error: _ } => StatusCode::BAD_REQUEST,
        }
    }
}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let string = match self {
            ServerError::InternalDatabaseError { error } => {
                format!("Internal database error:{}", error)
            }
            ServerError::ConnectionError { error } => {
                format!("Error connecting to the database:{}", error)
            }
            ServerError::UnknownMethod { method_name } => format!("Unknow method:{}", method_name),
            ServerError::BadParameterFormat { error } => format!("Invalid parameters:{}", error),
        };
        write!(f, "{}", string)
    }
}

impl From<jsonError> for ServerError {
    fn from(error: jsonError) -> ServerError {
        ServerError::BadParameterFormat { error }
    }
}

impl From<dieselError> for ServerError {
    fn from(error: dieselError) -> ServerError {
        ServerError::InternalDatabaseError { error }
    }
}

impl From<ConnectionError> for ServerError {
    fn from(error: ConnectionError) -> ServerError {
        ServerError::ConnectionError { error }
    }
}

fn extract_parameter<T>(param: &serde_json::Value) -> Result<T, jsonError>
where
    T: DeserializeOwned,
{
    serde_json::from_value(param.clone())
}

#[post("/")]
async fn answer_query(query: web::Json<Sqlquery>) -> Result<impl Responder, ServerError> {
    let connection = &mut establish_connection()?;
    let method_requested = query.name.as_str();
    let response = match method_requested {
        "select_blueprint" => {
            let id = extract_parameter(&query.params[0])?;
            let (payload, timestamp) = Blueprint::select(connection, id)?;
            SqlResponse::Blueprint { payload, timestamp }
        }
        "select_blueprint_range" => {
            let lowerlevel = extract_parameter(&query.params[0])?;
            let upperlevel = extract_parameter(&query.params[1])?;
            let idandpayloads = Blueprint::select_range(connection, lowerlevel, upperlevel)?;
            SqlResponse::BlueprintRange { idandpayloads }
        }
        "select_block_with_level" => {
            let id = extract_parameter(&query.params[0])?;
            let block = Block::select_with_level(connection, id)?;
            SqlResponse::Block { block }
        }
        "select_block_with_hash" => {
            let hash = extract_parameter(&query.params[0])?;
            let block = Block::select_with_hash(connection, &hash)?;
            SqlResponse::Block { block }
        }
        "select_block_hash_of_number" => {
            let id = extract_parameter(&query.params[0])?;
            let hash = Block::select_hash_of_number(connection, id)?;
            SqlResponse::BlockHash { hash }
        }
        "select_block_number_of_hash" => {
            let hash = extract_parameter(&query.params[0])?;
            let id = Block::select_number_of_hash(connection, &hash)?;
            SqlResponse::BlockId { id }
        }
        "select_pending_confirmation_with_level" => {
            let level = extract_parameter(&query.params[0])?;
            let hash: Vec<u8> = PendingConfirmation::select_with_level(connection, level)?;
            SqlResponse::PendingConfirmation { hash }
        }
        "count_pending_confirmations" => {
            let count = PendingConfirmation::count(connection)?;
            SqlResponse::PendingConfirmationCount { count }
        }
        "select_transaction_receipt" => {
            let hash = extract_parameter(&query.params[0])?;
            let (block_hash, block_number, index_, hash, from_, to_, receipt_fields) =
                Transaction::select_receipt(connection, &hash)?;
            SqlResponse::TransactionReceipt {
                block_hash,
                block_number,
                index_,
                hash,
                from_,
                to_,
                receipt_fields,
            }
        }
        "select_transaction_receipts_from_block_number" => {
            let block_number = extract_parameter(&query.params[0])?;
            let receipts =
                Transaction::select_receipts_from_block_number(connection, block_number)?;
            SqlResponse::TransactionReceipts { receipts }
        }
        "select_transaction_object" => {
            let hash = extract_parameter(&query.params[0])?;
            let (block_hash, block_number, index_, hash, from_, to_, object_fields) =
                Transaction::select_object(connection, &hash)?;
            SqlResponse::TransactionObject {
                block_hash,
                block_number,
                index_,
                hash,
                from_,
                to_,
                object_fields,
            }
        }
        "select_transaction_objects_from_block_number" => {
            let block_number = extract_parameter(&query.params[0])?;
            let objects = Transaction::select_objects_from_block_number(connection, block_number)?;
            SqlResponse::TransactionObjects { objects }
        }
        "select_context_hash" => {
            let id = extract_parameter(&query.params[0])?;
            let context_hash = ContextHash::select(connection, id)?;
            SqlResponse::ContextHash {
                context_hash: context_hash,
            }
        }
        "get_latest_context_hash" => {
            let (id, context_hash) = ContextHash::get_latest(connection)?;
            SqlResponse::ContextHashGet { id, context_hash }
        }
        "get_earliest_context_hash" => {
            let (id, context_hash) = ContextHash::get_earliest(connection)?;
            SqlResponse::ContextHashGet { id, context_hash }
        }
        "get_smart_rollup_address" => {
            let address = Metadata::get_smart_rollup_address(connection)?;
            SqlResponse::MetadataSmartRollupAddress { address }
        }
        "get_history_mode" => {
            let history_mode = Metadata::get_history_mode(connection)?;
            SqlResponse::MetadataHistoryMode { history_mode }
        }
        "kernel_upgrade_activation_levels" => {
            let activation_levels = KernelUpgrade::activation_levels(connection)?;
            SqlResponse::ActivationLevels { activation_levels }
        }
        "get_latest_unapplied_kernel_upgrade" => {
            let (injected_before, root_hash, activation_timestamp) =
                KernelUpgrade::get_latest_unapplied(connection)?;
            SqlResponse::KernelUpgradeLatestUnapplied {
                injected_before,
                root_hash,
                activation_timestamp,
            }
        }
        "find_kernel_upgrade_injected_before" => {
            let injected_before = extract_parameter(&query.params[0])?;
            let (root_hash, activation_timestamp) =
                KernelUpgrade::find_injected_before(connection, injected_before)?;
            SqlResponse::KernelUpgradeInjected {
                root_hash,
                activation_timestamp,
            }
        }
        "find_latest_kernel_upgrade_injected_after" => {
            let injected_after = extract_parameter(&query.params[0])?;
            let (root_hash, activation_timestamp) =
                KernelUpgrade::find_latest_injected_after(connection, injected_after)?;
            SqlResponse::KernelUpgradeInjected {
                root_hash,
                activation_timestamp,
            }
        }
        "sequencer_upgrade_activation_levels" => {
            let activation_levels = SequencerUpgrade::activation_levels(connection)?;
            SqlResponse::ActivationLevels { activation_levels }
        }
        "get_latest_unapplied_sequencer_upgrade" => {
            let (injected_before, sequencer, pool_address, activation_timestamp) =
                SequencerUpgrade::get_latest_unapplied(connection)?;
            SqlResponse::SequencerUpgradeLatestUnapplied {
                injected_before,
                sequencer,
                pool_address,
                activation_timestamp,
            }
        }
        "find_sequencer_upgrade_injected_before" => {
            let injected_before = extract_parameter(&query.params[0])?;
            let (sequencer, pool_address, activation_timestamp) =
                SequencerUpgrade::find_injected_before(connection, injected_before)?;
            SqlResponse::SequencerUpgradeInjected {
                sequencer,
                pool_address,
                activation_timestamp,
            }
        }
        "find_latest_sequencer_upgrade_injected_after" => {
            let injected_after = extract_parameter(&query.params[0])?;
            let (sequencer, pool_address, activation_timestamp) =
                SequencerUpgrade::find_latest_injected_after(connection, injected_after)?;
            SqlResponse::SequencerUpgradeInjected {
                sequencer,
                pool_address,
                activation_timestamp,
            }
        }
        "select_delayed_transaction_at_level" => {
            let injected_before = extract_parameter(&query.params[0])?;
            let payload = DelayedTransaction::select_at_level(connection, injected_before)?;
            SqlResponse::DelayedTransactionSelect { payload }
        }
        "select_delayed_transaction_at_hash" => {
            let hash = extract_parameter(&query.params[0])?;
            let payload = DelayedTransaction::select_at_hash(connection, &hash)?;
            SqlResponse::DelayedTransactionSelect { payload }
        }
        "get_l1_l2_level_relationship" => {
            let (latest_l2_level, l1_level) = L1L2LevelRelationship::get(connection)?;
            SqlResponse::L1L2LevelRelationshipGet {
                latest_l2_level,
                l1_level,
            }
        }
        "get_l1_l2_finalized_level" => {
            let l1_level = extract_parameter(&query.params[0])?;
            let (start_l2_level, end_l2_level) = L1L2FinalizedLevel::get(connection, l1_level)?;
            SqlResponse::L1L2FinalizedLevelGet {
                start_l2_level,
                end_l2_level,
            }
        }
        "last_finalized_l2_level" => {
            let last_l2_level = L1L2FinalizedLevel::last_l2_level(connection)?;
            SqlResponse::L1L2FinalizedLevel {
                level: last_l2_level,
            }
        }
        "last_l1_l2_finalized_level" => {
            let (l1_level, start_l2_level, end_l2_level) = L1L2FinalizedLevel::last(connection)?;
            SqlResponse::L1L2FinalizedLevelLast {
                l1_level,
                start_l2_level,
                end_l2_level,
            }
        }
        "find_finalized_l1_level" => {
            let l2_level = extract_parameter(&query.params[0])?;
            let l1_level = L1L2FinalizedLevel::find_l1_level(connection, l2_level)?;
            SqlResponse::L1L2FinalizedLevel { level: l1_level }
        }
        "list_l1_l1_finalized_levels_by_l2_levels" => {
            let start_l2 = extract_parameter(&query.params[0])?;
            let end_l2 = extract_parameter(&query.params[1])?;
            let levels = L1L2FinalizedLevel::list_by_l2_levels(connection, start_l2, end_l2)?;
            SqlResponse::L1L2FinalizedLevelList { levels }
        }
        "list_l1_l1_finalized_levels_by_l1_levels" => {
            let start_l1 = extract_parameter(&query.params[0])?;
            let end_l1 = extract_parameter(&query.params[1])?;
            let levels = L1L2FinalizedLevel::list_by_l1_levels(connection, start_l1, end_l1)?;
            SqlResponse::L1L2FinalizedLevelList { levels }
        }
        "nth_irmin_chunk" => {
            let offset = extract_parameter(&query.params[0])?;
            let (level, timestamp) = IrminChunk::nth(connection, offset)?;
            SqlResponse::IrminChunk { level, timestamp }
        }
        "latest_irmin_chunk" => {
            let (level, timestamp) = IrminChunk::latest(connection)?;
            SqlResponse::IrminChunk { level, timestamp }
        }
        "block_storage_mode" => {
            let legacy = BlockStorageMode::legacy(connection)?;
            SqlResponse::BlockStorageMode { legacy }
        }
        "current_migration" => {
            let id = Migration::current_migration(connection)?;
            SqlResponse::CurrentMigrationId { id }
        }
        _ => {
            return Err(ServerError::UnknownMethod {
                method_name: method_requested.to_string(),
            });
        }
    };

    Ok(HttpResponse::Ok().json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(answer_query))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
