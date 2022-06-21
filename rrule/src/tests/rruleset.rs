#![allow(deprecated)]

use crate::tests::common::{check_occurrences, test_recurring_rrule_set, ymd_hms};
use crate::{Frequency, NWeekday, RRule, RRuleSet, Weekday};

#[test]
fn rrule_and_exrule() {
    let dt_start = ymd_hms(1997, 9, 2, 9, 0, 0);

    let rrule1 = RRule {
        freq: Frequency::Yearly,
        count: Some(6),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    let rrule = rrule1.validate(dt_start).unwrap();

    let rrule2 = RRule {
        freq: Frequency::Yearly,
        count: Some(3),
        by_weekday: vec![NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    let exrule = rrule2.validate(dt_start).unwrap();

    let set = RRuleSet::new(dt_start).rrule(rrule).exrule(exrule);

    test_recurring_rrule_set(
        set,
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 9, 9, 0, 0),
            ymd_hms(1997, 9, 16, 9, 0, 0),
        ],
    );
}

#[test]
fn setdate_and_exdate() {
    let set = RRuleSet::new(ymd_hms(1970, 1, 1, 0, 0, 0))
        .set_rdates(vec![
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 4, 9, 0, 0),
            ymd_hms(1997, 9, 9, 9, 0, 0),
            ymd_hms(1997, 9, 11, 9, 0, 0),
            ymd_hms(1997, 9, 16, 9, 0, 0),
            ymd_hms(1997, 9, 18, 9, 0, 0),
        ])
        .set_exdates(vec![
            ymd_hms(1997, 9, 4, 9, 0, 0),
            ymd_hms(1997, 9, 11, 9, 0, 0),
            ymd_hms(1997, 9, 18, 9, 0, 0),
        ]);

    test_recurring_rrule_set(
        set,
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 9, 9, 0, 0),
            ymd_hms(1997, 9, 16, 9, 0, 0),
        ],
    );
}

#[test]
fn setdate_and_exrule() {
    let dt_start = ymd_hms(1997, 9, 2, 9, 0, 0);
    let rrule = RRule {
        freq: Frequency::Yearly,
        count: Some(3),
        by_weekday: vec![NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    let exrule = rrule.validate(dt_start).unwrap();

    let set = RRuleSet::new(dt_start)
        .set_rdates(vec![
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 4, 9, 0, 0),
            ymd_hms(1997, 9, 9, 9, 0, 0),
            ymd_hms(1997, 9, 11, 9, 0, 0),
            ymd_hms(1997, 9, 16, 9, 0, 0),
            ymd_hms(1997, 9, 18, 9, 0, 0),
        ])
        .exrule(exrule);

    test_recurring_rrule_set(
        set,
        &[
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 9, 9, 0, 0),
            ymd_hms(1997, 9, 16, 9, 0, 0),
        ],
    );
}

#[test]
fn rrule_and_exdate() {
    let dt_start = ymd_hms(1997, 9, 2, 9, 0, 0);
    let rrule = RRule {
        freq: Frequency::Yearly,
        count: Some(6),
        by_weekday: vec![NWeekday::Every(Weekday::Tue), NWeekday::Every(Weekday::Thu)],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    let rrule = rrule.validate(dt_start).unwrap();

    let set = RRuleSet::new(dt_start).rrule(rrule).set_exdates(vec![
        ymd_hms(1997, 9, 2, 9, 0, 0),
        ymd_hms(1997, 9, 4, 9, 0, 0),
        ymd_hms(1997, 9, 9, 9, 0, 0),
    ]);

    test_recurring_rrule_set(
        set,
        &[
            ymd_hms(1997, 9, 11, 9, 0, 0),
            ymd_hms(1997, 9, 16, 9, 0, 0),
            ymd_hms(1997, 9, 18, 9, 0, 0),
        ],
    );
}

#[test]
fn rrule_and_exyearly_yearly_big() {
    let dt_start = ymd_hms(1997, 9, 2, 9, 0, 0);

    let rrule = RRule {
        freq: Frequency::Yearly,
        count: Some(13),
        by_month: vec![9],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![2],
        ..Default::default()
    };
    let rrule = rrule.validate(dt_start).unwrap();

    let exrule = RRule {
        freq: Frequency::Yearly,
        count: Some(10),
        by_month: vec![9],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![2],
        ..Default::default()
    };
    let exrule = exrule.validate(dt_start).unwrap();

    let set = RRuleSet::new(dt_start).rrule(rrule).exrule(exrule);

    test_recurring_rrule_set(
        set,
        &[
            ymd_hms(2007, 9, 2, 9, 0, 0),
            ymd_hms(2008, 9, 2, 9, 0, 0),
            ymd_hms(2009, 9, 2, 9, 0, 0),
        ],
    );
}

#[test]
fn before() {
    let dt_start = ymd_hms(1997, 9, 2, 9, 0, 0);

    let rrule = RRule {
        freq: Frequency::Yearly,
        by_month: vec![9],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![2],
        ..Default::default()
    };
    let rrule = rrule.validate(dt_start).unwrap();

    let exrule = RRule {
        freq: Frequency::Yearly,
        count: Some(10),
        by_month: vec![9],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![2],
        ..Default::default()
    };
    let exrule = exrule.validate(dt_start).unwrap();

    let set = RRuleSet::new(dt_start).rrule(rrule).exrule(exrule);

    assert_eq!(
        set.just_before(ymd_hms(2015, 9, 2, 9, 0, 0), false)
            .unwrap()
            .unwrap(),
        ymd_hms(2014, 9, 2, 9, 0, 0),
    );
}

#[test]
fn after() {
    let dt_start = ymd_hms(1997, 9, 2, 9, 0, 0);

    let rrule = RRule {
        freq: Frequency::Yearly,
        by_month: vec![9],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![2],
        ..Default::default()
    };
    let rrule = rrule.validate(dt_start).unwrap();

    let exrule = RRule {
        freq: Frequency::Yearly,
        count: Some(10),
        by_month: vec![9],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![2],
        ..Default::default()
    };
    let exrule = exrule.validate(dt_start).unwrap();

    let set = RRuleSet::new(dt_start).rrule(rrule).exrule(exrule);

    assert_eq!(
        set.just_after(ymd_hms(2000, 9, 2, 9, 0, 0), false)
            .unwrap()
            .unwrap(),
        ymd_hms(2007, 9, 2, 9, 0, 0),
    );
}

#[test]
fn between() {
    let dt_start = ymd_hms(1997, 9, 2, 9, 0, 0);

    let rrule = RRule {
        freq: Frequency::Yearly,
        by_month: vec![9],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![2],
        ..Default::default()
    };
    let rrule = rrule.validate(dt_start).unwrap();

    let exrule = RRule {
        freq: Frequency::Yearly,
        count: Some(10),
        by_month: vec![9],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![2],
        ..Default::default()
    };
    let exrule = exrule.validate(dt_start).unwrap();

    let set = RRuleSet::new(dt_start).rrule(rrule).exrule(exrule);

    check_occurrences(
        &set.all_between(
            ymd_hms(2000, 9, 2, 9, 0, 0),
            ymd_hms(2010, 9, 2, 9, 0, 0),
            false,
        )
        .unwrap(),
        &[
            "2007-09-02T09:00:00-00:00",
            "2008-09-02T09:00:00-00:00",
            "2009-09-02T09:00:00-00:00",
        ],
    );
}

#[test]
fn before_70s() {
    let dt_start = ymd_hms(1960, 1, 1, 9, 0, 0);

    let rrule = RRule {
        freq: Frequency::Yearly,
        count: Some(2),
        by_month: vec![1],
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![1],
        ..Default::default()
    };
    let rrule = rrule.validate(dt_start).unwrap();

    let set = RRuleSet::new(dt_start).rrule(rrule);

    test_recurring_rrule_set(
        set,
        &[ymd_hms(1960, 1, 1, 9, 0, 0), ymd_hms(1961, 1, 1, 9, 0, 0)],
    );
}

#[test]
fn secondly_with_interval_1() {
    let dt_start = ymd_hms(1960, 1, 1, 9, 0, 0);

    let rrule = RRule {
        freq: Frequency::Secondly,
        count: Some(2),
        ..Default::default()
    };
    let rrule = rrule.validate(dt_start).unwrap();

    let set = RRuleSet::new(dt_start).rrule(rrule);

    test_recurring_rrule_set(
        set,
        &[ymd_hms(1960, 1, 1, 9, 0, 0), ymd_hms(1960, 1, 1, 9, 0, 1)],
    );
}

#[test]
fn secondly_with_interval_2() {
    let dt_start = ymd_hms(1960, 1, 1, 9, 0, 0);

    let rrule = RRule {
        freq: Frequency::Secondly,
        count: Some(2),
        interval: 2,
        ..Default::default()
    };
    let rrule = rrule.validate(dt_start).unwrap();

    let set = RRuleSet::new(dt_start).rrule(rrule);

    test_recurring_rrule_set(
        set,
        &[ymd_hms(1960, 1, 1, 9, 0, 0), ymd_hms(1960, 1, 1, 9, 0, 2)],
    );
}

#[test]
fn minutely_with_interval_1() {
    let dt_start = ymd_hms(1960, 1, 1, 9, 0, 0);

    let rrule = RRule {
        freq: Frequency::Minutely,
        count: Some(2),
        by_second: vec![0],
        ..Default::default()
    };
    let rrule = rrule.validate(dt_start).unwrap();

    let set = RRuleSet::new(dt_start).rrule(rrule);

    test_recurring_rrule_set(
        set,
        &[ymd_hms(1960, 1, 1, 9, 0, 0), ymd_hms(1960, 1, 1, 9, 1, 0)],
    );
}

#[test]
fn minutely_with_interval_2() {
    let dt_start = ymd_hms(1960, 1, 1, 9, 0, 0);

    let rrule = RRule {
        freq: Frequency::Minutely,
        count: Some(2),
        by_second: vec![0],
        interval: 2,
        ..Default::default()
    };
    let rrule = rrule.validate(dt_start).unwrap();

    let set = RRuleSet::new(dt_start).rrule(rrule);

    test_recurring_rrule_set(
        set,
        &[ymd_hms(1960, 1, 1, 9, 0, 0), ymd_hms(1960, 1, 1, 9, 2, 0)],
    );
}

#[test]
fn hourly_with_interval_1() {
    let dt_start = ymd_hms(1960, 1, 1, 9, 0, 0);

    let rrule = RRule {
        freq: Frequency::Hourly,
        count: Some(2),
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    let rrule = rrule.validate(dt_start).unwrap();

    let set = RRuleSet::new(dt_start).rrule(rrule);

    test_recurring_rrule_set(
        set,
        &[ymd_hms(1960, 1, 1, 9, 0, 0), ymd_hms(1960, 1, 1, 10, 0, 0)],
    );
}

#[test]
fn hourly_with_interval_2() {
    let dt_start = ymd_hms(1960, 1, 1, 9, 0, 0);

    let rrule = RRule {
        freq: Frequency::Hourly,
        count: Some(2),
        by_minute: vec![0],
        by_second: vec![0],
        interval: 2,
        ..Default::default()
    };
    let rrule = rrule.validate(dt_start).unwrap();

    let set = RRuleSet::new(dt_start).rrule(rrule);

    test_recurring_rrule_set(
        set,
        &[ymd_hms(1960, 1, 1, 9, 0, 0), ymd_hms(1960, 1, 1, 11, 0, 0)],
    );
}

#[test]
fn daily_with_interval_1() {
    let dt_start = ymd_hms(1960, 1, 1, 9, 0, 0);

    let rrule = RRule {
        freq: Frequency::Daily,
        count: Some(2),
        by_minute: vec![0],
        by_second: vec![0],
        ..Default::default()
    };
    let rrule = rrule.validate(dt_start).unwrap();

    let set = RRuleSet::new(dt_start).rrule(rrule);

    test_recurring_rrule_set(
        set,
        &[ymd_hms(1960, 1, 1, 9, 0, 0), ymd_hms(1960, 1, 2, 9, 0, 0)],
    );
}

#[test]
fn daily_with_interval_2() {
    let dt_start = ymd_hms(1960, 1, 1, 9, 0, 0);

    let rrule = RRule {
        freq: Frequency::Daily,
        count: Some(2),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        interval: 2,
        ..Default::default()
    };
    let rrule = rrule.validate(dt_start).unwrap();

    let set = RRuleSet::new(dt_start).rrule(rrule);

    test_recurring_rrule_set(
        set,
        &[ymd_hms(1960, 1, 1, 9, 0, 0), ymd_hms(1960, 1, 3, 9, 0, 0)],
    );
}

#[test]
fn weekly_with_interval_1() {
    let dt_start = ymd_hms(1960, 1, 4, 9, 0, 0);

    let rrule = RRule {
        freq: Frequency::Weekly,
        count: Some(2),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_weekday: vec![NWeekday::Every(Weekday::Mon)],
        ..Default::default()
    };
    // 4th is Monday
    let rrule = rrule.validate(dt_start).unwrap();

    let set = RRuleSet::new(dt_start).rrule(rrule);

    test_recurring_rrule_set(
        set,
        &[ymd_hms(1960, 1, 4, 9, 0, 0), ymd_hms(1960, 1, 11, 9, 0, 0)],
    );
}

#[test]
fn weekly_with_interval_2() {
    let dt_start = ymd_hms(1960, 1, 4, 9, 0, 0);

    let rrule = RRule {
        freq: Frequency::Weekly,
        count: Some(2),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_weekday: vec![NWeekday::Every(Weekday::Mon)],
        interval: 2,
        ..Default::default()
    };
    // 4th is Monday
    let rrule = rrule.validate(dt_start).unwrap();

    let set = RRuleSet::new(dt_start).rrule(rrule);

    test_recurring_rrule_set(
        set,
        &[ymd_hms(1960, 1, 4, 9, 0, 0), ymd_hms(1960, 1, 18, 9, 0, 0)],
    );
}

#[test]
fn monthly_with_interval_1() {
    let dt_start = ymd_hms(1960, 1, 1, 9, 0, 0);

    let rrule = RRule {
        freq: Frequency::Monthly,
        count: Some(2),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_month_day: vec![1],
        ..Default::default()
    };
    let rrule = rrule.validate(dt_start).unwrap();

    let set = RRuleSet::new(dt_start).rrule(rrule);

    test_recurring_rrule_set(
        set,
        &[ymd_hms(1960, 1, 1, 9, 0, 0), ymd_hms(1960, 2, 1, 9, 0, 0)],
    );
}

#[test]
fn monthly_with_interval_2() {
    let dt_start = ymd_hms(1960, 1, 1, 9, 0, 0);

    let rrule = RRule {
        freq: Frequency::Monthly,
        count: Some(2),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        interval: 2,
        by_month_day: vec![1],
        ..Default::default()
    };
    let rrule = rrule.validate(dt_start).unwrap();

    let set = RRuleSet::new(dt_start).rrule(rrule);

    test_recurring_rrule_set(
        set,
        &[ymd_hms(1960, 1, 1, 9, 0, 0), ymd_hms(1960, 3, 1, 9, 0, 0)],
    );
}

#[test]
fn yearly_with_interval_1() {
    let dt_start = ymd_hms(1960, 1, 1, 9, 0, 0);

    let rrule = RRule {
        freq: Frequency::Yearly,
        count: Some(2),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_year_day: vec![1],
        ..Default::default()
    };
    let rrule = rrule.validate(dt_start).unwrap();

    let set = RRuleSet::new(dt_start).rrule(rrule);

    test_recurring_rrule_set(
        set,
        &[ymd_hms(1960, 1, 1, 9, 0, 0), ymd_hms(1961, 1, 1, 9, 0, 0)],
    );
}

#[test]
fn yearly_with_interval_2() {
    let dt_start = ymd_hms(1960, 1, 1, 9, 0, 0);

    let rrule = RRule {
        freq: Frequency::Yearly,
        count: Some(2),
        by_hour: vec![9],
        by_minute: vec![0],
        by_second: vec![0],
        by_year_day: vec![1],
        interval: 2,
        ..Default::default()
    };
    let rrule = rrule.validate(dt_start).unwrap();

    let set = RRuleSet::new(dt_start).rrule(rrule);

    test_recurring_rrule_set(
        set,
        &[ymd_hms(1960, 1, 1, 9, 0, 0), ymd_hms(1962, 1, 1, 9, 0, 0)],
    );
}