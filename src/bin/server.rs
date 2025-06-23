use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use evmnodetooling::dieselsqlite::{establish_connection,models::{Blueprint,Block}};
use serde::{Deserialize,Serialize};

// use rpcdieselserver::dieselsqlite::{models::Block, TOP_LEVEL};



#[derive(Deserialize)]
struct Sqlquery {
    name: String,
    params: serde_json::Value
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum SqlResponse {
    BlueprintSelect{ payload:Vec<u8>, timestamp:i32},
    BlockHashSelect{hash:Vec<u8>},
    Other
}





#[post("/")]
async fn answer_query(query: web::Json<Sqlquery>) -> impl Responder{
    let mut connection=establish_connection();
    let response=match query.name.as_str() {
            
            "select_blueprint"=>{
                let id:i32=serde_json::from_value(query.params[0].clone()).unwrap();
                let (payload,timestamp)=Blueprint::select(&mut connection,id);
                SqlResponse::BlueprintSelect{payload,timestamp}},
            "select_block_from_level"=>{
                let id:i32=serde_json::from_value(query.params[0].clone()).unwrap();
                let hash=Block::select_hash_of_number(&mut connection, id);
                SqlResponse::BlockHashSelect {hash: hash}
                
            }
            _=>SqlResponse::Other
        };
    
    HttpResponse::Ok().json(response)
}

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

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

// fn main(){
//     let mut connection=establish_connection();

//     let _=Block::clear_after(&mut connection,TOP_LEVEL);
// }