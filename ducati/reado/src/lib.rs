use suborbital::runnable::*;
use suborbital::db;
use suborbital::req;
use suborbital::db::query;

struct Readover{}

impl Runnable for Readover {
    fn run(&self, _: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        suborbital::resp::content_type("application/json; charset=utf-8");

        let key = req::url_param("key");

        let mut query_args: Vec<query::QueryArg> = Vec::new();
        query_args.push(query::QueryArg::new("id", key.as_str()));
        
        match db::select("SelectOneBike", query_args) {
            Ok(result) => Ok(result),
            Err(e) => {
                Err(RunErr::new(500, e.message.as_str()))
            }
        }
    }
}

// initialize the runner, do not edit below //
static RUNNABLE: &Readover = &Readover{};

#[no_mangle]
pub extern fn _start() {
    use_runnable(RUNNABLE);
}
