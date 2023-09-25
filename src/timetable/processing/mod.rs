use std::collections::HashMap;

use crate::timetable::*;
use itertools::Itertools;
pub async fn get_processed_timetable(
    token: &str,
    user_id: &str,
    begin: &Date,
    end: &Date,
) -> SduiResult<ProcessedTimeTable> {
    let timetable = get_timetable(token, user_id, begin, end).await?;
    let mut days: Vec<(u64, Lesson)> = vec![];
    for lesson in timetable.0.lessons {
        let day = lesson.begins_at / 86400;
        days.push((day, lesson));
    }
    let mut processed_time_table: ProcessedTimeTable = ProcessedTimeTable { days: vec![] };
    days.sort_by_key(|day| day.0);
    for (_, values) in &days.into_iter().group_by(|day| day.0) {
        let mut day = Day {
            times: HashMap::default(),
        };
        values
            .group_by(|v| v.1.meta.displayname_hour.parse().unwrap())
            .into_iter()
            .sorted_by_key(|v| v.0)
            .for_each(|(key, value)| {
                day.times
                    .insert(key, value.map(|(_, value)| value).collect_vec());
            });
        processed_time_table.days.push(day);
    }
    return Ok((processed_time_table, timetable.1));
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProcessedTimeTable {
    pub days: Vec<Day>,
}
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Day {
    pub times: HashMap<u8, Vec<Lesson>>,
}
