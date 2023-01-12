

mod service;

use actix_web::{web, get, post, HttpResponse, Responder, HttpRequest};
use serde::{Deserialize, Serialize};
use actix_web::web::Json;



#[derive(Deserialize, Debug)]
struct PersonReq{
    name: String,
    age: i32
}

#[derive(Deserialize, Serialize, Debug)]
struct PersonResp{
    id:  String,
    name: String,
    age: i32
}

#[derive(Serialize, Debug)]
struct SuccessVo{
    code: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PersonQueryReq{
    name: String,
    age: i32
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NullResp{

}


// 查询单个
#[get("/person/{id}")]
pub async fn get_person(id: web::Path<String>) -> impl Responder {
    let option = service::person_service::query_single(id.clone());
    match option{
        Some(person) => {
            let p0 = PersonResp{id: person.id.clone(), name: person.name.clone(), age: person.age };
            HttpResponse::Ok().json(p0)
        },
        _ => HttpResponse::Ok().json(NullResp{})
    }

}

//查询列表
#[get("/person")]
pub async fn get_person_list(query: web::Query<PersonQueryReq>) -> impl Responder {
    let vec = service::person_service::query_list(query.name.clone(), query.age);
    let mut r: Vec<PersonResp> = Vec::with_capacity(vec.len());
    for i in vec {
        let p0 = PersonResp{id: i.id.clone(), name: i.name, age: i.age };
        r.push(p0);
    }
    HttpResponse::Ok().json(r)
}



//保存
#[post("/person")]
pub async fn add_person(body: Json<PersonReq>) -> impl Responder {
    let req = body.into_inner();
    println!("req = {:?}", req);
    service::person_service::save(req.name, req.age);
    let string = String::from("success");
    let s = SuccessVo{ code: string};
    HttpResponse::Ok().json(&s)
}

//修改

//删除
