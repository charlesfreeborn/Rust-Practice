// Simple Enumeration
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// An enumeration with values
enum TrafficLightWithTime {
    Red(u8),
    Yellow(char),
    Green(String),
}

fn main() {
    let red = TrafficLight::Red;
    let yellow = TrafficLight::Yellow;

    let red_with_time = TrafficLightWithTime::Red(10);
    let yellow_with_time = TrafficLightWithTime::Yellow('3');
    let green_with_time =
        TrafficLightWithTime::Green(String::from("Green light lasts for 30 seconds"));

    // println!("{}", green_with_time);
}
