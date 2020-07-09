use chrono::prelude::*;
use time::Duration;

use crate::rewards_program::RewardsProgram;

#[derive(Debug)]
struct StreakLogic {
    streak_current: u32,
    streak_bucket: u32,
    last_scan: Option<DateTime<Utc>>,
}

#[derive(Debug, PartialEq)]
struct StreakState {
    streak_missed: u32,
    streak_current: u32,
    streak_bucket: u32,
    bucket: u32,
    mining_ratio: f64,
}

impl StreakLogic {
    // will implement a from member function
    fn new(streak_current: u32, streak_bucket: u32, last_scan: Option<DateTime<Utc>>) -> Self {
        Self {
            streak_bucket,
            streak_current,
            last_scan,
        }
    }

    fn evaluate(&self, at: DateTime<Utc>) -> StreakState {
        // if we do not have a last scan, it is simple
        // no scan was performed and thus streakstate is default
        if self.last_scan.is_none() {
            return StreakState {
                streak_current: 0,
                streak_bucket: 0,
                streak_missed: 0,
                bucket: 0,
                mining_ratio: RewardsProgram::find_mining_ratio(0),
            };
        }

        // lets determine the last_scan streak begin
        let last_scan = self.last_scan.unwrap().date().and_hms(0, 0, 0);
        let window = at.signed_duration_since(last_scan);

        let days_since_last_scan = window.num_days() as u32;

        println!("{:?}", days_since_last_scan);

        match days_since_last_scan {
            0 | 1 => {
                let bucket = RewardsProgram::find_bucket(self.streak_bucket);
                StreakState {
                    streak_current: self.streak_current,
                    streak_bucket: self.streak_bucket,
                    streak_missed: 0,
                    bucket: bucket,
                    mining_ratio: RewardsProgram::find_mining_ratio(bucket),
                }
            }
            _ => {
                let streak_missed = days_since_last_scan - 1;

                // find our current bucket and compensate for days missed
                let mut bucket = RewardsProgram::find_bucket(self.streak_bucket);
                bucket = (bucket - streak_missed.min(bucket)).max(0);

                // now realign our streak_bucket to the beginning of the bucket
                let streak_bucket = RewardsProgram::find_streak_bucket(bucket);
                StreakState {
                    // we missed a day, so our current streak is back to 0
                    streak_current: 0,
                    streak_missed: streak_missed,
                    streak_bucket,
                    bucket,
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
    let state = logic.evaluate(add_hours(8));

    assert_eq!(
        state,
        StreakState {
            streak_current: 0,
            streak_bucket: 0,
            streak_missed: 0,
            bucket: 0,
            mining_ratio: 0.0025,
        }
    );

    // now we have one streak
    let logic = StreakLogic::new(1, 1, Some(last_scan));
    let state = logic.evaluate(add_hours(2));

    assert_eq!(
        state,
        StreakState {
            streak_current: 1,
            streak_bucket: 1,
            streak_missed: 0,
            bucket: 0,
            mining_ratio: 0.0025,
        }
    );

    // now we are moving! 15 days in!
    let logic = StreakLogic::new(15, 15, Some(last_scan));
    let state = logic.evaluate(add_hours(8));

    assert_eq!(
        state,
        StreakState {
            streak_current: 15,
            streak_bucket: 15,
            streak_missed: 0,
            bucket: 3,
            mining_ratio: 0.004,
        }
    );

    // now we are in the new streak zone!
    let state = logic.evaluate(add_hours(24));

    assert_eq!(
        state,
        StreakState {
            streak_current: 15,
            streak_bucket: 15,
            streak_missed: 0,
            bucket: 3,
            mining_ratio: 0.004,
        }
    );

    // now we missed a day
    let state = logic.evaluate(add_hours(48));
    assert_eq!(
        state,
        StreakState {
            streak_current: 0,
            streak_bucket: 5,
            streak_missed: 1,
            bucket: 2,
            mining_ratio: 0.1750 / 50.,
        }
    );

    // now we missed 2 days
    let state = logic.evaluate(add_hours(64));
    assert_eq!(
        state,
        StreakState {
            streak_current: 0,
            streak_bucket: 3,
            streak_missed: 2,
            bucket: 1,
            mining_ratio: 0.003,
        }
    );

    // now we missed a lot
    let state = logic.evaluate(add_hours(640));
    assert_eq!(
        state,
        StreakState {
            streak_current: 0,
            streak_bucket: 0,
            streak_missed: 26,
            bucket: 0,
            mining_ratio: 0.0025,
        }
    );

    // other edge case, we are streaking out!
    let logic = StreakLogic::new(200, 200, Some(last_scan));
    let state = logic.evaluate(add_hours(8));

    assert_eq!(
        state,
        StreakState {
            streak_current: 200,
            streak_bucket: 200,
            streak_missed: 0,
            bucket: 10,
            mining_ratio: 0.0075,
        }
    );
}
