use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AvalancheForecastJson {
    pub id: Option<i32>,
    pub published_time: Option<String>,
    pub expires_time: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub author: Option<String>,
    pub product_type: Option<String>,
    pub bottom_line: Option<String>,
    pub hazard_discussion: Option<String>,
    pub weather_discussion: Option<String>,
    pub announcement: Option<String>,
    pub status: Option<String>,
    pub media: Vec<MediaJson>,
    pub weather_data: Option<AvalancheZoneWeatherJson>,
    pub avalanche_center: Option<AvalancheCenterJson>,
    pub forecast_avalanche_problems: Vec<AvalancheProblemJson>,
    pub danger: Vec<DangerJson>,
    pub forecast_zone: Vec<ForecastZoneJson>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MediaJson {
    id: Option<i32>,
    url: Option<MediaUrlJson>,
    r#type: Option<String>,
    title: Option<String>,
    caption: Option<String>,
    favorite: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MediaUrlJson {
    large: Option<String>,
    medium: Option<String>,
    original: Option<String>,
    thumbnail: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AvalancheZoneWeatherJson {
    weather_product_id: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AvalancheCenterJson {
    id: Option<String>,
    name: Option<String>,
    url: Option<String>,
    city: Option<String>,
    state: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AvalancheProblemJson {
    pub id: Option<i32>,
    pub forecast_id: Option<i32>,
    pub avalanche_problem_id: Option<i32>,
    pub rank: Option<i32>,
    pub likelihood: Option<String>,
    pub discussion: Option<String>,
    pub location: Vec<String>,
    pub size: Vec<String>,
    pub name: Option<String>,
    pub problem_description: Option<String>,
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DangerJson {
    pub lower: Option<i32>,
    pub upper: Option<i32>,
    pub middle: Option<i32>,
    pub valid_day: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ForecastZoneJson {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub url: Option<String>,
    pub state: Option<String>,
    pub zone_id: Option<String>,
    pub config: Option<String>,
}
