struct Calc{
    num_1: i32,
    num_2: i32,
    znak: String,
}

fn main(){
    calc()
}

fn calc(){
    let mut finnal = 0;
    let mut a = Calc {
        num_1: 0,
        num_2: 0,
        znak: "dada".to_string()
    };

    println!("число 1");
    a.num_1 = read_console().parse::<i32>().unwrap();

    println!("число 2");
    a.num_2 = read_console().parse::<i32>().unwrap();

    println!("Знак ( +, -, /, * )");
    a.znak = read_console();

    if a.znak == "-".to_string() {
        finnal = a.num_1 - a.num_2;
        println!("{}", finnal);
    } else if a.znak == "+".to_string() {
        finnal = a.num_1 + a.num_2;
        println!("{}", finnal);
    } else if a.znak == "/".to_string()  {
        finnal = a.num_1 / a.num_2;
        println!("{}", finnal);
    } else if a.znak == "*".to_string()  {
        finnal = a.num_1 * a.num_2;
        println!("{}", finnal);
    } else {
        println!("нет такого знака пошол на");
    }
}

fn read_console() -> String{
    let mut a = "".to_string();

    std::io::stdin().read_line(&mut a).expect("error читать");

    a.trim().to_string()
}