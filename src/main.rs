mod interval;
use self::interval::Interval;

fn main() {
    let interval = Interval::from_millis(500);
    let duration = std::time::Duration::from_millis(2000);

    for i in 1..11 {
        println!("Iteration number {}, counter is {}", i, interval.get_counter());
        std::thread::sleep(duration);
    }
}
