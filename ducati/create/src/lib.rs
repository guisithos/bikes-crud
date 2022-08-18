use suborbital::runnable::*;
use suborbital::db;
use suborbital::db::query;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Ducati {
    modelo: String,
    cor: String,
    cilindradas: String
}

struct Create{}

impl Runnable for Create {
    fn run(&self, input: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        suborbital::resp::content_type("application/json; charset=utf-8");

        let in_string = String::from_utf8(input).unwrap();
        // read the JSON message
        let ducati: Ducati = serde_json::from_str(&in_string).unwrap();
        // add parameters
        let mut query_args: Vec<query::QueryArg> = Vec::new();
        query_args.push(query::QueryArg::new("Modelo", ducati.modelo.as_str()));
        query_args.push(query::QueryArg::new("Cor", ducati.cor.as_str()));
        query_args.push(query::QueryArg::new("Cilindradas", ducati.cilindradas.as_str()));

        match db::insert("AddDucati", query_args) {
            Ok(result) => Ok(result),
            Err(e) => {
                Err(RunErr::new(500, e.message.as_str()))
            }
        }
    }
}

// initialize the runner, do not edit below //
static RUNNABLE: &Create = &Create{};

#[no_mangle]
pub extern fn _start() {
    use_runnable(RUNNABLE);
}
