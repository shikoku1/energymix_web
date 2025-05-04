pub struct DatoStorico {
    pub anno: Vec<i32>,
    pub idrico: Vec<f64>,
    pub geotermoelettrico: Vec<f64>,
    pub fotovoltaico: Vec<f64>,
    pub eolico: Vec<f64>,
    pub bioenergie: Vec<f64>,
    pub termoelettrico: Vec<f64>,
}

use std::collections::{HashMap, HashSet};

fn parse_csv_to_hashmap(csv_content: &str) -> (HashSet<i32>, HashMap<i32, HashMap<&str, f64>>) {
    let mut data: HashMap<i32, HashMap<&str, f64>> = HashMap::new();
    let mut years = HashSet::new();

    // Parse each line of the CSV
    for line in csv_content.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() < 3 {
            continue;
        }

        // Parse year, value, and category
        if let Ok(anno) = parts[0].trim().parse::<i32>() {
            let valore_str = parts[1].trim().replace(',', ".");
            if let Ok(valore) = valore_str.parse::<f64>() {
                let categoria = parts[2].trim();

                // Store the year
                years.insert(anno);

                // Get or create the year's data map
                let year_data = data.entry(anno).or_insert_with(HashMap::new);

                // Store the value for this category
                year_data.insert(categoria, valore);
            }
        }
    }

    (years, data)
}

/// Missing data is filled with zeros
fn parse_csv(csv_content: &str) -> DatoStorico {
    // Parse CSV into HashMap structure
    let (years, data) = parse_csv_to_hashmap(csv_content);

    // Convert years to sorted vector
    let mut year_vec: Vec<i32> = years.into_iter().collect();
    year_vec.sort();

    // Initialize vectors for each category
    let mut idrico = vec![0.0; year_vec.len()];
    let mut geotermoelettrico = vec![0.0; year_vec.len()];
    let mut fotovoltaico = vec![0.0; year_vec.len()];
    let mut eolico = vec![0.0; year_vec.len()];
    let mut bioenergie = vec![0.0; year_vec.len()];
    let mut termoelettrico = vec![0.0; year_vec.len()];

    // Fill vectors with data from the HashMap
    for (i, &year) in year_vec.iter().enumerate() {
        if let Some(year_data) = data.get(&year) {
            idrico[i] = *year_data.get("Idrico").unwrap_or(&0.0);
            geotermoelettrico[i] = *year_data.get("Geotermoelettrico").unwrap_or(&0.0);
            fotovoltaico[i] = *year_data.get("Fotovoltaico").unwrap_or(&0.0);
            eolico[i] = *year_data.get("Eolico").unwrap_or(&0.0);
            bioenergie[i] = *year_data.get("Bioenergie").unwrap_or(&0.0);
            termoelettrico[i] = *year_data.get("Termoelettrico").unwrap_or(&0.0);
        }
    }

    DatoStorico {
        anno: year_vec,
        idrico,
        geotermoelettrico,
        fotovoltaico,
        eolico,
        bioenergie,
        termoelettrico,
    }
}

pub fn get_potenza_installata() -> DatoStorico {
    let csv_content = include_str!("dati/potenza_installata.csv");
    parse_csv(csv_content)
}

pub fn get_produzione() -> DatoStorico {
    let csv_content = include_str!("dati/produzione.csv");
    parse_csv(csv_content)
}
