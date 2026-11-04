//! Decoupled background solver for interactive push-and-shove routing.
//!
//! Prevents UI frame drops on dense boards (10,000+ nets) by offloading
//! A* graph search and elastic collision deflection to a dedicated worker thread.
//! The UI thread sends mouse position updates and polls non-blockingly for the latest solution.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

use crate::LayerId;
use oxide_physics::Microns;

use crate::geometry::Point2D;
use crate::geometry::rtree::NetId;
use crate::interactive::corner_modes::CornerMode;
use crate::interactive::{InteractiveRouter, RouteSegment, RoutingMode};

/// Work item dispatched from the UI thread to the background solver.
#[derive(Debug, Clone)]
pub struct SolverRequest {
    pub request_id: u64,
    pub epoch: u64,
    pub target: Point2D,
    pub mode: RoutingMode,
    pub corner_mode: CornerMode,
    pub net_id: NetId,
    pub layer: LayerId,
    pub track_width: Microns,
}

/// Solved path segments sent back to the UI thread.
#[derive(Debug, Clone)]
pub struct SolverResponse {
    pub request_id: u64,
    pub epoch: u64,
    pub segments: Vec<RouteSegment>,
    pub computation_time_us: u64,
}

/// Background worker managing an asynchronous routing solver.
pub struct AsyncRoutingSolver {
    sender: Option<Sender<SolverRequest>>,
    receiver: Receiver<SolverResponse>,
    request_counter: Arc<AtomicU64>,
    latest_response: Arc<Mutex<Option<SolverResponse>>>,
    shutdown_flag: Arc<AtomicBool>,
    worker_handle: Option<JoinHandle<()>>,
}

impl std::fmt::Debug for AsyncRoutingSolver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsyncRoutingSolver")
            .field(
                "requests_issued",
                &self.request_counter.load(Ordering::Relaxed),
            )
            .finish()
    }
}

impl AsyncRoutingSolver {
    /// Spawns a dedicated solver thread with the given base router.
    pub fn spawn(mut router: InteractiveRouter) -> Self {
        let (req_tx, req_rx) = channel::<SolverRequest>();
        let (resp_tx, resp_rx) = channel::<SolverResponse>();
        let request_counter = Arc::new(AtomicU64::new(0));
        let shutdown = Arc::new(AtomicBool::new(false));

        let shutdown_clone = Arc::clone(&shutdown);

        let worker_handle = thread::Builder::new()
            .name("oxide-routing-solver".to_string())
            .spawn(move || {
                let mut last_processed_id = 0u64;

                while !shutdown_clone.load(Ordering::Relaxed) {
                    // Wait for requests with timeout so shutdown flag is periodically checked
                    let first_req = match req_rx.recv_timeout(std::time::Duration::from_millis(50))
                    {
                        Ok(req) => req,
                        Err(std::sync::mpsc::RecvTimeoutError::Timeout) => continue,
                        Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
                    };

                    // Coalesce / drain any backlog of newer requests to always solve the latest cursor position
                    let mut latest_req = first_req;
                    while let Ok(newer_req) = req_rx.try_recv() {
                        if newer_req.request_id > latest_req.request_id {
                            latest_req = newer_req;
                        }
                    }

                    if latest_req.request_id <= last_processed_id {
                        continue;
                    }
                    last_processed_id = latest_req.request_id;

                    let start_time = std::time::Instant::now();

                    // Apply request configuration
                    router.mode = latest_req.mode;
                    if let Some(s) = &mut router.session {
                        s.corner_mode = latest_req.corner_mode;
                        s.current_net = latest_req.net_id;
                        s.current_layer = latest_req.layer;
                        s.track_width = latest_req.track_width;
                    }

                    // Compute route
                    let segments = router.on_mouse_move(latest_req.target);
                    let elapsed_us = start_time.elapsed().as_micros() as u64;

                    let response = SolverResponse {
                        request_id: latest_req.request_id,
                        epoch: latest_req.epoch,
                        segments,
                        computation_time_us: elapsed_us,
                    };

                    let _ = resp_tx.send(response);
                }
            })
            .expect("failed to spawn async routing solver thread");

        Self {
            sender: Some(req_tx),
            receiver: resp_rx,
            request_counter,
            latest_response: Arc::new(Mutex::new(None)),
            shutdown_flag: shutdown,
            worker_handle: Some(worker_handle),
        }
    }

    /// Submit a new mouse target to the solver with an epoch fence token.
    #[allow(clippy::too_many_arguments)]
    pub fn submit_target(
        &self,
        target: Point2D,
        mode: RoutingMode,
        corner_mode: CornerMode,
        net_id: NetId,
        layer: LayerId,
        track_width: Microns,
        epoch: u64,
    ) -> u64 {
        let req_id = self.request_counter.fetch_add(1, Ordering::SeqCst) + 1;
        let req = SolverRequest {
            request_id: req_id,
            epoch,
            target,
            mode,
            corner_mode,
            net_id,
            layer,
            track_width,
        };
        if let Some(s) = &self.sender {
            let _ = s.send(req);
        }
        req_id
    }

    /// Poll for the latest solved segments matching the current epoch without blocking the UI thread.
    /// Drains any backlog and discards obsolete / out-of-order epoch solutions.
    pub fn poll_latest(&self, current_epoch: u64) -> Option<SolverResponse> {
        let mut latest = None;
        while let Ok(resp) = self.receiver.try_recv() {
            if resp.epoch >= current_epoch {
                latest = Some(resp);
            }
        }
        if let Some(resp) = latest {
            let mut guard = self.latest_response.lock().unwrap();
            *guard = Some(resp.clone());
            Some(resp)
        } else {
            let guard = self.latest_response.lock().unwrap();
            if let Some(resp) = &*guard
                && resp.epoch >= current_epoch
            {
                return Some(resp.clone());
            }
            None
        }
    }
}

impl Drop for AsyncRoutingSolver {
    fn drop(&mut self) {
        self.shutdown_flag.store(true, Ordering::Relaxed);
        // Explicitly drop sender first to unblock any waiting recv
        self.sender.take();
        if let Some(handle) = self.worker_handle.take() {
            let _ = handle.join();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::rtree::SpatialIndex;
    use oxide_rules::ConstraintManager;

    #[test]
    fn test_async_solver_request_draining_and_response() {
        let rules = Arc::new(ConstraintManager::default());
        let spatial_index = Arc::new(SpatialIndex::new());
        let mut router = InteractiveRouter::new(rules, spatial_index);
        let start = Point2D::new(0, 0);
        router.start_routing(start, 1, 0).unwrap();

        let solver = AsyncRoutingSolver::spawn(router);

        // Submit multiple fast targets (simulating rapid cursor drag)
        for i in 1..=5 {
            solver.submit_target(
                Point2D::new(i * 1000, i * 1000),
                RoutingMode::WalkAround,
                CornerMode::default(),
                1,
                0,
                200,
                1,
            );
        }

        // Wait briefly for worker thread to process
        std::thread::sleep(std::time::Duration::from_millis(50));

        let res = solver.poll_latest(1);
        assert!(res.is_some(), "expected solved response from async worker");
        let unwrapped = res.unwrap();
        assert!(!unwrapped.segments.is_empty());
        assert!(unwrapped.request_id >= 1);
        assert_eq!(unwrapped.epoch, 1);
    }

    #[test]
    fn test_async_solver_epoch_fencing_discards_stale_requests() {
        let rules = Arc::new(ConstraintManager::default());
        let spatial_index = Arc::new(SpatialIndex::new());
        let mut router = InteractiveRouter::new(rules, spatial_index);
        let start = Point2D::new(0, 0);
        router.start_routing(start, 1, 0).unwrap();

        let solver = AsyncRoutingSolver::spawn(router);

        // Submit target with old epoch 1
        solver.submit_target(
            Point2D::new(1000, 1000),
            RoutingMode::WalkAround,
            CornerMode::default(),
            1,
            0,
            200,
            1,
        );

        std::thread::sleep(std::time::Duration::from_millis(50));

        // When polling with newer epoch 2, old epoch 1 solutions should be ignored
        let res_stale = solver.poll_latest(2);
        assert!(
            res_stale.is_none(),
            "stale epoch 1 response must be fenced off"
        );

        // Now submit with epoch 2
        solver.submit_target(
            Point2D::new(2000, 2000),
            RoutingMode::WalkAround,
            CornerMode::default(),
            1,
            0,
            200,
            2,
        );

        std::thread::sleep(std::time::Duration::from_millis(50));
        let res_current = solver.poll_latest(2);
        assert!(res_current.is_some(), "epoch 2 response must be received");
        assert_eq!(res_current.unwrap().epoch, 2);
    }
}
