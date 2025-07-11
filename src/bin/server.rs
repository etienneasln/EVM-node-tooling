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
    NumbersofRowsAffected {
        number: usize,
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
            let id: i32 = extract_parameter(&query.params[0])?;
            let (payload, timestamp) = Blueprint::select(connection, id)?;
            SqlResponse::BlueprintSelect { payload, timestamp }
        }
        "insert_blueprint" => {
            let id: i32 = extract_parameter(&query.params[0])?;
            let payload: Vec<u8> = extract_parameter(&query.params[1])?;
            let timestamp: i32 = extract_parameter(&query.params[2])?;
            let blueprint = Blueprint {
                id,
                payload,
                timestamp,
            };
            let insert_result = blueprint.insert(connection)?;
            SqlResponse::NumbersofRowsAffected {
                number: insert_result,
            }
        }
        "select_blueprint_range" => {
            let lowerlevel = extract_parameter(&query.params[0])?;
            let upperlevel = extract_parameter(&query.params[1])?;
            let idandpayloads = Blueprint::select_range(connection, lowerlevel, upperlevel)?;
            SqlResponse::BlueprintRangeSelect { idandpayloads }
        }
        "clear_after_blueprint" => {
            let level: i32 = extract_parameter(&query.params[0])?;
            let clear_after_result = Blueprint::clear_after(connection, level)?;
            SqlResponse::NumbersofRowsAffected {
                number: clear_after_result,
            }
        }
        "clear_before_blueprint" => {
            let level: i32 = extract_parameter(&query.params[0])?;
            let clear_before_result = Blueprint::clear_before(connection, level)?;
            SqlResponse::NumbersofRowsAffected {
                number: clear_before_result,
            }
        }
        "insert_block" => {
            let level: i32 = extract_parameter(&query.params[0])?;
            let hash: Vec<u8> = extract_parameter(&query.params[1])?;
            let block: Vec<u8> = extract_parameter(&query.params[2])?;
            let block = Block { level, hash, block };
            let insert_result = block.insert(connection)?;
            SqlResponse::NumbersofRowsAffected {
                number: insert_result,
            }
        }
        "select_block_with_level" => {
            let id: i32 = extract_parameter(&query.params[0])?;
            let block = Block::select_with_level(connection, id)?;
            SqlResponse::BlockSelect { block }
        }
        "select_block_with_hash" => {
            let hash: Vec<u8> = extract_parameter(&query.params[0])?;
            let block = Block::select_with_hash(connection, &hash)?;
            SqlResponse::BlockSelect { block }
        }
        "select_block_hash_of_number" => {
            let id: i32 = extract_parameter(&query.params[0])?;
            let hash = Block::select_hash_of_number(connection, id)?;
            SqlResponse::BlockHashSelect { hash }
        }
        "select_block_number_of_hash" => {
            let hash: Vec<u8> = extract_parameter(&query.params[0])?;
            let id = Block::select_number_of_hash(connection, &hash)?;
            SqlResponse::BlockIdSelect { id }
        }
        "clear_after_block" => {
            let level: i32 = extract_parameter(&query.params[0])?;
            let clear_after_result = Block::clear_after(connection, level)?;
            SqlResponse::NumbersofRowsAffected {
                number: clear_after_result,
            }
        }
        "clear_before_block" => {
            let level: i32 = extract_parameter(&query.params[0])?;
            let clear_before_result = Block::clear_before(connection, level)?;
            SqlResponse::NumbersofRowsAffected {
                number: clear_before_result,
            }
        }
        "insert_pending_confirmation" => {
            let level: i32 = extract_parameter(&query.params[0])?;
            let hash: Vec<u8> = extract_parameter(&query.params[1])?;
            let pending_confirmation = PendingConfirmation { level, hash };
            let insertresult = pending_confirmation.insert(connection)?;
            SqlResponse::NumbersofRowsAffected {
                number: insertresult,
            }
        }
        "select_pending_confirmation_with_level" => {
            let level: i32 = extract_parameter(&query.params[0])?;
            let hash: Vec<u8> = PendingConfirmation::select_with_level(connection, level)?;
            SqlResponse::PendingConfirmationSelect { hash }
        }
        "delete_pending_confirmation_with_level" => {
            let level: i32 = extract_parameter(&query.params[0])?;
            let delete_with_level_result =
                PendingConfirmation::delete_with_level(connection, level)?;
            SqlResponse::NumbersofRowsAffected {
                number: delete_with_level_result,
            }
        }
        "clear_pending_confirmations" => {
            let clear_result = PendingConfirmation::clear(connection)?;
            SqlResponse::NumbersofRowsAffected {
                number: clear_result,
            }
        }
        "count_pending_confirmations" => {
            let count = PendingConfirmation::count(connection)?;
            SqlResponse::PendingConfirmationCount { count }
        }
        "insert_transaction" => {
            let block_hash: Vec<u8> = extract_parameter(&query.params[0])?;
            let block_number: i32 = extract_parameter(&query.params[1])?;
            let index_: i32 = extract_parameter(&query.params[2])?;
            let hash: Vec<u8> = extract_parameter(&query.params[3])?;
            let from_: Vec<u8> = extract_parameter(&query.params[4])?;
            let to_: Option<Vec<u8>> = extract_parameter(&query.params[5])?;
            let receipt_fields: Vec<u8> = extract_parameter(&query.params[6])?;
            let object_fields: Vec<u8> = extract_parameter(&query.params[7])?;
            let transaction = Transaction {
                block_hash,
                block_number,
                index_,
                hash,
                from_,
                to_,
                receipt_fields,
                object_fields,
            };
            let insert_result = transaction.insert(connection)?;
            SqlResponse::NumbersofRowsAffected {
                number: insert_result,
            }
        }
        "select_transaction_receipt" => {
            let hash: Vec<u8> = extract_parameter(&query.params[0])?;
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
            let block_number: i32 = extract_parameter(&query.params[0])?;
            let receipts =
                Transaction::select_receipts_from_block_number(connection, block_number)?;
            SqlResponse::TransactionReceipts { receipts }
        }
        "select_transaction_object" => {
            let hash: Vec<u8> = extract_parameter(&query.params[0])?;
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
            let block_number: i32 = extract_parameter(&query.params[0])?;
            let objects = Transaction::select_objects_from_block_number(connection, block_number)?;
            SqlResponse::TransactionObjects { objects }
        }
        "clear_transactions_after_block_number" => {
            let level: i32 = extract_parameter(&query.params[0])?;
            let clear_after_result = Transaction::clear_after(connection, level)?;
            SqlResponse::NumbersofRowsAffected {
                number: clear_after_result,
            }
        }
        "clear_transactions_before_block_number" => {
            let level: i32 = extract_parameter(&query.params[0])?;
            let clear_after_result = Transaction::clear_before(connection, level)?;
            SqlResponse::NumbersofRowsAffected {
                number: clear_after_result,
            }
        }
        "select_context_hash" => {
            let id: i32 = extract_parameter(&query.params[0])?;
            let context_hash = ContextHash::select(connection, id)?;
            SqlResponse::ContextHashSelect {
                context_hash: context_hash,
            }
        }
        "insert_context_hash" => {
            let id: i32 = extract_parameter(&query.params[0])?;
            let context_hash = extract_parameter(&query.params[1])?;
            let context_hash = ContextHash { id, context_hash };
            let inserted_result = context_hash.insert(connection)?;
            SqlResponse::NumbersofRowsAffected {
                number: inserted_result,
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
        "clear_after_context_hash" => {
            let level: i32 = extract_parameter(&query.params[0])?;
            let clear_result = ContextHash::clear_after(connection, level)?;
            SqlResponse::NumbersofRowsAffected {
                number: clear_result,
            }
        }
        "clear_before_context_hash" => {
            let level: i32 = extract_parameter(&query.params[0])?;
            let clear_result = ContextHash::clear_before(connection, level)?;
            SqlResponse::NumbersofRowsAffected {
                number: clear_result,
            }
        }
        "insert_smart_rollup_address" => {
            let address: String = extract_parameter(&query.params[0])?;
            let insert_result = Metadata::insert_smart_rollup_address(connection, &address)?;
            SqlResponse::NumbersofRowsAffected {
                number: insert_result,
            }
        }
        "get_smart_rollup_address" => {
            let address = Metadata::get_smart_rollup_address(connection)?;
            SqlResponse::MetadataSmartRollupAddress { address }
        }
        "insert_history_mode" => {
            let history_mode: String = extract_parameter(&query.params[0])?;
            let insert_result = Metadata::insert_history_mode(connection, &history_mode)?;
            SqlResponse::NumbersofRowsAffected {
                number: insert_result,
            }
        }
        "get_history_mode" => {
            let history_mode = Metadata::get_history_mode(connection)?;
            SqlResponse::MetadataHistoryMode { history_mode }
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
