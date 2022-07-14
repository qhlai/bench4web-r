
use std::time::{SystemTime, UNIX_EPOCH};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

//use async_std::stream::Stream;
use tokio_stream::{self as Stream, StreamExt};
use futures_timer::Delay;

fn duration_to_nanos(dur: Duration) -> Option<u64> {
    dur.as_secs()
        .checked_mul(1_000_000_000)
        .and_then(|v| v.checked_add(u64::from(dur.subsec_nanos())))
}

fn next_interval(prev: Instant, now: Instant, interval: Duration) -> Instant {
    let new = prev + interval;
    if new > now {
        return new;
    }

    let spent_ns = duration_to_nanos(now.duration_since(prev)).expect("interval should be expired");
    let interval_ns =
        duration_to_nanos(interval).expect("interval is less that 427 thousand years");
    let mult = spent_ns / interval_ns + 1;
    assert!(
        mult < (1 << 32),
        "can't skip more than 4 billion intervals of {:?} \
         (trying to skip {})",
        interval,
        mult
    );
    prev + interval * (mult as u32)
}

pub fn time_stamp() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let ms = since_the_epoch.as_secs() as i64 * 1000i64 + (since_the_epoch.subsec_nanos() as f64 / 1_000_000.0) as i64;
    ms
}
