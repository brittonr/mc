//! Minimal packet compose routing example.
//!
//! Real systems should put complete Valence packet-frame bytes in the bundle or
//! use `PacketBundle::try_push_packet` with an explicit compression threshold.
//! Direct `Client::write_packet` remains the best choice when one system already
//! owns exactly one target client.

use std::collections::BTreeMap;

use valence::prelude::*;

const RED_CLIENT_INDEX: u32 = 31;
const BLUE_CLIENT_INDEX: u32 = 32;
const OBSERVER_CLIENT_INDEX: u32 = 33;
const LOCAL_RADIUS: f64 = 24.0;
const BLUE_DISTANCE: f64 = 12.0;
const OBSERVER_DISTANCE: f64 = 48.0;
const ANNOUNCE_PACKET_FRAME: &[u8] = b"length-prefixed packet frame placeholder";

fn main() {
    let red = Entity::from_raw(RED_CLIENT_INDEX);
    let blue = Entity::from_raw(BLUE_CLIENT_INDEX);
    let observer = Entity::from_raw(OBSERVER_CLIENT_INDEX);
    let spectators = PacketComposeGroup::new("spectators");

    let bundle = PacketBundle::from_packet_bytes([ANNOUNCE_PACKET_FRAME.to_vec()]);
    let clients = [
        PacketComposeClient::active(red, DVec3::ZERO),
        PacketComposeClient::active(blue, DVec3::X * BLUE_DISTANCE),
        PacketComposeClient::active(observer, DVec3::X * OBSERVER_DISTANCE)
            .with_group(spectators.clone()),
    ];
    let intents = [
        PacketRouteIntent::new(PacketBundleId::new(0), PacketRoute::Global).exclude(red),
        PacketRouteIntent::new(
            PacketBundleId::new(0),
            PacketRoute::Local {
                center: DVec3::ZERO,
                radius: LOCAL_RADIUS,
            },
        ),
        PacketRouteIntent::new(PacketBundleId::new(0), PacketRoute::Group(spectators)),
    ];

    let plan = plan_packet_delivery(&intents, &clients, &[bundle.clone()]);
    assert!(plan.is_success());

    let mut frame_count_by_client = BTreeMap::<Entity, usize>::new();
    let report = flush_packet_delivery_plan_with(&plan, &[bundle], |recipient, bundle| {
        frame_count_by_client.insert(recipient, bundle.frame_count());
        Ok(())
    });

    assert!(report.is_success());
    println!(
        "planned {} delivery rows across {} clients",
        report.delivered().len(),
        frame_count_by_client.len()
    );
}
