enum TrafficLight {
    RedLight,
    GreenLight,
    YellowLight,
}
trait TrafficLightDuration{
    fn trafic_light_duration(&self) -> u8 ;
}

impl TrafficLightDuration for TrafficLight {
    fn trafic_light_duration(&self) -> u8{
        match self {
            TrafficLight::RedLight => 1,
            TrafficLight::GreenLight => 2,
            TrafficLight::YellowLight => 3,
        }

    }
}

fn main() {
    let red_light = TrafficLight::RedLight;
    let green_light = TrafficLight::GreenLight;
    let yellow_light = TrafficLight::YellowLight;

    println!("Red light duration is {:?}",red_light.trafic_light_duration());
    println!("Green light duration is {:?}",green_light.trafic_light_duration());
    println!("Yellow light duration is {:?}",yellow_light.trafic_light_duration());
}
