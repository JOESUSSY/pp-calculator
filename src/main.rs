struct Dick
{
    dick_length: usize,
    cum_distance: usize,
    dick_string: String
}

impl Dick
{
    fn calc_string(&mut self)
    {   
        self.dick_string = format!("8{}D{}", "=".repeat(self.dick_length), "-".repeat(self.cum_distance));
    }
    fn new_dick(length: usize, coom_distance: usize) -> Dick
    {
        Dick
        {
            dick_length: length,
            cum_distance: coom_distance,
            dick_string: String::from("")
        }
    }
    fn equal_dick(bruh: usize) -> Dick
    {
        Dick 
        {
            dick_length: bruh,
            cum_distance: bruh,
            dick_string: String::from("")
        }
    }
}
fn main()
{
    let mut cuck = Dick::new_dick(0, 1);
    let mut soyboy = Dick::equal_dick(5);
    let mut chad = Dick::new_dick(420, 911);
    let mut gigachad = Dick::new_dick(911, 0);

    cuck.calc_string();
    soyboy.calc_string();
    chad.calc_string();
    gigachad.calc_string();
    println!("cuck: {}, soyboy: {}, chad {}, gigachad {}", cuck.dick_string, soyboy.dick_string, chad.dick_string, gigachad.dick_string);
}