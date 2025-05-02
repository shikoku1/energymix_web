fn lerp(anno_start: f64, valore_start: f64, anno_stop: f64, valore_stop: f64, anno: f64) -> f64 {
    let m = (valore_stop - valore_start) / (anno_stop - anno_start);
    let q = valore_start - m * anno_start;
    m * anno + q
}

pub struct SingleSourceSimResult {
    pub anni: Vec<i32>,
    /// Potenza installata in GW, in totale anno per anno
    pub potenza_installata: Vec<f64>,
    /// nova potenza installata in GW, sul singolo anno
    pub nuova_potenza_installata_y: Vec<f64>,
    /// Energia prodotta in GWh
    pub energia_prodotta: Vec<f64>,
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
            potenza_installata: vec![0.0; size],
            nuova_potenza_installata_y: vec![0.0; size],
            energia_prodotta: vec![0.0; size],
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

        for year in &time_series.anni {
            // se siamo in un anno di costruzione
            if *year >= (self.anno_inizio_installazioni) && *year < self.anno_fine_installazioni {
                // calcoliamo la nuova potenza e la durata del cantiere in base a lerp nell'anno corrente
                // (grossa semplificazione)

                let power_to_build = lerp(
                    self.anno_inizio_installazioni as f64,
                    self.nuova_potenza_annuale_foak,
                    self.anno_fine_installazioni as f64,
                    self.nuova_potenza_annuale_noak,
                    *year as f64,
                );

                let durata_cantiere = lerp(
                    self.anno_inizio_installazioni as f64,
                    self.durata_cantieri_foak as f64,
                    self.anno_fine_installazioni as f64,
                    self.durata_cantieri_noak as f64,
                    *year as f64,
                );

                // ha senso sta cosa??
                let costo = lerp(
                    self.anno_inizio_installazioni as f64,
                    self.costo_foak,
                    self.anno_fine_installazioni as f64,
                    self.costo_noak,
                    *year as f64,
                );

                time_series.costo_totale += costo * power_to_build * 1_000_000.0;
                // ???

                let anno_online = year + durata_cantiere as i32;

                let index = (anno_online - self.scenario_start_year) as usize;

                if index < time_series.potenza_installata.len() {
                    // Aggiungi la potenza all'anno corrente
                    time_series.nuova_potenza_installata_y[index] += power_to_build;
                }
            }
        }

        // info!("{:?}", time_series.nuova_potenza_installata_y);

        // cumsum di potenza installata annuale per ottenere la potenza installata totale, che rimane negli anni successivi
        time_series.potenza_installata = time_series
            .nuova_potenza_installata_y
            .iter()
            .scan(self.potenza_iniziale, |state, &x| {
                *state += x;
                Some(*state)
            })
            .collect();

        time_series.energia_prodotta = time_series
            .potenza_installata
            .iter()
            .map(|&x| x * self.capacity_factor * 24.0 * 365.0)
            .collect();

        time_series.emissioni_totali = time_series
            .energia_prodotta
            .iter()
            .map(|&x| x * self.emissioni_co2)
            .sum();

        time_series
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_lerp() {
        let result = super::lerp(2000.0, 0.0, 2020.0, 1.0, 2010.0);
        assert_eq!(result, 0.5);

        let result = super::lerp(2000.0, 0.0, 2020.0, 1.0, 2020.0);
        assert_eq!(result, 1.0);

        let result = super::lerp(2000.0, 0.0, 2020.0, 1.0, 2000.0);
        assert_eq!(result, 0.0);

        let result = super::lerp(2000.0, 0.0, 2020.0, 100.0, 2010.0);

        assert_eq!(result, 50.0);
    }
}
