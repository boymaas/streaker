use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use time::Duration;

use crate::rewards_program::RewardsProgram;

#[derive(Debug)]
pub struct StreakLogic {
    streak_current: i32,
    streak_bucket: i32,
    last_scan: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct StreakState {
    pub streak_missed: i32,
    pub streak_current: i32,
    pub streak_bucket: i32,
    pub bucket: i32,
    pub days_since_last_scan: i32,
    pub mining_ratio: f64,
}

impl StreakLogic {
    // will implement a from member function
    pub fn new(streak_current: i32, streak_bucket: i32, last_scan: Option<DateTime<Utc>>) -> Self {
        Self {
            streak_bucket,
            streak_current,
            last_scan,
        }
    }

    pub fn evaluate(&self, at: &DateTime<Utc>) -> StreakState {
        // if we do not have a last scan, it is simple
        // no scan was performed and thus streakstate is default
        if self.last_scan.is_none() {
            return StreakState {
                mining_ratio: RewardsProgram::find_mining_ratio(0),
                ..Default::default()
            };
        }

        // lets determine the last_scan streak begin
        let last_scan = self.last_scan.unwrap().date().and_hms(0, 0, 0);
        let window = at.signed_duration_since(last_scan);

        let days_since_last_scan = window.num_days() as i32;

        match days_since_last_scan {
            0 => {
                let bucket = RewardsProgram::find_bucket(self.streak_bucket);
                StreakState {
                    streak_current: self.streak_current,
                    streak_bucket: self.streak_bucket,
                    streak_missed: 0,
                    bucket: bucket,
                    days_since_last_scan,
                    mining_ratio: RewardsProgram::find_mining_ratio(bucket),
                }
            }
            1 => {
                let bucket = RewardsProgram::find_bucket(self.streak_bucket + 1);
                StreakState {
                    streak_current: self.streak_current + 1,
                    streak_bucket: self.streak_bucket + 1,
                    streak_missed: 0,
                    bucket: bucket,
                    days_since_last_scan,
                    mining_ratio: RewardsProgram::find_mining_ratio(bucket),
                }
            }
            _ => {
                let streak_missed = days_since_last_scan - 1;

                // find our current bucket and compensate for days missed
                let mut bucket = RewardsProgram::find_bucket(self.streak_bucket);
                bucket = (bucket - streak_missed).max(0);

                // now realign our streak_bucket to the beginning of the bucket
                let streak_bucket = RewardsProgram::find_streak_bucket(bucket);
                StreakState {
                    // we missed a day, so our current streak is back to 0
                    streak_current: 0,
                    streak_missed: streak_missed,
                    streak_bucket,
                    bucket,
                    days_since_last_scan,
                    mining_ratio: RewardsProgram::find_mining_ratio(bucket),
                }
            }
        }
    }
}

#[test]
fn test_evaluate() {
    let last_scan = Utc.ymd(2020, 01, 01).and_hms(9, 0, 0);

    let add_hours = |hours: i64| last_scan + Duration::hours(hours);

    // lets start from the beginning
    let logic = StreakLogic::new(0, 0, None);
    let state = logic.evaluate(&add_hours(8));

    assert_eq!(
        state,
        StreakState {
            streak_current: 0,
            streak_bucket: 0,
            streak_missed: 0,
            bucket: 0,
            mining_ratio: 0.0025,
            days_since_last_scan: 0
        }
    );

    // now we have one streak
    let logic = StreakLogic::new(1, 1, Some(last_scan));
    let state = logic.evaluate(&add_hours(2));

    assert_eq!(
        state,
        StreakState {
            streak_current: 1,
            streak_bucket: 1,
            streak_missed: 0,
            bucket: 0,
            mining_ratio: 0.0025,
            days_since_last_scan: 0
        }
    );

    // now we are moving! 15 days in!
    let logic = StreakLogic::new(15, 15, Some(last_scan));
    let state = logic.evaluate(&add_hours(8));

    assert_eq!(
        state,
        StreakState {
            streak_current: 15,
            streak_bucket: 15,
            streak_missed: 0,
            bucket: 3,
            mining_ratio: 0.004,
            days_since_last_scan: 0
        }
    );

    // now we are in the new streak zone!
    let state = logic.evaluate(&add_hours(24));

    assert_eq!(
        state,
        StreakState {
            streak_current: 16,
            streak_bucket: 16,
            streak_missed: 0,
            bucket: 3,
            mining_ratio: 0.004,
            days_since_last_scan: 1
        }
    );

    // now we missed a day
    let state = logic.evaluate(&add_hours(48));
    assert_eq!(
        state,
        StreakState {
            streak_current: 0,
            streak_bucket: 6,
            streak_missed: 1,
            bucket: 2,
            mining_ratio: 0.1750 / 50.,
            days_since_last_scan: 2
        }
    );

    // now we missed 2 days
    let state = logic.evaluate(&add_hours(64));
    assert_eq!(
        state,
        StreakState {
            streak_current: 0,
            streak_bucket: 4,
            streak_missed: 2,
            bucket: 1,
            mining_ratio: 0.003,
            days_since_last_scan: 3
        }
    );

    // now we missed a lot
    let state = logic.evaluate(&add_hours(640));
    assert_eq!(
        state,
        StreakState {
            streak_current: 0,
            streak_bucket: 0,
            streak_missed: 26,
            bucket: 0,
            mining_ratio: 0.0025,
            days_since_last_scan: 27
        }
    );

    // other edge case, we are streaking out!
    let logic = StreakLogic::new(200, 200, Some(last_scan));
    let state = logic.evaluate(&add_hours(8));

    assert_eq!(
        state,
        StreakState {
            streak_current: 200,
            streak_bucket: 200,
            streak_missed: 0,
            bucket: 10,
            mining_ratio: 0.0075,
            days_since_last_scan: 0
        }
    );
}
