use std::any::Any;

pub trait Limit {
    fn get_limit() -> u32;

    fn notify_on_change(f: dyn Fn(i32) -> ());

    fn on_sample(start_time: i64, rtt: i64, inflight: i32, did_drop: bool);
}



pub trait Listener {

    fn on_success();

    fn on_ignore();

    fn on_dropped();
}

struct Limiter<L> {
    limit: Box<L>,
}

impl<'a, L: Limit, C: Any> Limiter<L>
where
    L: Limit,
    C: Any,
{
    fn acquire(context: C) -> Option<dyn Listener> {
        None
    }
}
