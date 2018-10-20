use mysql::{self, params};

extern crate time;

mod database;
mod snowflake;

use crate::snowflake::SnowFlakeId;

use self::database::{
    WorkerNode, 
    MYSQL_CONN,
    SQL_CREATE_DATABASE,
    SQL_CREATE_TABLE_WOKER_ENV,
    SQL_CREATE_TABLE_WOKER_NODE,
    SQL_UPDATE_INSERT_WORKER_ENV,
};

fn main() {
    println!("{:x}", -1i64 ^ (-1i64 << 12));
    println!("macro timestamp: {}", SnowFlakeId::curr_time_macro());
    let pool = mysql::Pool::new(MYSQL_CONN); 
    match pool {
        Ok(ref _r) => println!("exec ok"),
        Err(ref e) => {
            println!("{}", e.to_string());
            return;
        },
    }

    let pool = pool.unwrap();
    match pool.prep_exec(SQL_CREATE_DATABASE, ()) {
        Ok(_r) => println!("create database ok"),
        Err(e) => println!("{}", e.to_string()),
    }

    match pool.prep_exec(SQL_CREATE_TABLE_WOKER_ENV, ()) {
        Ok(_r) => println!("create worker env table ok"),
        Err(e) => println!("{}", e.to_string()),
    }

    match pool.prep_exec(SQL_CREATE_TABLE_WOKER_NODE,()) {
        Ok(_r) => println!("create worker node table ok"),
        Err(e) => println!("{}", e.to_string()),
    }

    match pool.prep_exec(SQL_UPDATE_INSERT_WORKER_ENV, params!{"id"=>1u32, "is_busy"=>1u32}) {
        Ok(_r) => println!("update worker env data ok"),
        Err(e) => println!("{}", e.to_string()),
    }

    match pool.prep_exec(SQL_UPDATE_INSERT_WORKER_ENV, params!{"id"=>1u32, "is_busy"=>0u32}) {
        Ok(_r) => println!("update worker env data ok"),
        Err(e) => println!("{}", e.to_string()),
    }
    let nodes = vec![
        WorkerNode::new( 1, false,  None ),
        WorkerNode::new( 3, false, Some("foo".into()) ),
        WorkerNode::new( 5, false, None ),
        WorkerNode::new( 7, false, None ),
        WorkerNode::new( 9, false, Some("bar".into()) ),
    ];

//    for mut stmt in pool.prepare(SQL_INSERT_PAYMENT).into_iter() {
//        for p in nodes.iter() {
//            stmt.execute(params!{
//                "worker_id" => p.worker_id,
//                "is_online" => p.is_online,
//                "content" => &p.content,
//            }).unwrap();
//        }
//    }
}
