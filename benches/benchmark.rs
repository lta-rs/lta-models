use criterion::{black_box, criterion_group, criterion_main, Criterion};

mod de {
    use lta_models::prelude::*;
    use serde::{Deserialize, Serialize};
    use std::fmt::Debug;

    pub fn generate_bench<'de, I, S, F>(input_fn: F)
    where
        F: FnOnce() -> &'de str,
        I: Deserialize<'de> + Into<S>,
        S: Serialize + Debug,
    {
        let str_data = input_fn();
        let _: S = serde_json::from_str(str_data).map(|v: I| v.into()).unwrap();
    }

    macro_rules! gen {
        ($a:ty, $b:ty, $c:expr) => {
            generate_bench::<$a, $b, _>(|| include_str!($c));
        };
    }

    pub fn bike_parking() {
        gen!(
            BikeParkingResp,
            Vec<BikeParking>,
            "../dumped_data/bike_parking.json"
        );
    }

    pub fn bus_arrival() {
        gen!(
            RawBusArrivalResp,
            BusArrivalResp,
            "../dumped_data/bus_arrival.json"
        );
    }

    pub fn bus_routes() {
        gen!(BusRouteResp, Vec<BusRoute>, "../dumped_data/bus_route.json");
    }

    pub fn bus_service() {
        gen!(
            BusServiceResp,
            Vec<BusService>,
            "../dumped_data/bus_services.json"
        );
    }

    pub fn bus_stops() {
        gen!(BusStopsResp, Vec<BusStop>, "../dumped_data/bus_stops.json");
    }

    pub fn carpark_avail() {
        gen!(
            CarparkAvailResp,
            Vec<CarPark>,
            "../dumped_data/carpark_avail.json"
        );
    }

    pub fn erp_rates() {
        gen!(ErpRatesResp, Vec<ErpRate>, "../dumped_data/erp_rates.json");
    }

    pub fn faulty_traffic_lights() {
        gen!(
            FaultyTrafficLightResp,
            Vec<FaultyTrafficLight>,
            "../dumped_data/faulty_traffic_lights.json"
        );
    }

    pub fn passenger_vol_bus_stops() {
        gen!(
            PassengerVolRawResp,
            Vec<String>,
            "../dumped_data/passenger_vol_bus_stops.json"
        );
    }

    pub fn passenger_vol_od_bus_stops() {
        gen!(
            PassengerVolRawResp,
            Vec<String>,
            "../dumped_data/passenger_vol_od_bus_stops.json"
        );
    }

    pub fn passenger_vol_od_train() {
        gen!(
            PassengerVolRawResp,
            Vec<String>,
            "../dumped_data/passenger_vol_od_train.json"
        );
    }

    pub fn passenger_vol_train() {
        gen!(
            PassengerVolRawResp,
            Vec<String>,
            "../dumped_data/passenger_vol_train.json"
        );
    }

    pub fn taxi_avail() {
        gen!(
            TaxiAvailResp,
            Vec<Coordinates>,
            "../dumped_data/taxi_avail.json"
        );
    }

    pub fn taxi_stands() {
        gen!(
            TaxiStandsResp,
            Vec<TaxiStand>,
            "../dumped_data/taxi_stands.json"
        );
    }

    pub fn train_service_alert() {
        gen!(
            TrainServiceAlertResp,
            TrainServiceAlert,
            "../dumped_data/train_service_alert.json"
        );
    }

    pub fn est_travel_time() {
        gen!(
            EstTravelTimeResp,
            Vec<EstTravelTime>,
            "../dumped_data/est_travel_time.json"
        );
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("bike_parking.json", |b| b.iter(|| de::bike_parking()));
    c.bench_function("bus_arrival.json", |b| b.iter(|| de::bus_arrival()));
    c.bench_function("bus_route.json", |b| b.iter(|| de::bus_routes()));
    c.bench_function("bus_services.json", |b| b.iter(|| de::bus_service()));
    c.bench_function("bus_stops.json", |b| b.iter(|| de::bus_stops()));
    c.bench_function("carpark_avail.json", |b| b.iter(|| de::carpark_avail()));
    c.bench_function("erp_rates.json", |b| b.iter(|| de::erp_rates()));
    c.bench_function("est_travel_time.json", |b| b.iter(|| de::est_travel_time()));
    c.bench_function("faulty_traffic_lights.json", |b| b.iter(|| de::faulty_traffic_lights()));
    c.bench_function("passenger_vol_bus_stops.json", |b| b.iter(|| de::passenger_vol_bus_stops()));
    c.bench_function("passenger_od_bus_stops.json", |b| b.iter(|| de::passenger_vol_od_bus_stops()));
    c.bench_function("passenger_vol_od_trian.json", |b| b.iter(|| de::passenger_vol_od_train()));
    c.bench_function("passenger_vol_train.json", |b| b.iter(|| de::passenger_vol_train()));
    c.bench_function("taxi_avail.json", |b| b.iter(|| de::taxi_avail()));
    c.bench_function("taxi_stands.json", |b| b.iter(|| de::taxi_stands()));
    c.bench_function("train_service_alert.json", |b| b.iter(|| de::train_service_alert()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
