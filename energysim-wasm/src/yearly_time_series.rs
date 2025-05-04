fn lerp(anno_start: f64, valore_start: f64, anno_stop: f64, valore_stop: f64, anno: f64) -> f64 {
    let m = (valore_stop - valore_start) / (anno_stop - anno_start);
    let q = valore_start - m * anno_start;
    m * anno + q
}

#[derive(Debug, Clone)]
pub struct YearlyTimeSeries {
    pub start_year: i32,
    pub stop_year: i32,
    pub values: Vec<f64>,
}

impl YearlyTimeSeries {
    pub fn new(start_year: i32, stop_year: i32) -> YearlyTimeSeries {
        let size = (stop_year - start_year + 1) as usize;

        YearlyTimeSeries {
            start_year,
            stop_year,
            values: vec![0.0; size],
        }
    }

    pub fn insert(&mut self, year: i32, value: f64) {
        if year < self.start_year || year > self.stop_year {
            //panic!("Year out of range");
            println!("Year out of range");
            return;
        }
        let index = (year - self.start_year) as usize;
        self.values[index] = value;
    }

    pub fn insert_add(&mut self, year: i32, value: f64) {
        if year < self.start_year || year > self.stop_year {
            //panic!("Year out of range");
            println!("Year out of range");
            return;
        }
        let index = (year - self.start_year) as usize;
        self.values[index] += value;
    }

    pub fn get(&self, year: i32) -> f64 {
        if year < self.start_year || year > self.stop_year {
            panic!("Year out of range");
        }
        let index = (year - self.start_year) as usize;
        self.values[index]
    }

    pub fn from_lerp(values_to_interpolate: Vec<(i32, f64)>) -> YearlyTimeSeries {
        let mut sorted_values = values_to_interpolate.clone();
        sorted_values.sort_by(|a, b| a.0.cmp(&b.0));

        let start_year = sorted_values[0].0;
        let stop_year = sorted_values[sorted_values.len() - 1].0;

        let mut time_series = YearlyTimeSeries::new(start_year, stop_year);

        for i in 0..(sorted_values.len() - 1) {
            let (anno_start, valore_start) = sorted_values[i];
            let (anno_stop, valore_stop) = sorted_values[i + 1];

            for anno in anno_start..=anno_stop {
                time_series.values[(anno - start_year) as usize] = lerp(
                    anno_start as f64,
                    valore_start,
                    anno_stop as f64,
                    valore_stop,
                    anno as f64,
                );
            }
        }

        time_series
    }

    pub fn crop_extend(&mut self, new_start_year: i32, new_stop_year: i32) {
        if new_start_year < self.start_year {
            // add leading zeros
            self.values = vec![0.0; (self.start_year - new_start_year) as usize]
                .into_iter()
                .chain(self.values.iter().cloned())
                .collect();
            self.start_year = new_start_year;
        }

        if new_stop_year > self.stop_year {
            let new_size = (new_stop_year - self.start_year + 1) as usize;
            self.values.resize(new_size, 0.0);
            self.stop_year = new_stop_year;
        }

        if new_start_year > self.start_year {
            let offset = (new_start_year - self.start_year) as usize;
            self.values.drain(0..offset);
            self.start_year = new_start_year;
        }
        if new_stop_year < self.stop_year {
            let offset = (self.stop_year - new_stop_year) as usize;
            self.values.drain(self.values.len() - offset..);
            self.stop_year = new_stop_year;
        }
    }

    pub fn cumsum(&mut self) {
        let mut sum = 0.0;
        for i in 0..self.values.len() {
            sum += self.values[i];
            self.values[i] = sum;
        }
    }

    pub fn add_constant(&mut self, constant: f64) {
        for i in 0..self.values.len() {
            self.values[i] += constant;
        }
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

    #[test]
    fn test_yearly_time_series() {
        let values = vec![(2000, 0.0), (2010, 1.0), (2020, 2.0)];
        let time_series = super::YearlyTimeSeries::from_lerp(values);

        assert_eq!(time_series.start_year, 2000);
        assert_eq!(time_series.stop_year, 2020);
        assert_eq!(time_series.values.len(), 21);
        assert_eq!(time_series.values[10], 1.0);
        assert_eq!(time_series.values[20], 2.0);
    }

    #[test]
    fn test_yearly_time_series_crop() {
        let values = vec![(2000, 0.0), (2010, 1.0), (2020, 2.0)];
        let mut time_series = super::YearlyTimeSeries::from_lerp(values);

        time_series.crop_extend(1990, 2030);

        assert_eq!(time_series.start_year, 1990);
        assert_eq!(time_series.stop_year, 2030);
        assert_eq!(time_series.values.len(), 41);

        assert_eq!(time_series.values[0], 0.0);
        assert_eq!(time_series.values[10], 0.0);
        assert_eq!(time_series.values[20], 1.0);
    }

    #[test]
    fn test_insert() {
        let mut time_series = super::YearlyTimeSeries::new(2000, 2020);
        time_series.insert(2005, 1.0);
        assert_eq!(time_series.values[5], 1.0);
        time_series.insert(2010, 2.0);
        assert_eq!(time_series.values[10], 2.0);
        time_series.insert(2015, 3.0);
        assert_eq!(time_series.values[15], 3.0);
    }
}
