use suborbital::runnable::*;
use suborbital::db;

struct Read{}

impl Runnable for Read {
    fn run(&self, _: Vec<u8>) -> Result<Vec<u8>, RunErr> {
        suborbital::resp::content_type("application/json; charset=utf-8");
        
        match db::select("SelectAllBikes", Vec::new()) {
            Ok(result) => Ok(result),
            Err(e) => {
                Err(RunErr::new(500, e.message.as_str()))
            }
        }
    }
}

static RUNNABLE: &Read = &Read{};

#[no_mangle]
pub extern fn _start() {
    use_runnable(RUNNABLE);
}
