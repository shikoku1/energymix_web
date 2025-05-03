use js_sys::Array;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use sim::{EnergyGeneratorScenario, SingleSourceSimResult};
use storico::DatoStorico;
use wasm_bindgen::prelude::*;

mod sim;
mod storico;

#[wasm_bindgen(start)]
pub fn start() {
    wasm_logger::init(wasm_logger::Config::default());
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone)]
pub struct SimulationSettings {
    pub fotovoltaico_inizio: f64,
    pub fotovoltaico_fine: f64,
    pub eolico_inizio: f64,
    pub eolico_fine: f64,
    pub nucleare_inizio: f64,
    pub nucleare_fine: f64,
    pub durata_cantiere_foak: i32,
    pub durata_cantiere_noak: i32,
    pub tasso_crescita_consumo: f64,
    pub produzione_altre_fonti_lowc: f64,
    pub anno_inizio_installazioni_nucleare: i32,
    pub anno_fine_installazioni_nucleare: i32,
    pub end_year: i32,
}

#[wasm_bindgen]
pub struct SimulationResults {
    scenario_fv: SingleSourceSimResult,
    scenario_eol: SingleSourceSimResult,
    scenario_nucl: SingleSourceSimResult,
    other_lowc_production: Vec<f64>,
    consumo: Vec<f64>,
    historical_capacity: DatoStorico,
    historical_production: DatoStorico,
}

#[wasm_bindgen]
impl SimulationResults {
    #[wasm_bindgen(getter)]
    pub fn years(&self) -> Array {
        vec_to_js_array(&self.scenario_fv.anni)
    }

    #[wasm_bindgen(getter)]
    pub fn pv_production(&self) -> Array {
        vec_to_js_array(&self.scenario_fv.energia_prodotta)
    }

    #[wasm_bindgen(getter)]
    pub fn pv_capacity(&self) -> Array {
        vec_to_js_array(
            &self
                .scenario_fv
                .potenza_installata
                .iter()
                .map(|x| x * 1000.0)
                .collect::<Vec<f64>>(),
        )
    }

    #[wasm_bindgen(getter)]
    pub fn wind_production(&self) -> Array {
        vec_to_js_array(&self.scenario_eol.energia_prodotta)
    }

    #[wasm_bindgen(getter)]
    pub fn wind_capacity(&self) -> Array {
        vec_to_js_array(
            &self
                .scenario_eol
                .potenza_installata
                .iter()
                .map(|x| x * 1000.0)
                .collect::<Vec<f64>>(),
        )
    }

    #[wasm_bindgen(getter)]
    pub fn nuclear_production(&self) -> Array {
        vec_to_js_array(&self.scenario_nucl.energia_prodotta)
    }

    #[wasm_bindgen(getter)]
    pub fn nuclear_capacity(&self) -> Array {
        vec_to_js_array(
            &self
                .scenario_nucl
                .potenza_installata
                .iter()
                .map(|x| x * 1000.0)
                .collect::<Vec<f64>>(),
        )
    }

    #[wasm_bindgen(getter)]
    pub fn other_lowc_production(&self) -> Array {
        vec_to_js_array(&self.other_lowc_production)
    }

    #[wasm_bindgen(getter)]
    pub fn consumo(&self) -> Array {
        vec_to_js_array(&self.consumo)
    }

    #[wasm_bindgen(getter)]
    pub fn historical_years(&self) -> Array {
        vec_to_js_array(&self.historical_capacity.anno)
    }

    #[wasm_bindgen(getter)]
    pub fn historical_pv_production(&self) -> Array {
        vec_to_js_array(&self.historical_production.fotovoltaico)
    }

    #[wasm_bindgen(getter)]
    pub fn historical_wind_production(&self) -> Array {
        vec_to_js_array(&self.historical_production.eolico)
    }

    #[wasm_bindgen(getter)]
    pub fn historical_hydro_production(&self) -> Array {
        vec_to_js_array(&self.historical_production.idrico)
    }

    #[wasm_bindgen(getter)]
    pub fn historical_geothermal_production(&self) -> Array {
        vec_to_js_array(&self.historical_production.geotermoelettrico)
    }

    #[wasm_bindgen(getter)]
    pub fn historical_pv_capacity(&self) -> Array {
        vec_to_js_array(&self.historical_capacity.fotovoltaico)
    }

    #[wasm_bindgen(getter)]
    pub fn historical_wind_capacity(&self) -> Array {
        vec_to_js_array(&self.historical_capacity.eolico)
    }

    #[wasm_bindgen(getter)]
    pub fn historical_hydro_capacity(&self) -> Array {
        vec_to_js_array(&self.historical_capacity.idrico)
    }

    #[wasm_bindgen(getter)]
    pub fn historical_geothermal_capacity(&self) -> Array {
        vec_to_js_array(&self.historical_capacity.geotermoelettrico)
    }

    #[wasm_bindgen(getter)]
    pub fn historical_bioenergy_capacity(&self) -> Array {
        vec_to_js_array(&self.historical_capacity.bioenergie)
    }
}

fn vec_to_js_array<T: Clone + Into<JsValue>>(vec: &[T]) -> Array {
    Array::from_iter(vec.iter().map(|x| x.clone().into()))
}

#[wasm_bindgen]
pub fn run_simulation(settings: JsValue) -> SimulationResults {
    let settings: SimulationSettings = serde_wasm_bindgen::from_value(settings).unwrap();

    let start_year = 2023;
    let end_year = settings.end_year;

    // Calcola il consumo per ogni anno
    let mut consumo = Vec::new();
    let mut consumo_corrente = 319.0 * 1000.0;
    for _ in start_year..=end_year {
        consumo.push(consumo_corrente);
        consumo_corrente *= 1.0 + settings.tasso_crescita_consumo / 100.0;
    }

    let mut produzione_altre_fonti = Vec::new();
    for _ in start_year..=end_year {
        produzione_altre_fonti.push(settings.produzione_altre_fonti_lowc * 1000.0);
    }

    let fottovoltaico = EnergyGeneratorScenario {
        scenario_start_year: start_year,
        scenario_stop_year: end_year,
        anno_inizio_installazioni: start_year,
        anno_fine_installazioni: end_year,
        potenza_iniziale: *storico::get_potenza_installata()
            .fotovoltaico
            .last()
            .unwrap() / 1000.0,
        nuova_potenza_annuale_foak: settings.fotovoltaico_inizio,
        nuova_potenza_annuale_noak: settings.fotovoltaico_fine,
        durata_cantieri_foak: 1,
        capacity_factor: 0.12,
        life_years: 25,
        costo_foak: 0.0,
        costo_noak: 0.0,
        emissioni_co2: 40.0,
        durata_cantieri_noak: 1,
    };

    let eolico = EnergyGeneratorScenario {
        scenario_start_year: start_year,
        scenario_stop_year: end_year,
        anno_inizio_installazioni: start_year,
        anno_fine_installazioni: end_year,
        potenza_iniziale: *storico::get_potenza_installata()
            .eolico
            .last()
            .unwrap() / 1000.0,
        nuova_potenza_annuale_foak: settings.eolico_inizio,
        nuova_potenza_annuale_noak: settings.eolico_fine,
        durata_cantieri_foak: 1,
        capacity_factor: 0.2,
        life_years: 20,
        costo_foak: 0.0,
        costo_noak: 0.0,
        emissioni_co2: 12.0,
        durata_cantieri_noak: 1,
    };

    let nucleare = EnergyGeneratorScenario {
        scenario_start_year: start_year,
        scenario_stop_year: end_year,
        anno_inizio_installazioni: settings.anno_inizio_installazioni_nucleare,
        anno_fine_installazioni: settings.anno_fine_installazioni_nucleare,
        potenza_iniziale: 0.0,
        nuova_potenza_annuale_foak: settings.nucleare_inizio,
        nuova_potenza_annuale_noak: settings.nucleare_fine,
        durata_cantieri_foak: settings.durata_cantiere_foak,
        durata_cantieri_noak: settings.durata_cantiere_noak,
        capacity_factor: 0.85,
        life_years: 60,
        costo_foak: 0.0,
        costo_noak: 0.0,
        emissioni_co2: 5.0,
    };

    SimulationResults {
        scenario_fv: fottovoltaico.generate_time_series(),
        scenario_eol: eolico.generate_time_series(),
        scenario_nucl: nucleare.generate_time_series(),
        other_lowc_production: produzione_altre_fonti,
        consumo,
        historical_capacity: storico::get_potenza_installata(),
        historical_production: storico::get_produzione(),
    }
}

#[derive(Serialize, Deserialize)]
pub struct SliderConfig {
    pub name_human: String,
    pub name_machine: String,
    pub unit: String,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub default_value: f64,
}

#[wasm_bindgen]
pub fn get_sliders_json() -> String {
    let sliders = vec![
        SliderConfig {
            name_human: "☀️ Fotovoltaico installato all'anno, inizio".to_string(),
            name_machine: "fotovoltaico_inizio".to_string(),
            unit: "GW".to_string(),
            min: 0.0,
            max: 10.0,
            step: 0.1,
            default_value: 1.0,
        },
        SliderConfig {
            name_human: "☀️ Fotovoltaico installato all'anno, fine".to_string(),
            name_machine: "fotovoltaico_fine".to_string(),
            unit: "GW".to_string(),
            min: 0.0,
            max: 10.0,
            step: 0.1,
            default_value: 2.0,
        },
        SliderConfig {
            name_human: "💨 Eolico installato all'anno, inizio".to_string(),
            name_machine: "eolico_inizio".to_string(),
            unit: "GW".to_string(),
            min: 0.0,
            max: 10.0,
            step: 0.1,
            default_value: 0.5,
        },
        SliderConfig {
            name_human: "💨 Eolico installato all'anno, fine".to_string(),
            name_machine: "eolico_fine".to_string(),
            unit: "GW".to_string(),
            min: 0.0,
            max: 10.0,
            step: 0.1,
            default_value: 1.0,
        },
        SliderConfig {
            name_human: "☢️ Nucleare installato all'anno, inizio".to_string(),
            name_machine: "nucleare_inizio".to_string(),
            unit: "GW".to_string(),
            min: 0.0,
            max: 10.0,
            step: 0.1,
            default_value: 2.0,
        },
        SliderConfig {
            name_human: "☢️ Nucleare installato all'anno, fine".to_string(),
            name_machine: "nucleare_fine".to_string(),
            unit: "GW".to_string(),
            min: 0.0,
            max: 10.0,
            step: 0.1,
            default_value: 3.0,
        },
        SliderConfig {
            name_human: "☢️🕐 Durata Cantiere Nucleare (foak)".to_string(),
            name_machine: "durata_cantiere_foak".to_string(),
            unit: "anni".to_string(),
            min: 0.0,
            max: 20.0,
            step: 1.0,
            default_value: 8.0,
        },
        SliderConfig {
            name_human: "☢️🕐 Durata Cantiere Nucleare (noak)".to_string(),
            name_machine: "durata_cantiere_noak".to_string(),
            unit: "anni".to_string(),
            min: 0.0,
            max: 20.0,
            step: 1.0,
            default_value: 5.0,
        },
        SliderConfig {
            name_human: "☢️ Anno primo cantiere nucleare".to_string(),
            name_machine: "anno_inizio_installazioni_nucleare".to_string(),
            unit: "".to_string(),
            min: 2025.0,
            max: 2040.0,
            step: 1.0,
            default_value: 2029.0,
        },
        SliderConfig {
            name_human: "☢️ Anno ultimo cantiere nucleare".to_string(),
            name_machine: "anno_fine_installazioni_nucleare".to_string(),
            unit: "".to_string(),
            min: 2030.0,
            max: 2050.0,
            step: 1.0,
            default_value: 2045.0,
        },
        SliderConfig {
            name_human: "🔌📈 Tasso di crescita consumo".to_string(),
            name_machine: "tasso_crescita_consumo".to_string(),
            unit: "%".to_string(),
            min: 0.0,
            max: 5.0,
            step: 0.1,
            default_value: 2.0,
        },
        SliderConfig {
            name_human: "💦♻️🌋 Produzione altre fonti low-carbon".to_string(),
            name_machine: "produzione_altre_fonti_lowc".to_string(),
            unit: "TWh".to_string(),
            min: 0.0,
            max: 100.0,
            step: 1.0,
            default_value: 50.0,
        },
        SliderConfig {
            name_human: "📅 Anno finale simulazione".to_string(),
            name_machine: "end_year".to_string(),
            unit: "".to_string(),
            min: 2030.0,
            max: 2100.0,
            step: 1.0,
            default_value: 2050.0,
        },
    ];
    serde_json::to_string(&sliders).unwrap()
}
