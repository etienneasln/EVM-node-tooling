use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use evmnodetooling::dieselsqlite::{establish_connection,models::{Blueprint,Block}};
use serde::{de::DeserializeOwned, Deserialize, Serialize};




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
    MethodNotSupported{message:String}
}

fn extract_parameter<T>(param:&serde_json::Value)->T where T:DeserializeOwned{
    serde_json::from_value(param.clone())
    .unwrap_or_else(|e| panic!("Error extracting parameter:{e}"))
}




#[post("/")]
async fn answer_query(query: web::Json<Sqlquery>) -> impl Responder{
    let mut connection=establish_connection();
    let method_requested=query.name.as_str();
    let response=match  method_requested{
            
            "select_blueprint"=>{
                let id:i32=extract_parameter(&query.params[0]);
                let (payload,timestamp)=Blueprint::select(&mut connection,id);
                SqlResponse::BlueprintSelect{payload,timestamp}}
            "insert_blueprint"=>{
                let id:i32=extract_parameter(&query.params[0]);
                let payload:Vec<u8>=extract_parameter(&query.params[1]);
                let timestamp:i32=extract_parameter(&query.params[2]);
                let insertresult=Blueprint::insert(&mut connection, id, &payload, timestamp);
                SqlResponse::NumbersofRowsAffected { number:insertresult }
            }
            "select_blueprint_range"=>{
                let lowerlevel=extract_parameter(&query.params[0]);
                let upperlevel=extract_parameter(&query.params[1]);
                let idandpayloads=Blueprint::select_range(&mut connection, lowerlevel, upperlevel);
                SqlResponse::BlueprintRangeSelect {idandpayloads}
            }
            "clear_after_blueprint"=>{
                let level:i32=extract_parameter(&query.params[0]);
                let clear_after_result=Blueprint::clear_after(&mut connection, level);
                SqlResponse::NumbersofRowsAffected { number:clear_after_result}
            }
            "clear_before_blueprint"=>{
                let level:i32=extract_parameter(&query.params[0]);
                let clear_before_result=Blueprint::clear_before(&mut connection, level);
                SqlResponse::NumbersofRowsAffected { number:clear_before_result}
            }
            "select_block_with_level"=>{
                let id:i32=extract_parameter(&query.params[0]);
                let block=Block::select_with_level(&mut connection, id);
                SqlResponse::BlockSelect{block}
            }
            "select_block_with_hash"=>{
                let hash:Vec<u8>=extract_parameter(&query.params[0]);
                let block=Block::select_with_hash(&mut connection, &hash);
                SqlResponse::BlockSelect{block}
            }
            "select_block_hash_of_number"=>{
                let id:i32=extract_parameter(&query.params[0]);
                let hash=Block::select_hash_of_number(&mut connection, id);
                SqlResponse::BlockHashSelect {hash}
            }
            "select_block_number_of_hash"=>{
                let hash:Vec<u8>=extract_parameter(&query.params[0]);
                let id=Block::select_number_of_hash(&mut connection, &hash);
                SqlResponse::BlockIdSelect{id}
            }
            "clear_after_block"=>{
                let level:i32=extract_parameter(&query.params[0]);
                let clear_after_result=Block::clear_after(&mut connection, level);
                SqlResponse::NumbersofRowsAffected { number:clear_after_result}
            }
            "clear_before_block"=>{
                let level:i32=extract_parameter(&query.params[0]);
                let clear_before_result=Block::clear_before(&mut connection, level);
                SqlResponse::NumbersofRowsAffected { number:clear_before_result}
            }
            _=>{
                let response=SqlResponse::MethodNotSupported{
                    message:format!("Specified request {method_requested} is not supported")
                };
                return HttpResponse::BadRequest().json(response)
            }
        };
    
    HttpResponse::Ok().json(response)
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
