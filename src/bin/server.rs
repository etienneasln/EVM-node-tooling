use actix_web::{post, web, App, HttpResponse, HttpServer, Responder, Result};
use evmnodetooling::dieselsqlite::{establish_connection,models::{Blueprint}};
use serde::{Deserialize,Serialize};




#[derive(Deserialize)]
struct Sqlquery {
    name: String,
    params: serde_json::Value
}

#[derive(Serialize)]
struct SqlResponse {
    result:(Vec<u8>,i32)
}



#[post("/")]
async fn answer_query(query: web::Json<Sqlquery>) -> impl Responder{
    let mut connection=establish_connection();
    let result=match query.name.as_str() {
            
            "select_blueprint"=>{
                let id:i32=serde_json::from_value(query.params[0].clone()).unwrap();
                   Blueprint::select(&mut connection,id)}
            _=>{(Vec::new(),0)}
        };
    let response=SqlResponse{
        result
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
