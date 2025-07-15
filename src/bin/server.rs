use actix_web::{
    App, HttpResponse, HttpServer, Responder, Result, error,
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
    BlueprintSelect {
        payload: Vec<u8>,
        timestamp: i32,
    },
    BlueprintRangeSelect {
        idandpayloads: Vec<(i32, Vec<u8>)>,
    },
    BlockSelect {
        block: Vec<u8>,
    },
    BlockHashSelect {
        hash: Vec<u8>,
    },
    BlockIdSelect {
        id: i32,
    },
    PendingConfirmationSelect {
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
    ContextHashSelect {
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
}

#[derive(Debug)]
enum ServerError {
    InternalDatabaseError { error: dieselError },
    ConnectionError { error: ConnectionError },
    UnknownMethod { method_name: String },
    BadParameterFormat { error: jsonError },
}

impl error::ResponseError for ServerError {
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
            SqlResponse::BlueprintSelect { payload, timestamp }
        }
        "select_blueprint_range" => {
            let lowerlevel = extract_parameter(&query.params[0])?;
            let upperlevel = extract_parameter(&query.params[1])?;
            let idandpayloads = Blueprint::select_range(connection, lowerlevel, upperlevel)?;
            SqlResponse::BlueprintRangeSelect { idandpayloads }
        }
        "select_block_with_level" => {
            let id = extract_parameter(&query.params[0])?;
            let block = Block::select_with_level(connection, id)?;
            SqlResponse::BlockSelect { block }
        }
        "select_block_with_hash" => {
            let hash = extract_parameter(&query.params[0])?;
            let block = Block::select_with_hash(connection, &hash)?;
            SqlResponse::BlockSelect { block }
        }
        "select_block_hash_of_number" => {
            let id = extract_parameter(&query.params[0])?;
            let hash = Block::select_hash_of_number(connection, id)?;
            SqlResponse::BlockHashSelect { hash }
        }
        "select_block_number_of_hash" => {
            let hash = extract_parameter(&query.params[0])?;
            let id = Block::select_number_of_hash(connection, &hash)?;
            SqlResponse::BlockIdSelect { id }
        }
        "select_pending_confirmation_with_level" => {
            let level = extract_parameter(&query.params[0])?;
            let hash: Vec<u8> = PendingConfirmation::select_with_level(connection, level)?;
            SqlResponse::PendingConfirmationSelect { hash }
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
            SqlResponse::ContextHashSelect {
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
