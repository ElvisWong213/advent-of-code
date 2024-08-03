use std::str::FromStr;

use advent_of_code::open_file::open;

fn main() {
    let content = open("./src/day5/input.txt");
    let lines = content.lines();
    let mut map_type: Option<MapType> = None;
    for line in lines {
        let data: Vec<&str> = line.split(':').collect();
        if data.len() == 2 {
            map_type = match MapType::from_str(data[0]) {
                Ok(val) => Some(val),
                Err(_) => None
            };
        }
        println!("{}", data.len());
        for i in data {
            println!("{}", i);
        }
    }
}

fn match_to_map_type(input: &str) -> Option<MapType> {
    match input {
        "seed-to-soil" => Some(MapType::SeedToSoil),
        "soil-to-fertilizer" => Some(MapType::SoilToFertilizer),
        "fertilizer-to-water" => Some(MapType::FertilizerToWater),
        "water-to-light" => Some(MapType::WaterToLight),
        "light-to-temperature" => Some(MapType::LightToTemperature),
        "temperature-to-humidity" => Some(MapType::TemperatureToHumidity),
        "humidity-to-location" => Some(MapType::HumidityToLocation),
        _ => None,
    }
}

enum MapType {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

impl FromStr for MapType {
    type Err = ();

    fn from_str(input: &str) -> Result<MapType, Self::Err> {
        match input {
            "seed-to-soil" => Ok(MapType::SeedToSoil),
            "soil-to-fertilizer" => Ok(MapType::SoilToFertilizer),
            "fertilizer-to-water" => Ok(MapType::FertilizerToWater),
            "water-to-light" => Ok(MapType::WaterToLight),
            "light-to-temperature" => Ok(MapType::LightToTemperature),
            "temperature-to-humidity" => Ok(MapType::TemperatureToHumidity),
            "humidity-to-location" => Ok(MapType::HumidityToLocation),
            _ => Err(()),
        }
    }
}

struct Map {
    destination_start: u32,
    destination_end: u32,
    source_start: u32,
    source_end: u32,
}

impl Map {
    fn new() -> Self {
        Self {
            destination_start: 0,
            destination_end: 0,
            source_start: 0,
            source_end: 0,
        }
    }

    fn load_data(&mut self, destination_start: u32, source_start: u32, range: u32) {
        self.destination_start = destination_start;
        self.source_start = source_start;
        self.destination_end = destination_start + range - 1;
        self.source_end = source_start + range - 1;
    }
}

struct AllMap {
    seed_to_soil: Vec<Map>,
    soil_to_fertilizer: Vec<Map>,
    fertilizer_to_water: Vec<Map>,
    water_to_light: Vec<Map>,
    light_to_temperature: Vec<Map>,
    temperature_to_humidity: Vec<Map>,
    humidity_to_location: Vec<Map>,
}

impl AllMap {
    fn new() -> Self {
        Self {
            seed_to_soil: vec![],
            soil_to_fertilizer: vec![],
            fertilizer_to_water: vec![],
            water_to_light: vec![],
            light_to_temperature: vec![],
            temperature_to_humidity: vec![],
            humidity_to_location: vec![],
        }
    }
}
