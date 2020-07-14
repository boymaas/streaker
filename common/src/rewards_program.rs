/// bucket thresholds
static BUCKET: &[i32] = &[0, 3, 5, 10, 20, 30, 40, 50, 60, 70, 80];
/// mining ration per 50 scans
static MINING_RATIO: &[f64] = &[
    0.1250, 0.1500, 0.1750, 0.2000, 0.2250, 0.2500, 0.2750, 0.3000, 0.3250, 0.3500, 0.3750,
];

#[derive(Default)]
pub struct RewardsProgram {}

type Bucket = i32;
type StreakDays = i32;
type MiningRatioPerScan = f64;

impl RewardsProgram {
    /// finds mining ratio per bucket per scan
    pub fn find_mining_ratio(bucket: Bucket) -> MiningRatioPerScan {
        if bucket > MINING_RATIO.len() as i32 {
            return MINING_RATIO[MINING_RATIO.len() - 1] / 50.;
        }
        MINING_RATIO[bucket as usize] / 50.
    }

    pub fn find_bucket(streak: StreakDays) -> Bucket {
        for (bucket, threshold) in BUCKET.iter().enumerate().rev() {
            if *threshold < streak {
                return bucket as i32;
            }
        }
        return 0;
    }
    pub fn find_streak_bucket(bucket: Bucket) -> StreakDays {
        if bucket == 0 {
            return 0;
        }
        BUCKET[bucket.min(BUCKET.len() as i32) as usize] + 1
    }
}

#[test]
fn test_find_bucket() {
    assert_eq!(RewardsProgram::find_bucket(1), 0);
    assert_eq!(RewardsProgram::find_bucket(3), 0);
    assert_eq!(RewardsProgram::find_bucket(4), 1);
    assert_eq!(RewardsProgram::find_bucket(8), 2);
    assert_eq!(RewardsProgram::find_bucket(11), 3);
    assert_eq!(RewardsProgram::find_bucket(75), 9);
    assert_eq!(RewardsProgram::find_bucket(100), 10);
}

#[test]
fn test_find_streak_bucket() {
    assert_eq!(RewardsProgram::find_streak_bucket(0), 0);
    assert_eq!(RewardsProgram::find_streak_bucket(1), 4);
    assert_eq!(RewardsProgram::find_streak_bucket(2), 6);
    assert_eq!(RewardsProgram::find_streak_bucket(3), 11);
    assert_eq!(RewardsProgram::find_streak_bucket(4), 21);
    assert_eq!(RewardsProgram::find_streak_bucket(5), 31);
    assert_eq!(RewardsProgram::find_streak_bucket(6), 41);
    assert_eq!(RewardsProgram::find_streak_bucket(7), 51);
}

#[test]
fn test_find_mining_ratio() {
    assert_eq!(RewardsProgram::find_mining_ratio(0), 0.1250 / 50.0);
    assert_eq!(RewardsProgram::find_mining_ratio(1), 0.1500 / 50.0);
    assert_eq!(RewardsProgram::find_mining_ratio(8), 0.3250 / 50.0);
    assert_eq!(RewardsProgram::find_mining_ratio(10), 0.3750 / 50.0);
    assert_eq!(RewardsProgram::find_mining_ratio(15), 0.3750 / 50.0);
}
