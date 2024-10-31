fn time_conversion(s: &str) -> String {
    let (hour_part, am_pm) = s.split_at(8); 
    let hour = &hour_part[0..2].parse::<i32>().unwrap(); 

    let hour_converted = match am_pm {
        "AM" if *hour == 12 => 0,         
        "PM" if *hour != 12 => hour + 12,   
        _ => *hour,                         
    };

    format!("{:02}{}", hour_converted, &s[2..8]) 
}

fn main() {
    let input = "07:05:45PM";
    let result = time_conversion(input);
    println!("{}", result); 
}
