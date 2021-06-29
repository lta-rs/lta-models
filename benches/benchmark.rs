use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[rustfmt::skip]
mod de {
    use lta_models::prelude::*;
    use serde::{Deserialize, Serialize};

    pub fn generate_bench<'de, I, S, F>(input_fn: F) -> S
    where
        F: FnOnce() -> &'de str,
        I: Deserialize<'de> + Into<S>,
        S: Serialize,
    {
        let str_data = input_fn();
        serde_json::from_str(str_data).map(|v: I| v.into()).unwrap()
    }

    macro_rules! gen {
        ($a:ty, $b:ty, $c:expr) => {
            generate_bench::<$a, $b, _>(|| include_str!($c));
        };
    }

    pub fn bike_parking() -> Vec<BikeParking> {
        gen!(BikeParkingResp, _, "../dumped_data/bike_parking.json")
    }

    pub fn bus_arrival() -> BusArrivalResp {
        gen!(RawBusArrivalResp, _, "../dumped_data/bus_arrival.json")
    }

    pub fn bus_routes() -> Vec<BusRoute> {
        gen!(BusRouteResp, _, "../dumped_data/bus_route.json")
    }

    pub fn bus_service() -> Vec<BusService> {
        gen!(BusServiceResp, _, "../dumped_data/bus_services.json")
    }

    pub fn bus_stops() -> Vec<BusStop> {
        gen!(BusStopsResp, _, "../dumped_data/bus_stops.json")
    }

    pub fn carpark_avail() -> Vec<CarPark> {
        gen!(CarparkAvailResp, _, "../dumped_data/carpark_avail.json")
    }

    pub fn erp_rates() -> Vec<ErpRate> {
        gen!(ErpRatesResp, _, "../dumped_data/erp_rates.json")
    }

    pub fn faulty_traffic_lights() -> Vec<FaultyTrafficLight> {
        gen!(FaultyTrafficLightResp, _,"../dumped_data/faulty_traffic_lights.json")
    }

    pub fn passenger_vol_bus_stops() -> Vec<String> {
        gen!(PassengerVolRawResp,_,"../dumped_data/passenger_vol_bus_stops.json")
    }

    pub fn passenger_vol_od_bus_stops() -> Vec<String> {
        gen!(PassengerVolRawResp,_,"../dumped_data/passenger_vol_od_bus_stops.json")
    }

    pub fn passenger_vol_od_train() -> Vec<String> {
        gen!(PassengerVolRawResp,_,"../dumped_data/passenger_vol_od_train.json")
    }

    pub fn passenger_vol_train() -> Vec<String> {
        gen!(PassengerVolRawResp,_,"../dumped_data/passenger_vol_train.json")
    }

    pub fn taxi_avail() -> Vec<Coordinates> {
        gen!(TaxiAvailResp, _, "../dumped_data/taxi_avail.json")
    }

    pub fn taxi_stands() -> Vec<TaxiStand> {
        gen!(TaxiStandsResp, _, "../dumped_data/taxi_stands.json")
    }

    pub fn train_service_alert() -> TrainServiceAlert {
        gen!(TrainServiceAlertResp,_,"../dumped_data/train_service_alert.json")
    }

    pub fn est_travel_time() -> Vec<EstTravelTime> {
        gen!(EstTravelTimeResp, _, "../dumped_data/est_travel_time.json")
    }
}

#[rustfmt::skip]
pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("de::bike_parking.json", |b| b.iter(|| de::bike_parking()));
    c.bench_function("de::bus_arrival.json", |b| b.iter(|| de::bus_arrival()));
    c.bench_function("de::bus_route.json", |b| b.iter(|| de::bus_routes()));
    c.bench_function("de::bus_services.json", |b| b.iter(|| de::bus_service()));
    c.bench_function("de::bus_stops.json", |b| b.iter(|| de::bus_stops()));
    c.bench_function("de::carpark_avail.json", |b| b.iter(|| de::carpark_avail()));
    c.bench_function("de::erp_rates.json", |b| b.iter(|| de::erp_rates()));
    c.bench_function("de::est_travel_time.json", |b| b.iter(|| de::est_travel_time()));
    c.bench_function("de::faulty_traffic_lights.json", |b| b.iter(|| de::faulty_traffic_lights()));
    c.bench_function("de::passenger_vol_bus_stops.json", |b| b.iter(|| de::passenger_vol_bus_stops()));
    c.bench_function("de::passenger_od_bus_stops.json", |b| b.iter(|| de::passenger_vol_od_bus_stops()));
    c.bench_function("de::passenger_vol_od_train.json", |b| b.iter(|| de::passenger_vol_od_train()));
    c.bench_function("de::passenger_vol_train.json", |b| b.iter(|| de::passenger_vol_train()));
    c.bench_function("de::taxi_avail.json", |b| b.iter(|| de::taxi_avail()));
    c.bench_function("de::taxi_stands.json", |b| b.iter(|| de::taxi_stands()));
    c.bench_function("de::train_service_alert.json", |b| b.iter(|| de::train_service_alert()));

    let bike_parking = de::bike_parking();
    let bus_arrival = de::bus_arrival();
    let bus_routes = de::bus_routes();
    let bus_service = de::bus_service();
    let bus_stops = de::bus_stops();
    let carpark_avail = de::carpark_avail();
    let erp_rates = de::erp_rates();
    let est_travel_time = de::est_travel_time();
    let faulty_traffic_lights = de::faulty_traffic_lights();
    let passenger_vol_bus_stops = de::passenger_vol_bus_stops();
    let passenger_vol_od_bus_stops = de::passenger_vol_od_bus_stops();
    let passenger_vol_train = de::passenger_vol_train();
    let passenger_vol_od_train = de::passenger_vol_od_train();
    let taxi_avail = de::taxi_avail();
    let taxi_stands = de::taxi_stands();
    let train_service_alert = de::train_service_alert();

    c.bench_function("ser::bike_parking.json", |b| b.iter(|| serde_json::to_string(black_box(&bike_parking))));
    c.bench_function("ser::bus_arrival.json", |b| b.iter(|| serde_json::to_string(black_box(&bus_arrival))));
    c.bench_function("ser::bus_route.json", |b| b.iter(|| serde_json::to_string(black_box(&bus_routes))));
    c.bench_function("ser::bus_services.json", |b| b.iter(|| serde_json::to_string(black_box(&bus_service))));
    c.bench_function("ser::bus_stops.json", |b| b.iter(|| serde_json::to_string(black_box(&bus_stops))));
    c.bench_function("ser::carpark_avail.json", |b| b.iter(|| serde_json::to_string(black_box(&carpark_avail))));
    c.bench_function("ser::erp_rates.json", |b| b.iter(|| serde_json::to_string(black_box(&erp_rates))));
    c.bench_function("ser::est_travel_time.json", |b| b.iter(|| serde_json::to_string(black_box(&est_travel_time))));
    c.bench_function("ser::faulty_traffic_lights.json", |b| b.iter(|| serde_json::to_string(black_box(&faulty_traffic_lights))));
    c.bench_function("ser::passenger_vol_bus_stops.json", |b| b.iter(|| serde_json::to_string(black_box(&passenger_vol_bus_stops))));
    c.bench_function("ser::passenger_od_bus_stops.json", |b| b.iter(|| serde_json::to_string(black_box(&passenger_vol_od_bus_stops))));
    c.bench_function("ser::passenger_vol_od_train.json", |b| b.iter(|| serde_json::to_string(black_box(&passenger_vol_od_train))));
    c.bench_function("ser::passenger_vol_train.json", |b| b.iter(|| serde_json::to_string(black_box(&passenger_vol_train))));
    c.bench_function("ser::taxi_avail.json", |b| b.iter(|| serde_json::to_string(black_box(&taxi_avail))));
    c.bench_function("ser::taxi_stands.json", |b| b.iter(|| serde_json::to_string(black_box(&taxi_stands))));
    c.bench_function("ser::train_service_alert.json", |b| b.iter(|| serde_json::to_string(black_box(&train_service_alert))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
