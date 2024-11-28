//#![allow(unused_variables)]
#![allow(dead_code)]

#[derive(Debug)]
struct SoccerPlayer {
    name: String,
    position: String,
    team: String,
    caps: u16,
    goals: u16,
}

impl SoccerPlayer {
    fn get_caps(self) -> u16 {
        self.caps
    }

    fn add_goal(&mut self) {
        self.goals += 1;
    }
}

fn main() {
    let mut p1 = SoccerPlayer {
        name: String::from("Lionel Messi"),
        position: String::from("Right Winger"),
        team: String::from("Argentina"),
        caps: 166,
        goals: 92,
    };

    p1.add_goal();
    //p1.goals += 1;
    println!("{:?}", p1);
    println!("Goals scored: {}", p1.goals);
    println!("Caps: {}", p1.get_caps());
}