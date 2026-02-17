use datawarehousing_example_nyc_vehicle_incidents::raw::crashes::RawCrashRecord;

fn main() {
    let xs = RawCrashRecord::load_from_csv("data/crashes_preview.csv");
    for x in xs.iter().take(10) {
        println!("{:?}", x);
    }
}
