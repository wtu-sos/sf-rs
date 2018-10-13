use mysql::{self, params};

const MYSQL_CONN : &str = "mysql://root:111111@localhost:3306/snowflake";

const SQL_CREATE_DATABASE: &str = r"CREATE DATABASE IF NOT EXISTS `snowflake` DEFAULT CHARACTER SET utf8 COLLATE utf8_general_ci;";

const SQL_CREATE_TABLE_WOKER_ENV : &str = r"
    CREATE TABLE IF NOT EXISTS `worker_env` (
    id INT not null primary key,
    is_busy tinyint not null,
    begin_time Datetime
    ) ENGINE=InnoDB DEFAULT CHARSET=utf8;";

const SQL_UPDATE_INSERT_WORKER_ENV : &str = r"
    insert into worker_env (id, begin_time, is_busy)
    value (:id, now(), :is_busy) 
    on duplicate key update is_busy=:is_busy;";

const SQL_CREATE_TABLE_WOKER_NODE : &str = r"
    CREATE TABLE IF NOT EXISTS `worker_node` (
    worker_id int not null primary key, 
    is_online bool not null, 
    content varchar(128))
    ENGINE=InnoDB DEFAULT CHARSET=utf8;";

const SQL_INSERT_PAYMENT : &str = r"
    INSERT INTO woker_node 
        (worker_id, is_online, content) 
    VALUES 
        (:worker_id, :is_online, :content)";

#[derive(Debug, PartialEq, Eq)]
struct WorkerNode {
    worker_id: i32,
    is_online: bool,
    content: Option<String>,
}

fn main() {
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
        WorkerNode { worker_id: 1, is_online: false, content: None },
        WorkerNode { worker_id: 3, is_online: false, content: Some("foo".into()) },
        WorkerNode { worker_id: 5, is_online: false, content: None },
        WorkerNode { worker_id: 7, is_online: false, content: None },
        WorkerNode { worker_id: 9, is_online: false, content: Some("bar".into()) },
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
