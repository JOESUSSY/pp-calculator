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
    let mut cuck = Dick
    {
        dick_length: 0,
        cum_distance: 1,
        dick_string: String::from("")
    };
    let mut soyboy = Dick::equal_dick(5);
    let mut chad = Dick
    {
        dick_length: 420,
        cum_distance: 911,
        dick_string: String::from("")
    };
    let mut gigachad = Dick {
        dick_length: 911,
        cum_distance: 0,
        dick_string: String::from("")
    };

    cuck.calc_string();
    soyboy.calc_string();
    chad.calc_string();
    gigachad.calc_string();
    println!("cuck: {}, soyboy: {}, chad {}, gigachad {}", cuck.dick_string, soyboy.dick_string, chad.dick_string, gigachad.dick_string);
}