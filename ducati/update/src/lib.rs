use suborbital::runnable::*;

use suborbital::db;
use suborbital::req;
use suborbital::db::query;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]

struct Bike {
    modelo: String,
    cor: String,
    cilindrada: String
}

struct Update{}

impl Runnable for Update {
    fn run(&self, input: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        suborbital::resp::content_type("application/json; charset=utf-8");

        let in_string = String::from_utf8(input).unwrap();
        let bikes: Bikes = serde_json::from_str(&in_string).unwrap();

        let key = req::url_param("key");

        let mut query_args: Vec<query::QueryArg> = Vec::new();

        query_args.push(query::QueryArg::new("Modelo", bikes.modelo.as_str()));
        query_args.push(query::QueryArg::new("Cor", bikes.cor.as_str()));
        query_args.push(query::QueryArg::new("Cilindrada", bikes.cilindrada.as_str()));
        query_args.push(query::QueryArg::new("id", key.as_str()));

        match db::update("UpdateBikes", query_args) {
            Ok(result) => Ok(result),
            Err(e) => {
                Err(RunErr::new(500, e.message.as_str()))
            }
        }
    }
}

// initialize the runner, do not edit below //
static RUNNABLE: &Update = &Update{};

#[no_mangle]
pub extern fn _start() {
    use_runnable(RUNNABLE);
}
