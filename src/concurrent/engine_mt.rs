use std::thread;

use crossbeam_channel::{unbounded, Receiver, Sender};

use crate::{engine::MatchingEngine, types::{Order, Trade}};

/// A simple concurrent engine pipeline that decouples order ingestion from matching.
#[derive(Debug)]
pub struct ConcurrentEngine {
    order_sender: Option<Sender<Order>>,
    trade_receiver: Receiver<Trade>,
    worker_handle: Option<thread::JoinHandle<()>>,
}

impl ConcurrentEngine {
    pub fn new() -> Self {
        let (order_sender, order_receiver) = unbounded();
        let (trade_sender, trade_receiver) = unbounded();

        let worker_handle = thread::spawn(move || {
            let mut engine = MatchingEngine::new();
            for order in order_receiver.iter() {
                let trades = engine.process(order);
                for trade in trades {
                    let _ = trade_sender.send(trade);
                }
            }
        });

        Self {
            order_sender: Some(order_sender),
            trade_receiver,
            worker_handle: Some(worker_handle),
        }
    }

    pub fn order_sender(&self) -> Sender<Order> {
        self.order_sender
            .as_ref()
            .expect("Concurrent engine sender already closed")
            .clone()
    }

    pub fn trade_receiver(&self) -> Receiver<Trade> {
        self.trade_receiver.clone()
    }
}

impl Drop for ConcurrentEngine {
    fn drop(&mut self) {
        self.order_sender.take();
        if let Some(handle) = self.worker_handle.take() {
            let _ = handle.join();
        }
    }
}
