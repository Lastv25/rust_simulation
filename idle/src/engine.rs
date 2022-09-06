pub struct MageCity {
    pub nbr_of_plp: f64,
    pub money: f64,
    pub mana: f64,
}

impl MageCity {
    fn origin() -> MageCity {
        println!("Number of People({}", 0.0);
        println!("Money({}", 0.0);
        println!("Mana({}", 0.0);
        MageCity { nbr_of_plp: 0.0, money: 0.0, mana: 0.0 }
    }

    fn add_value_money(&mut self, v: f64) {
            self.money = self.money + v;
            println!("Money({}", self.money);
    }
}