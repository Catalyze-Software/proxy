use std::cell::RefCell;

/*
* Reward formulas
*
* user_count = user_count_base + user_count_icrement * milestone
* points = points_base + user_count * points_slope
* https://docs.google.com/spreadsheets/d/12JF28Eo-MidYGX-gRs-Yx_7DtvtvtQ7dxMf_Lo367QE/edit#gid=1348183526
 */

// Group member count parameters
const GROUP_MEMBER_COUNT_TOTAL_MILESTONES: u64 = 20;
const USER_COUNT_BASE: u64 = 25;
const USER_COUNT_SLOPE: u64 = 25;
const POINTS_BASE: u64 = 0;
const POINTS_SLOPE: u64 = 2;

// Group activity parameters
const GROUP_ACTIVITY_TOTAL_MILESTONES: u64 = 20;
const ACTIVITY_COUNT_BASE: u64 = 5;
const ACTIVITY_COUNT_SLOPE: u64 = 5;
const ACTIVITY_POINTS_BASE: u64 = 0;
const ACTIVITY_POINTS_SLOPE: u64 = 2;

// Event attendee parameters
const EVENT_ATTENDEE_TOTAL_MILESTONES: u64 = 20;
const ATTENDEE_COUNT_BASE: u64 = 10;
const ATTENDEE_COUNT_SLOPE: u64 = 20;
const ATTENDEE_POINTS_BASE: u64 = 0;
const ATTENDEE_POINTS_SLOPE: u64 = 2;

thread_local! {
    static REWARD_AMOUNTS: RefCell<RewardAmounts> = RefCell::new(RewardAmounts::new(
        LinearReward::new(
            GROUP_MEMBER_COUNT_TOTAL_MILESTONES,
            USER_COUNT_BASE,
            USER_COUNT_SLOPE,
            POINTS_BASE,
            POINTS_SLOPE,
        ),
        LinearReward::new(
            GROUP_ACTIVITY_TOTAL_MILESTONES,
            ACTIVITY_COUNT_BASE,
            ACTIVITY_COUNT_SLOPE,
            ACTIVITY_POINTS_BASE,
            ACTIVITY_POINTS_SLOPE,
        ),
        LinearReward::new(
            EVENT_ATTENDEE_TOTAL_MILESTONES,
            ATTENDEE_COUNT_BASE,
            ATTENDEE_COUNT_SLOPE,
            ATTENDEE_POINTS_BASE,
            ATTENDEE_POINTS_SLOPE,
        ),
    ));
}

struct RewardAmounts {
    group_member_count_rewards: LinearReward,
    group_activity_rewards: LinearReward,
    event_attendee_rewards: LinearReward,
}

struct LinearReward {
    total_milestones: u64,
    user_count_base: u64,
    user_count_icrement: u64,
    points_base: u64,
    points_slope: u64,
}

impl RewardAmounts {
    fn new(
        group_member_count_rewards: LinearReward,
        group_activity_rewards: LinearReward,
        event_attendee_rewards: LinearReward,
    ) -> Self {
        Self {
            group_member_count_rewards,
            group_activity_rewards,
            event_attendee_rewards,
        }
    }
}

impl LinearReward {
    fn new(
        total_milestones: u64,
        user_count_base: u64,
        user_count_icrement: u64,
        points_base: u64,
        points_slope: u64,
    ) -> Self {
        Self {
            total_milestones,
            user_count_base,
            user_count_icrement,
            points_base,
            points_slope,
        }
    }

    fn user_count_from_milestone(&self, milestone: u64) -> u64 {
        self.user_count_base + milestone * self.user_count_icrement
    }

    fn points_from_milestone(&self, milestone: u64) -> u64 {
        let user_count = self.user_count_from_milestone(milestone);
        self.points_base + user_count * self.points_slope
    }

    fn milestone_from_user_count(&self, user_count: u64) -> u64 {
        (user_count - self.user_count_base) / self.user_count_icrement
    }

    fn graph(&self) -> Vec<(u64, u64)> {
        let mut graph = Vec::new();
        for i in 0..self.total_milestones {
            let user_count = self.user_count_base + i * self.user_count_icrement;
            let points = self.points_base + user_count * self.points_slope;
            graph.push((user_count, points));
        }
        graph
    }
}

// Points calculation API for use inside proxy
pub fn member_count_points_from_milestone(milestone: u64) -> u64 {
    REWARD_AMOUNTS.with(|reward_amounts| {
        reward_amounts
            .borrow()
            .group_member_count_rewards
            .points_from_milestone(milestone)
    })
}

pub fn member_activity_points_from_milestone(milestone: u64) -> u64 {
    REWARD_AMOUNTS.with(|reward_amounts| {
        reward_amounts
            .borrow()
            .group_activity_rewards
            .points_from_milestone(milestone)
    })
}

pub fn event_attendee_points_from_milestone(milestone: u64) -> u64 {
    REWARD_AMOUNTS.with(|reward_amounts| {
        reward_amounts
            .borrow()
            .event_attendee_rewards
            .points_from_milestone(milestone)
    })
}

pub fn member_count_milestone_from_user_count(user_count: u64) -> u64 {
    REWARD_AMOUNTS.with(|reward_amounts| {
        reward_amounts
            .borrow()
            .group_member_count_rewards
            .milestone_from_user_count(user_count)
    })
}

pub fn member_activity_milestone_from_user_count(user_count: u64) -> u64 {
    REWARD_AMOUNTS.with(|reward_amounts| {
        reward_amounts
            .borrow()
            .group_activity_rewards
            .milestone_from_user_count(user_count)
    })
}

pub fn event_attendee_milestone_from_user_count(user_count: u64) -> u64 {
    REWARD_AMOUNTS.with(|reward_amounts| {
        reward_amounts
            .borrow()
            .event_attendee_rewards
            .milestone_from_user_count(user_count)
    })
}

// Graph API for monitor
pub fn graph_member_count_rewards() -> Vec<(u64, u64)> {
    REWARD_AMOUNTS.with(|reward_amounts| reward_amounts.borrow().group_member_count_rewards.graph())
}

pub fn graph_member_activity_rewards() -> Vec<(u64, u64)> {
    REWARD_AMOUNTS.with(|reward_amounts| reward_amounts.borrow().group_activity_rewards.graph())
}

pub fn graph_event_attendee_rewards() -> Vec<(u64, u64)> {
    REWARD_AMOUNTS.with(|reward_amounts| reward_amounts.borrow().event_attendee_rewards.graph())
}
