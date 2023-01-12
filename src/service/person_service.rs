

use mysql::*;
use mysql::prelude::*;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;
#[derive(Debug)]
pub struct Person{
    pub id: String,
    pub name: String,
    pub age: i32
}
#[derive(Debug, Serialize, Deserialize)]
struct MysqlConfig{
    host: String,
    username: String,
    pwd: String
}

lazy_static! {
    static ref POOL: Pool = {
        let yaml_str = include_str!("../application.yml");
        let config: MysqlConfig = serde_yaml::from_str(yaml_str).expect("app.yaml read failed!");
        println!("mysql config = {:?}" , config );
        let mut mysql_uri = "mysql://".to_string();
        mysql_uri.push_str(config.username.as_str());
        mysql_uri.push_str(":");
        mysql_uri.push_str(config.pwd.as_str());
        mysql_uri.push_str("@");
        mysql_uri.push_str(config.host.as_str());
        mysql_uri.push_str("/rust_db");
        println!("mysql uri = {}", mysql_uri);
        Pool::new(mysql_uri.as_str()).unwrap()
    };
}
// 查询列表
pub fn query_list(name: String, age: i32) -> Vec<Person>{
    let mut conn = POOL.get_conn().unwrap();
    conn.query_map("select id, name, age from person ", |(id, name, age)| {
        Person{id, name, age}
    } ).unwrap()
}
// 查询单个
pub fn query_single(id: String) -> Option<Person>{
    let mut conn = POOL.get_conn().unwrap();

    let vec = conn.exec_map("select id, name, age from person where id = :id", params! {"id" => id}, |(id, name, age)| {
        Person { id, name, age }
    }).unwrap();

    let option = vec.first();
    match option {
        None => Option::None,
        Some(p) => Option::Some(Person{id: p.id.clone(), name: p.name.clone(), age: p.age})
    }

}
// 保存
pub fn save(name: String, age: i32){
    println!("name = {} age = {}", name, age);
    let mut conn = POOL.get_conn().unwrap();
    let uuid = Uuid::new_v4().to_string();
    // 这里直接使用 ? 占位也 可以 ？？？ 我怎么知道要用这个 宏，  为什么不直接封装进去？？
    let result = conn.exec_drop("insert into person values(:uuid , :name, :age)", params!{
        "uuid" => uuid,
        "name" => name,
        "age" => age
    });
    // let result222 = conn.exec_drop("insert into person values(? , ?, ?)", (uuid, name, age));
}
