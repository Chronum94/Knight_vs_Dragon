extern crate rand;

use rand::Rng;

fn main() {
    // define a Fighter struct with hp, attack, defense, dodge, crit chance, and crit multiplier
    struct Fighter {
        hp: f32,
        atk: f32,
        dfn: i32,
        dodge: f32,
        crit_chance: f32,
        crit_mult: f32
    };

    // implement the attack function for our Fighters
    impl Fighter {
        fn attack(&self, dfn_f2: i32, dodge: f32) -> f32 {
            let mut acc = rand::thread_rng().gen::<f32>();
            let mut dmg: f32 = 0.0;
            // if
            if acc >= dodge {
                dmg = self.atk;
                let mut crit = rand::thread_rng().gen::<f32>();
                if crit <= self.crit_chance {
                    dmg *= self.crit_mult;
                    return dmg;
                } else {
                    return dmg;
                }
            } else {
                return dmg;
            }
        }
        fn ouch(&mut self, dmg: f32) {
            self.hp -= dmg;
        }
    }

    // set up a way for keeping score of wins
    let mut ties: f32 = 0.0;
    let mut dragon_wins: f32 = 0.0;
    let mut knight_wins: f32 = 0.0;
    let mut i: f32 = 0.0;
    while i <= 5100.0 {
        let mut knight: Fighter = Fighter {hp: 300.0, atk: 80.0, dfn: 30, dodge: 0.9, crit_chance: 0.33, crit_mult: 3.0 };
        let mut dragon: Fighter = Fighter {hp: 1500.0, atk: 120.0, dfn: 60, dodge: 0.0, crit_chance: 0.25, crit_mult: 1000.0 };

        while knight.hp > 0.0 && dragon.hp > 0.0 {
            let mut knight_dmg: f32 = knight.attack(dragon.dfn, dragon.dodge);
            let mut dragon_dmg: f32 = dragon.attack(knight.dfn, knight.dodge);
            dragon.ouch(knight_dmg);
            knight.ouch(dragon_dmg);
        };

        if knight.hp <= 0.0 && dragon.hp <= 0.0 {
            println!("It was a tie!");
            ties += 1.0;
        }
        else if knight.hp <= 0.0 {
            println!("The dragon wins!");
            dragon_wins += 1.0;
        }
        else if dragon.hp <= 0.0 {
            println!("The knight wins!");
            knight_wins += 1.0;
        }

        i += 1.0;
    }
    println!("Who won?");
    println!("The knight won {:?}% of the time!", (knight_wins/i)*100.0);
    println!("The dragon won {:?}% of the time!", (dragon_wins/i)*100.0);
    println!("The fighters tied {:?}% of the time!", (ties/i)*100.0);
}
