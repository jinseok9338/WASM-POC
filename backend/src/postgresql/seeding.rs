use deadpool_postgres::Object;
use std::fs::read_to_string;

pub async fn seeding (db_con: Object){
    let clear = db_con.batch_execute("DROP TABLE IF EXISTS users;").await;
    match clear {
        Ok(_) => println!("Cleared DataBase"),
        Err(e) => {
            println!("Error clearing database: {}", e);
            std::process::exit(1);
        }
    };

        // read seed.sql file
let sql_file_content = read_to_string("seed.sql").unwrap();
let seeding = db_con.batch_execute(&sql_file_content).await;

match seeding {
    Ok(_) => println!("Seeding the database"),
    Err(e) => {
        println!("Error seeding database: {}", e);
        std::process::exit(1);
    }
    };
}