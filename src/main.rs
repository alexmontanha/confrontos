mod load_times;

fn main() {
    let times1 = load_times::load_times();
    let times2 = load_times::load_times();
    //Itera as duas listas e gera confrontos da Lista 1 com a Lista 2
    for time1 in &times1 {
        for time2 in &times2 {
            if time1 != time2 {
                println!("{} x {}", time1, time2);
            }
        }
    }
}
