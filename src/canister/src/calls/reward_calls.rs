use crate::helpers::guards::is_monitor;
use ic_cdk::query;

#[query(guard = "is_monitor")]
fn graph_member_count_rewards() -> Vec<(u64, u64)> {
    crate::logic::reward_logic::graph_member_count_rewards()
}

#[query(guard = "is_monitor")]
fn graph_member_activity_rewards() -> Vec<(u64, u64)> {
    crate::logic::reward_logic::graph_member_activity_rewards()
}

#[query(guard = "is_monitor")]
fn graph_event_attendee_rewards() -> Vec<(u64, u64)> {
    crate::logic::reward_logic::graph_event_attendee_rewards()
}
