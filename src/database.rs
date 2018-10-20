use mysql::{self, params};

pub const MYSQL_CONN : &str = "mysql://root:111111@localhost:3306/snowflake";

pub const SQL_CREATE_DATABASE: &str = r"CREATE DATABASE IF NOT EXISTS `snowflake` DEFAULT CHARACTER SET utf8 COLLATE utf8_general_ci;";

pub const SQL_CREATE_TABLE_WOKER_ENV : &str = r"
    CREATE TABLE IF NOT EXISTS `worker_env` (
    id INT not null primary key,
    is_busy tinyint not null,
    begin_time Datetime
    ) ENGINE=InnoDB DEFAULT CHARSET=utf8;";

pub const SQL_UPDATE_INSERT_WORKER_ENV : &str = r"
    insert into worker_env (id, begin_time, is_busy)
    value (:id, now(), :is_busy) 
    on duplicate key update is_busy=:is_busy;";

pub const SQL_CREATE_TABLE_WOKER_NODE : &str = r"
    CREATE TABLE IF NOT EXISTS `worker_node` (
    worker_id int not null primary key, 
    is_online bool not null, 
    content varchar(128))
    ENGINE=InnoDB DEFAULT CHARSET=utf8;";

pub const SQL_INSERT_PAYMENT : &str = r"
    INSERT INTO woker_node 
        (worker_id, is_online, content) 
    VALUES 
        (:worker_id, :is_online, :content)";

#[derive(Debug, PartialEq, Eq)]
        pub struct WorkerNode {
            worker_id: i32,
            is_online: bool,
            content: Option<String>,
        }

impl WorkerNode {
    pub fn new(worker_id: i32, is_online: bool, content: Option<String>) -> Self {
        WorkerNode {
            worker_id,
            is_online,
            content,
        }
    }
}

struct Database {
    pool: mysql::Pool,
}

impl Database {
    pub fn new () -> Database {
        let pool = mysql::Pool::new(MYSQL_CONN); 
        match pool {
            Ok(ref _r) => println!("exec ok"),
            Err(ref e) => {
                panic!("{}", e.to_string());
            },
        }

        Database {
            pool: pool.unwrap(),
        }
    }
}

