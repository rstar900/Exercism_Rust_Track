use time::PrimitiveDateTime as DateTime;
use time::ext::NumericalDuration;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    // Time 1 Gigasecond after given time
    // time::ext::NumericalDuration needs to be used
    let one_giga_second_after: DateTime = start + 1_000_000_000.seconds();

    one_giga_second_after
}
