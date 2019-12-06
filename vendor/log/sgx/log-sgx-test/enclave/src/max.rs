use std::prelude::v1::*;
use std::sync::{Arc, SgxMutex as Mutex};
use log::{Level, LevelFilter, Log, Record, Metadata};

//#[cfg(feature = "std")]
//use log::set_boxed_logger;
//#[cfg(not(feature = "std"))]
fn set_boxed_logger(logger: Box<Log>) -> Result<(), log::SetLoggerError> {
    unsafe {
        log::set_logger(&*Box::into_raw(logger))
    }
}

#[derive(Debug)]
struct State {
    last_log: Mutex<Option<Level>>,
}

struct Logger(Arc<State>);

impl Log for Logger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        *self.0.last_log.lock().unwrap() = Some(record.level());
    }

    fn flush(&self) {}
}

pub fn max_main() {
    let me = Arc::new(State { last_log: Mutex::new(None) });
    let a = me.clone();
    set_boxed_logger(Box::new(Logger(me))).unwrap();

    test(&a, LevelFilter::Off);
    test(&a, LevelFilter::Error);
    test(&a, LevelFilter::Warn);
    test(&a, LevelFilter::Info);
    test(&a, LevelFilter::Debug);
    test(&a, LevelFilter::Trace);
    println!("finished max_main");
}

fn test(a: &State, filter: LevelFilter) {
    // Attention:
    // In the original test, the dependency is configured as
    // features = ["max_level_debug", "release_max_level_info"]
    // which means that in debug build, the max level is debug
    // and if in release build, the max level is info.
    // Since it's an isolated test, the author can do this.
    // However, we don't have such a chance. So we do conditional
    // compilation here.
    #[cfg(not(debug_assertions))]
    {
        if filter > LevelFilter::Info {
            log::set_max_level(LevelFilter::Info);
        } else {
            log::set_max_level(filter);
        }
    }

    #[cfg(debug_assertions)]
    {
        if filter > LevelFilter::Debug {
            log::set_max_level(LevelFilter::Debug);
        } else {
            log::set_max_level(filter);
        }
    }

    error!("");
    last(&a, t(Level::Error, filter));
    warn!("");
    last(&a, t(Level::Warn, filter));
    info!("");
    last(&a, t(Level::Info, filter));

    debug!("");
    if cfg!(debug_assertions) {
        last(&a, t(Level::Debug, filter));
    } else {
        last(&a, None);
    }

    trace!("");
    last(&a, None);

    fn t(lvl: Level, filter: LevelFilter) -> Option<Level> {
        if lvl <= filter {Some(lvl)} else {None}
    }
}

fn last(state: &State, expected: Option<Level>) {
    let mut lvl = state.last_log.lock().unwrap();
    assert_eq!(*lvl, expected);
    *lvl = None;
}
