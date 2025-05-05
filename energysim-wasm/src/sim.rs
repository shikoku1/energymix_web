use core::time;

use crate::yearly_time_series::YearlyTimeSeries;

pub struct RenewableGenerationInstallations {
    pub potenza_iniziale: f64,
    pub punti_lerp: Vec<(i32, f64)>,
    pub scenario_start_year: i32,
    pub scenario_stop_year: i32,
}

impl RenewableGenerationInstallations {
    pub fn get_nuova_potenza_installata(&self, anno: i32) -> YearlyTimeSeries {
        let cantieri_iniziati = YearlyTimeSeries::from_lerp(self.punti_lerp.clone());

        let mut nuova_potenza_installata_y =
            YearlyTimeSeries::new(self.scenario_start_year, self.scenario_stop_year);

        for anno in self.scenario_start_year..=self.scenario_stop_year {
            let cantiere = cantieri_iniziati.get(anno);
            nuova_potenza_installata_y.insert_add(anno, cantiere);
        }

        nuova_potenza_installata_y
    }
}

pub struct NuclearGenerationInstallations {
    // Parametri delle barre
    pub reattori_prima_learning: i32, // Numero reattori iniziali
    pub reattori_dopo_learning: i32,  // Numero reattori dopo learning
    pub potenza_reattore: f64,        // Potenza del reattore
    pub numero_reattori: i32,         // Numero totale reattori
    pub anno_inizio_installazioni_nucleare: i32, // Anno inizio costruzione
    pub durata_cantiere_foak: i32,    // Durata primo cantiere
    pub tempo_learning: i32,          // Tempo di learning
}

impl NuclearGenerationInstallations {
    pub fn get_nuova_potenza_installata(&self) -> YearlyTimeSeries {
        // Calcola l'anno di prima operazione nucleare
        let anno_prima_operazione =
            self.anno_inizio_installazioni_nucleare + self.durata_cantiere_foak;

        let anno_inizio_learning = anno_prima_operazione + self.tempo_learning;

        // questo è un higher bound per il numero di anni in cui la cosa può durare
        let mut nuova_potenza_installata_y = YearlyTimeSeries::new(
            self.anno_inizio_installazioni_nucleare,
            self.anno_inizio_installazioni_nucleare * self.numero_reattori,
        );

        // Costruisci i reattori in sequenza
        let mut reattori_costruiti = 0;

        for anno in self.anno_inizio_installazioni_nucleare..=nuova_potenza_installata_y.stop_year {
            if anno < anno_prima_operazione {
                nuova_potenza_installata_y.insert_add(anno, 0.0);
            } else {
                if anno < anno_inizio_learning {
                    for _ in 0..self.reattori_prima_learning {
                        nuova_potenza_installata_y.insert_add(anno, self.potenza_reattore);
                        reattori_costruiti += 1;

                        if reattori_costruiti >= self.numero_reattori {
                            break;
                        }
                    }
                } else {
                    for _ in 0..self.reattori_dopo_learning {
                        nuova_potenza_installata_y.insert_add(anno, self.potenza_reattore);
                        reattori_costruiti += 1;

                        if reattori_costruiti >= self.numero_reattori {
                            break;
                        }
                    }
                }
            }

            if reattori_costruiti >= self.numero_reattori {
                break;
            }
        }

        nuova_potenza_installata_y
    }
}

pub struct SingleSourceSimResult {
    pub anni: Vec<i32>,
    /// Potenza installata in GW, in totale anno per anno
    pub potenza_installata: YearlyTimeSeries,
    /// nova potenza installata in GW, sul singolo anno
    pub nuova_potenza_installata_y: YearlyTimeSeries,
    /// Energia prodotta in GWh
    pub energia_prodotta: YearlyTimeSeries,
    /// Emissioni totali in tonnellate di CO2
    pub emissioni_totali: f64,
    /// Costo totale in euro
    pub costo_totale: f64,
}

impl SingleSourceSimResult {
    pub fn new(start_year: i32, stop_year: i32) -> SingleSourceSimResult {
        let size = (stop_year - start_year) as usize;

        SingleSourceSimResult {
            anni: (start_year..stop_year).collect(),
            potenza_installata: YearlyTimeSeries::new(start_year, stop_year),
            nuova_potenza_installata_y: YearlyTimeSeries::new(start_year, stop_year),
            energia_prodotta: YearlyTimeSeries::new(start_year, stop_year),
            emissioni_totali: 0.0,
            costo_totale: 0.0,
        }
    }
}

/// Scenario per la generazione di energia
#[derive(Default)]
pub struct EnergyGeneratorScenario {
    pub nuova_potenza_installata_y: YearlyTimeSeries,
    /// Anno di inizio del range considerato
    pub scenario_start_year: i32,
    /// Anno di fine del range considerato
    pub scenario_stop_year: i32,
    /// Potenza che è già installata precedentemente all'anno di inizio
    pub potenza_iniziale: f64,
    /// Costo di costruzione di un kW all'anno di inizio
    pub costo_foak: f64,
    /// Costo di costruzione di un kW alla fine
    pub costo_noak: f64,
    /// emissioni di CO2 per kWh prodotto
    pub emissioni_co2: f64,
    /// Quanto dura la costruzione per il primo anno
    pub durata_cantieri_foak: i32,
    /// Quanto dura la costruzione per l'ultimo anno
    pub durata_cantieri_noak: i32,
    /// Fattore di capacità dell'impianto
    pub capacity_factor: f64,
    /// Anni di vita dell'impianto
    pub life_years: i32,
}

impl EnergyGeneratorScenario {
    pub fn generate_time_series(&self) -> SingleSourceSimResult {
        let mut time_series =
            SingleSourceSimResult::new(self.scenario_start_year, self.scenario_stop_year);

        time_series.nuova_potenza_installata_y = self.nuova_potenza_installata_y.clone();

        time_series
            .nuova_potenza_installata_y
            .crop_extend(self.scenario_start_year, self.scenario_stop_year);

        // cumsum di potenza installata annuale per ottenere la potenza installata totale, che rimane negli anni successivi
        time_series.potenza_installata = time_series.nuova_potenza_installata_y.clone();
        time_series.potenza_installata.cumsum();
        time_series
            .potenza_installata
            .add_constant(self.potenza_iniziale);

        time_series.energia_prodotta = time_series.potenza_installata.clone();

        time_series
            .energia_prodotta
            .values
            .iter_mut()
            .for_each(|x| {
                *x = *x * self.capacity_factor * 24.0 * 365.0;
            });

        time_series.emissioni_totali = time_series
            .energia_prodotta
            .values
            .iter()
            .map(|&x| x * self.emissioni_co2)
            .sum();

        time_series
    }
}
