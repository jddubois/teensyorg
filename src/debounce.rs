use rtic_monotonics::{systick::Systick, Monotonic};

type SystickInstant = <Systick as Monotonic>::Instant;
type SystickDuration = rtic_monotonics::systick::fugit::Duration<u32, 1, 1000>;

pub fn debounce_input(
    current_value: bool,
    stable_value: &mut bool,
    current_time: SystickInstant,
    previous_change_time: &mut Option<SystickInstant>,
    debounce_duration: SystickDuration,
) -> bool {
    if current_value != *stable_value {
        if previous_change_time.is_none() {
            *previous_change_time = Some(current_time);
        }
        if let Some(t0) = *previous_change_time {
            if current_time
                .checked_duration_since(t0)
                .unwrap()
                .gt(&debounce_duration)
            {
                *stable_value = current_value;
                *previous_change_time = None;
                return true;
            }
        }
    } else {
        *previous_change_time = None;
    }
    false
}
