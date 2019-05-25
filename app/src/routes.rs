use rocket_contrib::databases::postgres;
use rocket_contrib::json::Json;
use uuid::Uuid;
use serde::{Deserialize};

#[database("pg_db")]
pub struct DbConn(postgres::Connection);

pub struct Entry {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct EntryUpsert {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
}

#[get("/ping")]
pub fn ping() -> &'static str {
    "Hello there from the /ping endpoint!"
}

#[get("/test")]
pub fn test(conn: DbConn) -> String {
    let results = conn.query("SELECT * FROM test", &[]);
    if results.is_err() {
        return String::from("An internal server error ocurred");
    }

    match results {
        Ok(rows) => {
            if rows.len() == 0 {
                return String::from("No results");
            }

            let mut ret = Vec::with_capacity(rows.len());
            for row in &rows {
                let res = Entry {
                    firstname: row.get(0),
                    lastname: row.get(1),
                    email: row.get(2),
                    id: row.get(3),
                };

                ret.push(format!("{} {} {} {}", res.firstname, res.lastname, res.email, res.id));
            }

            return ret.join("\n");
        },
        Err(e) => {
            return format!("An internal server error occurred: {}", e);
        }
    }
}

#[post("/upsert", format="json", data="<entry>")]
pub fn post(conn: DbConn, entry: Json<EntryUpsert>) -> Result<String, String> {
    println!("{:#?}", entry);

    let result = conn.execute(
        "INSERT INTO
            test(firstname, lastname, email, id)
        VALUES(
            $1, $2, $3, uuid_generate_v4()
        )",
        &[&entry.firstname, &entry.lastname, &entry.email]);

    match result {
        Ok(_) => {
            Ok(format!("{:#?}", entry))
        },
        Err(e) => {
            Err(format!("An internal server error occurred: {}", e))
        }
    }
}
