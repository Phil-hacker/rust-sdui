use std::collections::HashMap;

use crate::timetable::*;
use itertools::Itertools;
use join::join_async;
pub async fn get_processed_timetable(
    token: &str,
    user_id: &str,
    begin: &Date,
    end: &Date,
) -> Result<(ProcessedTimeTable, RateLimit), SduiError> {
    let (timetable, times) =
        join_async!(get_timetable(token, user_id, begin, end), get_times(token)).await;
    let timetable = timetable?;
    let times = times?;
    let mut days: Vec<(u64, u64, Lesson)> = vec![];
    for lesson in timetable.0.lessons {
        let day = lesson.begins_at / 86400;
        let time = lesson.begins_at % 86400;
        days.push((day, time, lesson));
    }
    let time_hashmap: HashMap<u64, Time> = times
        .0
        .into_iter()
        .map(|time| (time.begins_at % 86400, time))
        .collect();
    let mut processed_time_table: ProcessedTimeTable = ProcessedTimeTable { days: vec![] };
    days.sort_by_key(|day| day.0);
    for (_, values) in &days.into_iter().group_by(|day| day.0) {
        let mut day = Day {
            times: HashMap::default(),
        };
        values
            .group_by(|v| v.1)
            .into_iter()
            .for_each(|(key, value)| {
                day.times.insert(
                    time_hashmap.get(&key).unwrap().to_owned(),
                    value.map(|(_, _, value)| value).collect_vec(),
                );
            });
        processed_time_table.days.push(day);
    }
    return Ok((processed_time_table, times.1.join(timetable.1)));
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProcessedTimeTable {
    pub days: Vec<Day>,
}
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Day {
    pub times: HashMap<Time, Vec<Lesson>>,
}
