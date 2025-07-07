use std::fmt::{self, Display, Formatter};
use actix_web::{Result,error, http::{header::ContentType, StatusCode}, post, web, App, HttpResponse, HttpServer, Responder};
use diesel::{result::Error as dieselError};
use evmnodetooling::dieselsqlite::{establish_connection,models::{Blueprint,Block}};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Error as jsonError;




#[derive(Deserialize)]
struct Sqlquery {
    name: String,
    params: serde_json::Value
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum SqlResponse{
    BlueprintSelect{ payload:Vec<u8>, timestamp:i32},
    BlueprintRangeSelect{idandpayloads:Vec<(i32,Vec<u8>)>},
    BlockSelect{block:Vec<u8>},
    BlockHashSelect{hash:Vec<u8>},
    BlockIdSelect{id:i32},
    NumbersofRowsAffected{number:usize},
}

#[derive(Debug)]
enum ServerError{
    InternalError{error:dieselError},
    UnknownMethod{method_name:String},
    BadParameterFormat{error:jsonError}
}

impl error::ResponseError for ServerError{
    fn error_response(&self)-> HttpResponse{
        HttpResponse::build(self.status_code())
        .insert_header(ContentType::html())
        .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ServerError::InternalError{ error: _ } => StatusCode::INTERNAL_SERVER_ERROR,
            ServerError::UnknownMethod{method_name:_} => StatusCode::BAD_REQUEST,
            ServerError::BadParameterFormat{error:_}=> StatusCode::BAD_REQUEST
        }
    }
}

impl Display for ServerError{
    fn fmt(&self, f:&mut Formatter)->fmt::Result{
        let string=match self{
            ServerError::InternalError { error } => format!("Internal database error:{}",error),
            ServerError::UnknownMethod { method_name }=>format!("Unknow method:{}",method_name),
            ServerError::BadParameterFormat { error }=>format!("Invalid parameters:{}",error),
        };
        write!(f,"{}", string)
    }
        
}


impl From<jsonError> for ServerError{
    fn from(error:jsonError)->ServerError{
        ServerError::BadParameterFormat{error}
    }
}

impl From<dieselError> for ServerError{
    fn from(error:dieselError)->ServerError{
        ServerError::InternalError{error}
    }
}

fn extract_parameter<T>(param:&serde_json::Value)->Result<T,jsonError> where T:DeserializeOwned{
    serde_json::from_value(param.clone())
}







#[post("/")]
async fn answer_query(query: web::Json<Sqlquery>) -> Result<impl Responder,ServerError>{
    let connection=&mut establish_connection();
    let method_requested=query.name.as_str();
    let response=match  method_requested{
            
            "select_blueprint"=>{
                let id:i32=extract_parameter(&query.params[0])?;
                let (payload,timestamp)=Blueprint::select(connection,id)?;
                SqlResponse::BlueprintSelect{payload,timestamp}}
            "insert_blueprint"=>{
                let id:i32=extract_parameter(&query.params[0])?;
                let payload:Vec<u8>=extract_parameter(&query.params[1])?;
                let timestamp:i32=extract_parameter(&query.params[2])?;
                let blueprint=Blueprint{
                    id,payload,timestamp
                };
                let insertresult=blueprint.insert(connection)?;
                SqlResponse::NumbersofRowsAffected { number:insertresult }
            }
            "select_blueprint_range"=>{
                let lowerlevel=extract_parameter(&query.params[0])?;
                let upperlevel=extract_parameter(&query.params[1])?;
                let idandpayloads=Blueprint::select_range(connection, lowerlevel, upperlevel)?;
                SqlResponse::BlueprintRangeSelect {idandpayloads}
            }
            "clear_after_blueprint"=>{
                let level:i32=extract_parameter(&query.params[0])?;
                let clear_after_result=Blueprint::clear_after(connection, level)?;
                SqlResponse::NumbersofRowsAffected { number:clear_after_result}
            }
            "clear_before_blueprint"=>{
                let level:i32=extract_parameter(&query.params[0])?;
                let clear_before_result=Blueprint::clear_before(connection, level)?;
                SqlResponse::NumbersofRowsAffected { number:clear_before_result}
            }
            "select_block_with_level"=>{
                let id:i32=extract_parameter(&query.params[0])?;
                let block=Block::select_with_level(connection, id)?;
                SqlResponse::BlockSelect{block}
            }
            "select_block_with_hash"=>{
                let hash:Vec<u8>=extract_parameter(&query.params[0])?;
                let block=Block::select_with_hash(connection, &hash)?;
                SqlResponse::BlockSelect{block}
            }
            "select_block_hash_of_number"=>{
                let id:i32=extract_parameter(&query.params[0])?;
                let hash=Block::select_hash_of_number(connection, id)?;
                SqlResponse::BlockHashSelect {hash}
            }
            "select_block_number_of_hash"=>{
                let hash:Vec<u8>=extract_parameter(&query.params[0])?;
                let id=Block::select_number_of_hash(connection, &hash)?;
                SqlResponse::BlockIdSelect{id}
            }
            "clear_after_block"=>{
                let level:i32=extract_parameter(&query.params[0])?;
                let clear_after_result=Block::clear_after(connection, level)?;
                SqlResponse::NumbersofRowsAffected { number:clear_after_result}
            }
            "clear_before_block"=>{
                let level:i32=extract_parameter(&query.params[0])?;
                let clear_before_result=Block::clear_before(connection, level)?;
                SqlResponse::NumbersofRowsAffected { number:clear_before_result}
            }
            _=>{
                return Err(ServerError::UnknownMethod{method_name:method_requested.to_string()});
            }
        };
    
    Ok(HttpResponse::Ok().json(response))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(answer_query) 
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await


}
