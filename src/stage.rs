use crate::fault::Fault;

use crossbeam::channel::{Receiver, Sender, RecvError, SendError, TryRecvError, TrySendError};
use std::mem::replace;
use tokio::io::{AsyncWriteExt, BufWriter, stdout};
use tokio::task::{JoinHandle, yield_now};

pub(crate) async fn async_send<T>(sender: &Sender<T>, msg: T) -> Result<(), SendError<T>> {
    let mut optional = Some(msg);
    loop {
        match sender.try_send(replace(&mut optional, None).unwrap()) {
            Ok(()) => return Ok(()),
            Err(TrySendError::Full(msg)) => {
                replace(&mut optional, Some(msg));
                yield_now().await;
            }
            Err(TrySendError::Disconnected(msg)) => return Err(SendError(msg)),
        }
    }
}

pub(crate) async fn async_recv<T>(receiver: &Receiver<T>) -> Result<T, RecvError> {
    loop {
        match receiver.try_recv() {
            Ok(msg) => return Ok(msg),
            Err(TryRecvError::Empty) => yield_now().await,
            Err(TryRecvError::Disconnected) => return Err(RecvError),
        }
    }
}

pub(crate) struct Report {
    pub(crate) r#in: Receiver<Fault>,
}

impl Report {
    pub(crate) fn spawn(self) -> JoinHandle<()> {
        tokio::spawn(async move {
            let mut out = BufWriter::new(stdout());

            while let Ok(msg) = async_recv(&self.r#in).await {
                let str = format!("{}", msg); // TODO Is there a more efficient solution?
                out.write_all(str.as_bytes()).await.unwrap();
            }

            out.flush().await.unwrap();
        })
    }
}
