use std::collections::HashMap;

struct Team {
    name: String,
    played: u8,
    win: u8,
    draw: u8,
    loss: u8,
    points: u8,
}

impl std::fmt::Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let spacing = (0..(31 - self.name.len())).map(|_| " ").collect::<String>();
        write!(
            f,
            "{}{}|  {} |  {} |  {} |  {} |  {}",
            self.name, spacing, self.played, self.win, self.draw, self.loss, self.points
        )
    }
}

enum Result {
    Win,
    Draw,
    Loss,
}

pub fn tally(match_results: &str) -> String {
    let mut teams: HashMap<String, Team> = HashMap::new();

    let lines = match_results.split('\n');
    for line in lines {
        let input: Vec<&str> = line.split(';').collect();
        if input.len() != 3 {
            continue;
        }

        teams = match input[2] {
            "win" => edit_teams(
                edit_teams(teams, input[0], Result::Win),
                input[1],
                Result::Loss,
            ),
            "draw" => edit_teams(
                edit_teams(teams, input[0], Result::Draw),
                input[1],
                Result::Draw,
            ),
            "loss" => edit_teams(
                edit_teams(teams, input[0], Result::Loss),
                input[1],
                Result::Win,
            ),
            _ => teams,
        }
    }

    let mut sorted_teams: Vec<&Team> = teams.values().collect();
    sorted_teams.sort_by(|a, b| {
        if a.points == b.points {
            a.name.cmp(&b.name)
        } else {
            b.points.cmp(&a.points)
        }
    });

    let mut table = String::from("Team                           | MP |  W |  D |  L |  P");

    for team in sorted_teams {
        table.push_str(format!("\n{}", team).as_str());
    }

    println!("{}", table);

    table
}

fn edit_teams(
    mut teams: HashMap<String, Team>,
    team: &str,
    result: Result,
) -> HashMap<String, Team> {
    let entry = teams.entry(team.to_string()).or_insert(Team {
        name: team.to_string(),
        played: 0,
        win: 0,
        draw: 0,
        loss: 0,
        points: 0,
    });

    entry.played += 1;
    match result {
        Result::Win => {
            entry.win += 1;
            entry.points += 3;
        }
        Result::Draw => {
            entry.draw += 1;
            entry.points += 1;
        }
        Result::Loss => {
            entry.loss += 1;
        }
    }

    teams
}
