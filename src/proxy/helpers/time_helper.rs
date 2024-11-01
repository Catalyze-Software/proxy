pub fn hours_to_nanoseconds(hours: u64) -> u64 {
    let minutes_per_hour = 60;
    let seconds_per_minute = 60;
    let nanoseconds_per_second = 1_000_000_000;

    hours * minutes_per_hour * seconds_per_minute * nanoseconds_per_second
}
