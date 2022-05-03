mod input_wrapper;
use chrono:: {NaiveDate,Datelike};
use rand::Rng;



fn main() {

    loop {

        let mut rng = rand::thread_rng();
        let num: i32 = rng.gen_range(600000..800000);
        let d = NaiveDate::from_num_days_from_ce(num);
        println!("please calculate the day of {}",d);
        let guess = input_wrapper::get_input();
        if guess == d.weekday().to_string() {
            println!("well done!");
        }
        else {

            println!("no it was {}",d.weekday());
        }



    }
}
