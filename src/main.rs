pub(crate) mod fault;
pub(crate) mod lint;
pub(crate) mod stage;

use crate::fault::{Fault, Message};
use crate::stage::Report;

use crossbeam::channel::unbounded;
use std::mem::drop;

#[tokio::main]
async fn main() {
    // TODO This is only a proof of concept for the staged architecture!

    let (s, r) = unbounded();
    let report = Report { r#in: r }.spawn();

    for i in 0..10_000 {
        s.send(Fault::simple(Message::bare(format!("{}", i)))).unwrap();
    }
    drop(s);

    report.await.unwrap();
}
