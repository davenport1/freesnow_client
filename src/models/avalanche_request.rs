use crate::utils::enumerations::{AspectFlags, ElevationFlags, Likelihood, ProblemTypes, Size};
use chrono::NaiveDateTime;
use serde::Serialize;
use log::debug;

use super::avalanche_json::{AvalancheForecastJson, AvalancheProblemJson, DangerJson};

#[derive(Debug, Serialize)]
pub struct AvalanchePostRequest {
    zone_id: u32,
    forecast_date: NaiveDateTime,
    bottom_line: String,
    overall_danger: u32,
    danger_above_treeline: u32,
    danger_at_treeline: u32,
    danger_below_treeling: u32,
    avalanche_problems: Vec<AvalancheProblem>,
}

#[derive(Debug, Serialize)]
pub struct AvalancheProblem {
    additional_notes: String,
    aspects: u32,
    elevations: u32,
    problem_type: u32,
    likelihood: u32,
    size: u32,
}

impl AvalanchePostRequest {
    pub fn from(avalanche_json: AvalancheForecastJson) -> Self {
        let mut problems: Vec<AvalancheProblem> = vec![];

        for problem in avalanche_json.forecast_avalanche_problems.iter() {
            problems.push(AvalancheProblem::from(problem.clone()));
        }

        Self {
            zone_id: avalanche_json
                .forecast_zone
                .get(0)
                .unwrap()
                .clone()
                .id
                .unwrap() as u32,
            forecast_date: NaiveDateTime::parse_from_str(
                avalanche_json.created_at.unwrap().as_str(),
                "%Y-%m-%d %H:%M:%S",
            )
            .unwrap(),
            bottom_line: avalanche_json.bottom_line.unwrap(),
            overall_danger: AvalanchePostRequest::get_overall_danger(avalanche_json.danger.clone()),
            danger_above_treeline: avalanche_json.danger.get(0).unwrap().upper.unwrap() as u32,
            danger_at_treeline: avalanche_json.danger.get(0).unwrap().middle.unwrap() as u32,
            danger_below_treeling: avalanche_json.danger.get(0).unwrap().lower.unwrap() as u32,
            avalanche_problems: problems,
        }
    }

    // only need the danger of the first item in the danger vector
    pub fn get_overall_danger(danger_arr: Vec<DangerJson>) -> u32 {
        let current_danger: DangerJson = danger_arr.get(0).unwrap().clone();
        (current_danger.lower.unwrap()
            + current_danger.middle.unwrap()
            + current_danger.upper.unwrap()) as u32
    }
}

impl AvalancheProblem {
    pub fn from(problem_json: AvalancheProblemJson) -> Self {
        let mut aspects: AspectFlags = AspectFlags::None;
        let mut elevations: ElevationFlags = ElevationFlags::None;
        let problem_type: ProblemTypes;
        let likelihood: Likelihood;
        let size: Size;

        for location_descriptor in problem_json.location.iter() {
            let aspect_elevation: Vec<&str> = location_descriptor.split_whitespace().collect();
            if let Some(aspect) = aspect_elevation.get(0) {
                match *aspect {
                    "north" => aspects |= AspectFlags::North,
                    "northwest" => aspects |= AspectFlags::Northwest,
                    "west" => aspects |= AspectFlags::West,
                    "southwest" => aspects |= AspectFlags::Southwest,
                    "south" => aspects |= AspectFlags::South,
                    "southeast" => aspects |= AspectFlags::Southeast,
                    "east" => aspects |= AspectFlags::East,
                    "northeast" => aspects |= AspectFlags::Northeast,
                    _ => println!("No match"),
                }
            }

            if let Some(elevation) = aspect_elevation.get(1) {
                match *elevation {
                    "upper" => elevations |= ElevationFlags::AboveTreeline,
                    "middle" => elevations |= ElevationFlags::AtTreeline,
                    "lower" => elevations |= ElevationFlags::BelowTreeline,
                    _ => println!("No match"),
                }
            }
        }

        let problem_str: &str = &problem_json
            .name
            .unwrap()
            .to_lowercase()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join("");

        let likelihood_str: &str = &problem_json
            .likelihood
            .unwrap()
            .to_lowercase()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join("");

        // there might be an avalanche_problem_id in the json that indicates
        // the problem type... look into the responses more for different avalanche
        // problems and check if they are meaningful.
        match problem_str {
            "windslab" => problem_type = ProblemTypes::WindSlab,
            "stormslab" => problem_type = ProblemTypes::StormSlab,
            "persistentslab" => problem_type = ProblemTypes::PersistentSlab,
            "loosedry" => problem_type = ProblemTypes::LooseDry,
            "persistentweaklayer" => problem_type = ProblemTypes::PersistentWeakLayer,
            "cornicefall" => problem_type = ProblemTypes::CorniceFall,
            "glide" => problem_type = ProblemTypes::Glide,
            "wetloose" => problem_type = ProblemTypes::WetSnow,
            _ => panic!("No match found for the problem type: {}", problem_str),
        }

        match likelihood_str {
            "unlikely" => likelihood = Likelihood::Unlikely,
            "possible" => likelihood = Likelihood::Possible,
            "likely" => likelihood = Likelihood::Likely,
            "verylikely" => likelihood = Likelihood::VeryLikely,
            "certain" => likelihood = Likelihood::Certain,
            _ => panic!("No match found for likelihood"),
        }

        if problem_json.size[0] == '1'.to_string() && problem_json.size[1] == '2'.to_string() {
            size = Size::SmallLarge;
        } else if problem_json.size[0] == '2'.to_string() && problem_json.size[1] == '3'.to_string()
        {
            size = Size::LargeVeryLarge;
        } else if problem_json.size[0] == '3'.to_string() && problem_json.size[1] == '4'.to_string()
        {
            size = Size::VeryLargeHistoric;
        } else {
            size = Size::SmallLarge; // this neeeds to be fixed
        }

        Self {
            additional_notes: problem_json.discussion.unwrap_or(String::from("")),
            aspects: aspects.bits() as u32,
            elevations: elevations.bits() as u32,
            problem_type: problem_type as u32,
            likelihood: likelihood as u32,
            size: size as u32,
        }
    }
}
