use js_sys::Array;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use sim::{EnergyGeneratorScenario, SingleSourceSimResult};
use storico::DatoStorico;
use wasm_bindgen::prelude::*;

mod sim;
mod storico;
mod yearly_time_series;

#[wasm_bindgen(start)]
pub fn start() {
    wasm_logger::init(wasm_logger::Config::default());
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone)]
pub struct SimulationSettings {
    pub fotovoltaico_inizio: f64,
    pub fotovoltaico_2030: f64,
    pub fotovoltaico_2035: f64,
    pub fotovoltaico_2040: f64,
    pub fotovoltaico_2050: f64,
    pub fotovoltaico_fine: f64,
    pub eolico_inizio: f64,
    pub eolico_2030: f64,
    pub eolico_2035: f64,
    pub eolico_2040: f64,
    pub eolico_2050: f64,
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
        vec_to_js_array(&self.scenario_fv.energia_prodotta.values)
    }

    #[wasm_bindgen(getter)]
    pub fn pv_capacity(&self) -> Array {
        vec_to_js_array(
            &&self
                .scenario_fv
                .potenza_installata
                .values
                .iter()
                .map(|x| x * 1000.0)
                .collect::<Vec<f64>>(),
        )
    }

    #[wasm_bindgen(getter)]
    pub fn wind_production(&self) -> Array {
        vec_to_js_array(&self.scenario_eol.energia_prodotta.values)
    }

    #[wasm_bindgen(getter)]
    pub fn wind_capacity(&self) -> Array {
        vec_to_js_array(
            &&self
                .scenario_eol
                .potenza_installata
                .values
                .iter()
                .map(|x| x * 1000.0)
                .collect::<Vec<f64>>(),
        )
    }

    #[wasm_bindgen(getter)]
    pub fn nuclear_production(&self) -> Array {
        vec_to_js_array(&self.scenario_nucl.energia_prodotta.values)
    }

    #[wasm_bindgen(getter)]
    pub fn nuclear_capacity(&self) -> Array {
        vec_to_js_array(
            &&self
                .scenario_nucl
                .potenza_installata
                .values
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

    // Costruisci i punti di interpolazione per fotovoltaico
    let mut punti_lerp_fv = vec![(start_year, settings.fotovoltaico_inizio)];
    let mut punti_lerp_eol = vec![(start_year, settings.eolico_inizio)];

    // Aggiungi punti intermedi solo se rientrano nell'intervallo della simulazione
    if start_year <= 2030 && 2030 <= end_year {
        punti_lerp_fv.push((2030, settings.fotovoltaico_2030));
        punti_lerp_eol.push((2030, settings.eolico_2030));
    }
    if start_year <= 2035 && 2035 <= end_year {
        punti_lerp_fv.push((2035, settings.fotovoltaico_2035));
        punti_lerp_eol.push((2035, settings.eolico_2035));
    }
    if start_year <= 2040 && 2040 <= end_year {
        punti_lerp_fv.push((2040, settings.fotovoltaico_2040));
        punti_lerp_eol.push((2040, settings.eolico_2040));
    }
    if start_year <= 2050 && 2050 <= end_year {
        punti_lerp_fv.push((2050, settings.fotovoltaico_2050));
        punti_lerp_eol.push((2050, settings.eolico_2050));
    }

    // Aggiungi l'anno finale solo se diverso dagli altri punti
    if !punti_lerp_fv.iter().any(|(anno, _)| *anno == end_year) {
        punti_lerp_fv.push((end_year, settings.fotovoltaico_fine));
    }

    // Aggiungi l'anno finale solo se diverso dagli altri punti
    if !punti_lerp_eol.iter().any(|(anno, _)| *anno == end_year) {
        punti_lerp_eol.push((end_year, settings.eolico_fine));
    }

    let fottovoltaico = EnergyGeneratorScenario {
        scenario_start_year: start_year,
        scenario_stop_year: end_year,
        anno_inizio_installazioni: start_year,
        anno_fine_installazioni: end_year,
        potenza_iniziale: *storico::get_potenza_installata()
            .fotovoltaico
            .last()
            .unwrap()
            / 1000.0,
        punti_lerp: punti_lerp_fv,
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
        potenza_iniziale: *storico::get_potenza_installata().eolico.last().unwrap() / 1000.0,
        punti_lerp: punti_lerp_eol,
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
        punti_lerp: vec![
            (
                settings.anno_inizio_installazioni_nucleare,
                settings.nucleare_inizio,
            ),
            (
                settings.anno_fine_installazioni_nucleare,
                settings.nucleare_fine,
            ),
        ],
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

#[derive(Serialize, Deserialize)]
pub struct SliderSection {
    pub name: String,
    pub sliders: Vec<SliderConfig>,
}

#[wasm_bindgen]
pub fn get_sliders_json() -> String {
    let sections = vec![
        SliderSection {
            name: "☀️ Fotovoltaico".to_string(),
            sliders: vec![
                SliderConfig {
                    name_human: "Potenza installata il primo anno".to_string(),
                    name_machine: "fotovoltaico_inizio".to_string(),
                    unit: "GW".to_string(),
                    min: 0.0,
                    max: 10.0,
                    step: 0.1,
                    default_value: 1.0,
                },
                SliderConfig {
                    name_human: "Potenza installata 2030".to_string(),
                    name_machine: "fotovoltaico_2030".to_string(),
                    unit: "GW".to_string(),
                    min: 0.0,
                    max: 10.0,
                    step: 0.1,
                    default_value: 1.3,
                },
                SliderConfig {
                    name_human: "Potenza installata 2035".to_string(),
                    name_machine: "fotovoltaico_2035".to_string(),
                    unit: "GW".to_string(),
                    min: 0.0,
                    max: 10.0,
                    step: 0.1,
                    default_value: 1.5,
                },
                SliderConfig {
                    name_human: "Potenza installata 2040".to_string(),
                    name_machine: "fotovoltaico_2040".to_string(),
                    unit: "GW".to_string(),
                    min: 0.0,
                    max: 10.0,
                    step: 0.1,
                    default_value: 1.7,
                },
                SliderConfig {
                    name_human: "Potenza installata 2050".to_string(),
                    name_machine: "fotovoltaico_2050".to_string(),
                    unit: "GW".to_string(),
                    min: 0.0,
                    max: 10.0,
                    step: 0.1,
                    default_value: 1.9,
                },
                SliderConfig {
                    name_human: "Potenza installata l'ultimo anno".to_string(),
                    name_machine: "fotovoltaico_fine".to_string(),
                    unit: "GW".to_string(),
                    min: 0.0,
                    max: 10.0,
                    step: 0.1,
                    default_value: 2.0,
                },
            ],
        },
        SliderSection {
            name: "💨 Eolico".to_string(),
            sliders: vec![
                SliderConfig {
                    name_human: "Potenza installata il primo anno".to_string(),
                    name_machine: "eolico_inizio".to_string(),
                    unit: "GW".to_string(),
                    min: 0.0,
                    max: 10.0,
                    step: 0.1,
                    default_value: 0.5,
                },
                SliderConfig {
                    name_human: "Potenza installata 2030".to_string(),
                    name_machine: "eolico_2030".to_string(),
                    unit: "GW".to_string(),
                    min: 0.0,
                    max: 10.0,
                    step: 0.1,
                    default_value: 0.6,
                },
                SliderConfig {
                    name_human: "Potenza installata 2035".to_string(),
                    name_machine: "eolico_2035".to_string(),
                    unit: "GW".to_string(),
                    min: 0.0,
                    max: 10.0,
                    step: 0.1,
                    default_value: 0.7,
                },
                SliderConfig {
                    name_human: "Potenza installata 2040".to_string(),
                    name_machine: "eolico_2040".to_string(),
                    unit: "GW".to_string(),
                    min: 0.0,
                    max: 10.0,
                    step: 0.1,
                    default_value: 0.8,
                },
                SliderConfig {
                    name_human: "Potenza installata 2050".to_string(),
                    name_machine: "eolico_2050".to_string(),
                    unit: "GW".to_string(),
                    min: 0.0,
                    max: 10.0,
                    step: 0.1,
                    default_value: 0.9,
                },
                SliderConfig {
                    name_human: "Potenza installata l'ultimo anno".to_string(),
                    name_machine: "eolico_fine".to_string(),
                    unit: "GW".to_string(),
                    min: 0.0,
                    max: 10.0,
                    step: 0.1,
                    default_value: 1.0,
                },
            ],
        },
        SliderSection {
            name: "☢️ Nucleare".to_string(),
            sliders: vec![
                SliderConfig {
                    name_human: "Potenza installata il primo anno".to_string(),
                    name_machine: "nucleare_inizio".to_string(),
                    unit: "GW".to_string(),
                    min: 0.0,
                    max: 10.0,
                    step: 0.1,
                    default_value: 2.0,
                },
                SliderConfig {
                    name_human: "Potenza installata l'ultimo anno".to_string(),
                    name_machine: "nucleare_fine".to_string(),
                    unit: "GW".to_string(),
                    min: 0.0,
                    max: 10.0,
                    step: 0.1,
                    default_value: 3.0,
                },
                SliderConfig {
                    name_human: "Durata Cantiere (foak)".to_string(),
                    name_machine: "durata_cantiere_foak".to_string(),
                    unit: "anni".to_string(),
                    min: 0.0,
                    max: 20.0,
                    step: 1.0,
                    default_value: 8.0,
                },
                SliderConfig {
                    name_human: "Durata Cantiere (noak)".to_string(),
                    name_machine: "durata_cantiere_noak".to_string(),
                    unit: "anni".to_string(),
                    min: 0.0,
                    max: 20.0,
                    step: 1.0,
                    default_value: 5.0,
                },
                SliderConfig {
                    name_human: "Anno inizio primo cantiere".to_string(),
                    name_machine: "anno_inizio_installazioni_nucleare".to_string(),
                    unit: "".to_string(),
                    min: 2025.0,
                    max: 2040.0,
                    step: 1.0,
                    default_value: 2029.0,
                },
                SliderConfig {
                    name_human: "Anno ultimo cantiere".to_string(),
                    name_machine: "anno_fine_installazioni_nucleare".to_string(),
                    unit: "".to_string(),
                    min: 2030.0,
                    max: 2050.0,
                    step: 1.0,
                    default_value: 2045.0,
                },
            ],
        },
        SliderSection {
            name: "⚡ Consumo e Altre Fonti".to_string(),
            sliders: vec![
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
                    default_value: 2055.0,
                },
            ],
        },
    ];
    serde_json::to_string(&sections).unwrap()
}
