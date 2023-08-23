macro_rules! map_error {
    ($msg: expr) => {
        |error| Rc::from(format!($msg, error))
    };
}

pub(crate) use map_error;
