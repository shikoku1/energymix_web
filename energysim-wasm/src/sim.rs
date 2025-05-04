use core::time;

use crate::yearly_time_series::YearlyTimeSeries;

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
    /// Anno di inizio del range considerato
    pub scenario_start_year: i32,
    /// Anno di fine del range considerato
    pub scenario_stop_year: i32,
    /// Potenza che è già installata precedentemente all'anno di inizio
    pub potenza_iniziale: f64,
    /// Anno di inizio della costruzione di nuovi impianti
    pub anno_inizio_installazioni: i32,
    /// Anno di fine della costruzione di nuovi impianti
    pub anno_fine_installazioni: i32,
    /// Quanta potenza installata viene iniziata a costruire ogni anno, GW, il primo anno
    pub nuova_potenza_annuale_foak: f64,
    /// Quanta potenza installata viene iniziata a costruire ogni anno, GW, l'ultimo anno
    pub nuova_potenza_annuale_noak: f64,
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

        // quanto iniziare a costruire ogni anno, occhio che non corrisponde alla potenza installata
        // perchè i cantieri durano più anni
        let cantieri_iniziati = YearlyTimeSeries::from_lerp(vec![
            (
                self.anno_inizio_installazioni,
                self.nuova_potenza_annuale_foak,
            ),
            (
                self.anno_fine_installazioni,
                self.nuova_potenza_annuale_noak,
            ),
        ]);

        // quanto dura il cantiere, in anni
        let durata_cantieri = YearlyTimeSeries::from_lerp(vec![
            (
                self.anno_inizio_installazioni,
                self.durata_cantieri_foak as f64,
            ),
            (
                self.anno_fine_installazioni,
                self.durata_cantieri_noak as f64,
            ),
        ]);

        for anno in self.anno_inizio_installazioni..=self.anno_fine_installazioni {
            let cantiere = cantieri_iniziati.get(anno);
            let durata = durata_cantieri.get(anno);

            time_series
                .nuova_potenza_installata_y
                .insert_add(anno + durata as i32, cantiere);
        }

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
