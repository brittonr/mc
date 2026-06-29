// Copyright 2026
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-APACHE> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{mpsc, Arc};

use super::model::{
    CaptureFrameContext, CapturePolicy, CaptureQueueError, CaptureReadbackError, CaptureRequest,
    CaptureServiceError, CapturedRgbaFrame, MAX_CAPTURE_REQUESTS_PER_FRAME,
    MAX_PENDING_CAPTURE_REQUESTS,
};
use super::service::{service_one_shot_capture_request_with_readback, ServicedCapture};
use super::validation::validate_one_shot_capture_request_shape;

#[derive(Clone)]
pub struct CaptureRequestSender {
    sender: mpsc::Sender<QueuedCaptureRequest>,
    pending_requests: Arc<AtomicUsize>,
    receiver_open: Arc<AtomicBool>,
}

pub struct CaptureRequestReceiver {
    receiver: mpsc::Receiver<QueuedCaptureRequest>,
    pending_requests: Arc<AtomicUsize>,
    receiver_open: Arc<AtomicBool>,
}

struct QueuedCaptureRequest {
    request: CaptureRequest,
    response_sender: mpsc::Sender<Result<ServicedCapture, CaptureServiceError>>,
}

pub fn capture_request_channel() -> (CaptureRequestSender, CaptureRequestReceiver) {
    let (sender, receiver) = mpsc::channel();
    let pending_requests = Arc::new(AtomicUsize::new(0));
    let receiver_open = Arc::new(AtomicBool::new(true));
    (
        CaptureRequestSender {
            sender,
            pending_requests: Arc::clone(&pending_requests),
            receiver_open: Arc::clone(&receiver_open),
        },
        CaptureRequestReceiver {
            receiver,
            pending_requests,
            receiver_open,
        },
    )
}

impl CaptureRequestSender {
    pub fn enqueue_deferred(
        &self,
        request: CaptureRequest,
    ) -> Result<mpsc::Receiver<Result<ServicedCapture, CaptureServiceError>>, CaptureQueueError>
    {
        validate_one_shot_capture_request_shape(&request).map_err(CaptureQueueError::Validation)?;
        ensure_capture_receiver_open(&self.receiver_open)?;
        reserve_pending_capture_slot(&self.pending_requests)?;
        let (response_sender, response_receiver) = mpsc::channel();
        let send_result = self.sender.send(QueuedCaptureRequest {
            request,
            response_sender,
        });
        if send_result.is_err() {
            release_pending_capture_slot(&self.pending_requests);
            return Err(CaptureQueueError::QueueClosed);
        }
        Ok(response_receiver)
    }
}

fn ensure_capture_receiver_open(receiver_open: &AtomicBool) -> Result<(), CaptureQueueError> {
    if receiver_open.load(Ordering::Acquire) {
        return Ok(());
    }
    Err(CaptureQueueError::QueueClosed)
}

fn reserve_pending_capture_slot(pending_requests: &AtomicUsize) -> Result<(), CaptureQueueError> {
    pending_requests
        .fetch_update(Ordering::AcqRel, Ordering::Acquire, |pending| {
            (pending < MAX_PENDING_CAPTURE_REQUESTS).then_some(pending + 1)
        })
        .map(|_| ())
        .map_err(|pending| CaptureQueueError::RateLimitExceeded {
            pending,
            max: MAX_PENDING_CAPTURE_REQUESTS,
        })
}

fn release_pending_capture_slot(pending_requests: &AtomicUsize) {
    let _ = pending_requests.fetch_update(Ordering::AcqRel, Ordering::Acquire, |pending| {
        pending.checked_sub(1)
    });
}

impl Drop for CaptureRequestReceiver {
    fn drop(&mut self) {
        self.receiver_open.store(false, Ordering::Release);
    }
}

impl CaptureRequestReceiver {
    pub fn service_pending_one_shot_with_readback<F>(
        &self,
        policy: &CapturePolicy,
        frame: CaptureFrameContext,
        mut readback: F,
    ) -> usize
    where
        F: FnMut(CaptureFrameContext) -> Result<CapturedRgbaFrame, CaptureReadbackError>,
    {
        let mut serviced = 0;
        while serviced < MAX_CAPTURE_REQUESTS_PER_FRAME {
            let queued = match self.receiver.try_recv() {
                Ok(queued) => {
                    release_pending_capture_slot(&self.pending_requests);
                    queued
                }
                Err(mpsc::TryRecvError::Empty) | Err(mpsc::TryRecvError::Disconnected) => break,
            };
            let response = service_one_shot_capture_request_with_readback(
                queued.request,
                policy,
                frame,
                &mut readback,
            );
            let _ = queued.response_sender.send(response);
            serviced += 1;
        }
        serviced
    }
}
