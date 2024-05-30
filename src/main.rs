fn main() {
    let mut water_level = 0; // Initial water level
    let tap_on = true; // Initial state of the tap
    let tap_off = false; // Initial state of the tap
    let mut tap_state = tap_off; // Initial state of the tap (off)

    loop {
        if water_level <= 15 {
            tap_state = tap_on;
            println!("The water level is {}L. The tap is now ON.", water_level);
        } else if water_level >= 100 {
            tap_state = tap_off;
            println!("The water level is {}L. The tap is now OFF.", water_level);
        } else {
            println!("The water level is {}L. The tap remains {}.", water_level, if tap_state { "ON" } else { "OFF" });
        }

        // Simulate water level changing over time
        if tap_state == tap_on {
            water_level += 10; // Increase water level
        } else {
            water_level -= 5; // Decrease water level
        }

        // Limit water level to reasonable range
        if water_level > 110 {
            water_level = 110;
        } else if water_level < 0 {
            water_level = 0;
        }

        // Pause for a short duration to simulate time passing
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
