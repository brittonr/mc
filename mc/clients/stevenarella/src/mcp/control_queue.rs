// Copyright 2026
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.

use crate::control::{ControlCommand, ControlResponse};
use std::sync::mpsc;
use std::time::Duration;

pub const MAX_MCP_COMMANDS_PER_FRAME: usize = 64;

const COMMAND_RESPONSE_TIMEOUT_MILLIS: u64 = 30_000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum McpCommandQueueError {
    QueueClosed,
    ResponseDropped,
    ResponseTimedOut,
}

#[derive(Clone)]
pub struct McpCommandSender {
    sender: mpsc::Sender<QueuedMcpCommand>,
    response_timeout: Duration,
}

pub struct McpCommandReceiver {
    receiver: mpsc::Receiver<QueuedMcpCommand>,
}

pub struct QueuedMcpCommand {
    command: ControlCommand,
    response_sender: mpsc::Sender<ControlResponse>,
}

pub fn control_command_channel() -> (McpCommandSender, McpCommandReceiver) {
    let (sender, receiver) = mpsc::channel();
    (
        McpCommandSender {
            sender,
            response_timeout: Duration::from_millis(COMMAND_RESPONSE_TIMEOUT_MILLIS),
        },
        McpCommandReceiver { receiver },
    )
}

impl McpCommandSender {
    pub fn enqueue_deferred(
        &self,
        command: ControlCommand,
    ) -> Result<mpsc::Receiver<ControlResponse>, McpCommandQueueError> {
        let (response_sender, response_receiver) = mpsc::channel();
        self.sender
            .send(QueuedMcpCommand {
                command,
                response_sender,
            })
            .map_err(|_| McpCommandQueueError::QueueClosed)?;
        Ok(response_receiver)
    }

    pub fn enqueue(
        &self,
        command: ControlCommand,
    ) -> Result<ControlResponse, McpCommandQueueError> {
        let response_receiver = self.enqueue_deferred(command)?;
        match response_receiver.recv_timeout(self.response_timeout) {
            Ok(response) => Ok(response),
            Err(mpsc::RecvTimeoutError::Timeout) => Err(McpCommandQueueError::ResponseTimedOut),
            Err(mpsc::RecvTimeoutError::Disconnected) => Err(McpCommandQueueError::ResponseDropped),
        }
    }

    pub fn with_response_timeout(mut self, response_timeout: Duration) -> Self {
        self.response_timeout = response_timeout;
        self
    }
}

impl McpCommandReceiver {
    pub fn drain_pending_with_handler<F>(&self, handler: F) -> usize
    where
        F: FnMut(ControlCommand) -> ControlResponse,
    {
        self.drain_pending_with_limit(MAX_MCP_COMMANDS_PER_FRAME, handler)
    }

    pub fn drain_pending_with_limit<F>(&self, limit: usize, mut handler: F) -> usize
    where
        F: FnMut(ControlCommand) -> ControlResponse,
    {
        let mut drained = 0;
        while drained < limit {
            let queued = match self.receiver.try_recv() {
                Ok(queued) => queued,
                Err(mpsc::TryRecvError::Empty) | Err(mpsc::TryRecvError::Disconnected) => break,
            };
            let response = handler(queued.command);
            let _ = queued.response_sender.send(response);
            drained += 1;
        }
        drained
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::control::ControlOutcome;

    const QUEUE_TEST_RESPONSE: &str = "main-thread-handler";
    const SINGLE_COMMAND_LIMIT: usize = 1;
    const QUEUE_TEST_TIMEOUT_MILLIS: u64 = 1;

    #[test]
    fn command_queue_drains_pending_command_with_main_thread_handler() {
        let (sender, receiver) = control_command_channel();
        let response_receiver = sender
            .enqueue_deferred(ControlCommand::Status)
            .expect("queue should accept command while receiver is alive");

        let drained = receiver.drain_pending_with_handler(|command| {
            assert_eq!(command, ControlCommand::Status);
            ControlResponse {
                outcome: ControlOutcome::Applied,
                message: Some(QUEUE_TEST_RESPONSE.to_owned()),
            }
        });

        assert_eq!(drained, 1);
        assert_eq!(
            response_receiver.recv().unwrap(),
            ControlResponse {
                outcome: ControlOutcome::Applied,
                message: Some(QUEUE_TEST_RESPONSE.to_owned()),
            }
        );
    }

    #[test]
    fn command_queue_respects_per_frame_drain_limit() {
        let (sender, receiver) = control_command_channel();
        let first_response = sender.enqueue_deferred(ControlCommand::Status).unwrap();
        let second_response = sender.enqueue_deferred(ControlCommand::Disconnect).unwrap();

        let first_drained = receiver.drain_pending_with_limit(SINGLE_COMMAND_LIMIT, |command| {
            assert_eq!(command, ControlCommand::Status);
            ControlResponse {
                outcome: ControlOutcome::Applied,
                message: None,
            }
        });

        assert_eq!(first_drained, 1);
        assert_eq!(
            first_response.recv().unwrap().outcome,
            ControlOutcome::Applied
        );
        assert!(matches!(
            second_response.try_recv(),
            Err(mpsc::TryRecvError::Empty)
        ));

        let second_drained = receiver.drain_pending_with_handler(|command| {
            assert_eq!(command, ControlCommand::Disconnect);
            ControlResponse {
                outcome: ControlOutcome::Rejected,
                message: None,
            }
        });

        assert_eq!(second_drained, 1);
        assert_eq!(
            second_response.recv().unwrap().outcome,
            ControlOutcome::Rejected
        );
    }

    #[test]
    fn command_queue_rejects_enqueue_after_receiver_drop() {
        let (sender, receiver) = control_command_channel();
        drop(receiver);

        assert!(matches!(
            sender.enqueue_deferred(ControlCommand::Status),
            Err(McpCommandQueueError::QueueClosed)
        ));
    }

    #[test]
    fn command_queue_reports_timeout_when_main_thread_does_not_drain() {
        let (sender, _receiver) = control_command_channel();
        let sender = sender.with_response_timeout(Duration::from_millis(QUEUE_TEST_TIMEOUT_MILLIS));

        assert_eq!(
            sender.enqueue(ControlCommand::Status),
            Err(McpCommandQueueError::ResponseTimedOut)
        );
    }
}
