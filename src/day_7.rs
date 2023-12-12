use std::{char, collections::HashMap};

#[derive(Eq, Debug)]
struct Game([u32; 5],u32, Type);

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Type {
    HighCard,
    OnePair,
    TwoPairs,
    TreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

impl Game {
    fn from(hand:[char; 5],bid: u32) -> Self {
        let mut number_of_kinds = (0,0,0,0);

        let numeric_value_hand:Vec<u32> = hand.iter().map(|&c| {
            match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                '1'..='9' => c.to_digit(10).unwrap(),
                _ => panic!()
            }
        }).collect();
        
        let numeric_value_hand: [u32; 5] = [
            numeric_value_hand[0],
            numeric_value_hand[1],
            numeric_value_hand[2],
            numeric_value_hand[3],
            numeric_value_hand[4],
        ];

        let mut cards: HashMap<u32, u32> = HashMap::default();
        for card in numeric_value_hand {
            match cards.get_mut(&card) {
                Some(c ) => *c += 1,
                None => {cards.insert(card, 1);},
            };
        }

        for nr_cards in cards {
            match nr_cards.1 {
                5 => number_of_kinds.0 += 1,
                4 => number_of_kinds.1 += 1,
                3 => number_of_kinds.2 += 1,
                2 => number_of_kinds.3 += 1,
                _ => ()
            }
        }

        let card_type = match number_of_kinds {
            (1,..) => Type::FiveKind,
            (0, 1, ..) => Type::FourKind,
            (0,0,1,1,..) => Type::FullHouse,
            (0,0,1,..) => Type::TreeKind,
            (0,0,0,2,..) => Type::TwoPairs,
            (0,0,0,1,..) => Type::OnePair,
            _ => Type::HighCard  
        };

        Game(numeric_value_hand, bid, card_type)
    }
}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.2.cmp(&other.2) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        };

        let mut self_iter = self.0.iter();
        let mut other_iter = other.0.iter();

        while let (Some(&c_self),Some( &c_other)) = (self_iter.next(), other_iter.next()) {
            match c_self.cmp(&c_other) {
                std::cmp::Ordering::Equal => {}
                ord => return ord
            }
        };

        std::cmp::Ordering::Equal
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}



pub(crate) fn camel_cards() {
    let data = 
        "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483";

    let mut games = parse_games(data);

    games.sort();

    let mut sum = 0;

    for (i, game) in games.iter().enumerate() {
        let i = i as u32;
        sum += (i+1_u32)*game.1;
    }
    println!("Sum {}",sum);
}

fn parse_games(data: &str) -> Vec<Game> {
    let mut data_iter = data.split_whitespace();

    let mut games: Vec<Game> = vec![];

    while let (Some(hand), Some(bid)) = (data_iter.next(), data_iter.next()) {
        let bid   = bid.parse().unwrap();
        let hand:Vec<char> = hand.chars().collect();
        match hand[..] {
            [a,b,c,d,e] => games.push(Game::from([a,b,c,d,e], bid)) ,
            _ => panic!("Invalid input")
        }
    }

    games
}
