use crate::{
    core::{
        stock_exchange::{StockExchange, StockExchangeSettings},
        time::TimeHandler,
    },
    logger::Logger,
    simulation::{settings::SimulationSettings, Simulation, SimulationState},
    storage::{prometheus::StoragePrometheusImpl, redis::StorageRedisImpl},
};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use log::{debug, error};
use prometheus_metrics::build_server_prometheus_metrics;
use serde_json::json;
use std::{
    sync::{Arc, RwLock},
    thread,
    time::{SystemTime, UNIX_EPOCH},
};
use storage_wrappers::{
    load_simulation_state, save_simulation_state, LoadSimulationStateError, RedisPriceStorage,
};

mod prometheus_metrics;
mod storage_wrappers;

type SEWrapper = Arc<RwLock<StockExchange>>;
type TimeWrapper = Arc<RwLock<TimeHandler>>;
type SimulationSettingsWrapper = Arc<RwLock<SimulationSettings>>;

impl StockExchange {
    fn get_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}

impl TimeHandler {
    fn get_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}

#[get("/health")]
async fn get_health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[get("/prometheus/metrics")]
async fn get_prometheus_metrics(
    se: web::Data<SEWrapper>,
    time: web::Data<TimeWrapper>,
    simulation_settings: web::Data<SimulationSettingsWrapper>,
) -> actix_web::Result<HttpResponse> {
    let (time, se, simulation_settings) = {
        let time_inner = time.read().unwrap();
        let se_inner = se.read().unwrap();
        let simulation_settings = simulation_settings.read().unwrap();

        (
            time_inner.clone(),
            se_inner.clone(),
            simulation_settings.clone(),
        )
    };

    build_server_prometheus_metrics(time, se, simulation_settings).map_or_else(
        |e| {
            error!("Failed to build prometheus metrics: {}", e);
            Ok(HttpResponse::InternalServerError().finish())
        },
        |response| Ok(HttpResponse::Ok().body(response)),
    )
}

#[get("/grafana/data")]
async fn get_grafana_data(
    time: web::Data<TimeWrapper>,
    simulation_settings: web::Data<SimulationSettingsWrapper>,
) -> actix_web::Result<HttpResponse> {
    let (time, simulation_settings) = {
        let time_inner = time.read().unwrap();
        let simulation_settings = simulation_settings.read().unwrap();

        (time_inner.clone(), simulation_settings.clone())
    };

    let response = json!({
        "current_time": time.get_virtual_time_formatted(),
        "simulation_settings": {
            "max_orders_per_tick": simulation_settings.max_orders_per_tick,
            "flush_storage": simulation_settings.flush_storage,
            "max_investor_age": simulation_settings.max_investor_age,
            "max_orders_per_tick": simulation_settings.max_orders_per_tick,
        }
    });

    Ok(HttpResponse::Ok().json(response))
}

const DEFAULT_SEED: [u8; 32] = [
    0x1b, 0x2e, 0x3d, 0x4c, 0x5a, 0x69, 0x78, 0x87, 0x96, 0xa5, 0xb4, 0xc3, 0xd2, 0xe1, 0xf0, 0x0f,
    0x1e, 0x2d, 0x3c, 0x4b, 0x5a, 0x69, 0x78, 0x87, 0x96, 0xa5, 0xb4, 0xc3, 0xd2, 0xe1, 0xf0, 0x0f,
];

pub async fn run_server(simulation_settings: SimulationSettings) -> Result<(), String> {
    let se_settings = StockExchangeSettings {
        name: "Market Simulator".to_string(),
        location: "Hong Kong".to_string(),
        timezone: "UTC+8".to_string(),
        trading_days: vec![0, 1, 2, 3, 4],
        trading_hours: vec![9, 10, 11, 12, 13, 14, 15],
        ..Default::default()
    };

    let create_new_state = || {
        let se = StockExchange::new(se_settings);
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let time = TimeHandler::new(now, None);

        (se, time)
    };
    let mut redis_storage: StorageRedisImpl = simulation_settings.clone().into();
    let prometheus_storage: StoragePrometheusImpl = simulation_settings.clone().into();

    let (se, time) = if simulation_settings.flush_storage {
        Simulation::flush_data(&mut redis_storage, &prometheus_storage)
            .await
            .unwrap_or_else(|e| {
                println!("Could not flush data: {}", e);
                std::process::exit(1);
            });

        create_new_state()
    } else {
        let simulation_state =
            load_simulation_state(&simulation_settings).unwrap_or_else(|e| match e {
                LoadSimulationStateError::Empty => {
                    debug!("No simulation state found, creating a new one");
                    let (se, time) = create_new_state();
                    SimulationState { se, time }
                }
                _ => {
                    error!("Failed to load simulation state");
                    std::process::exit(1);
                }
            });

        (simulation_state.se, simulation_state.time)
    };

    let se_wrapper = Arc::new(RwLock::new(se));
    let time_wrapper = Arc::new(RwLock::new(time));
    let simulation_settings_wrapper = Arc::new(RwLock::new(simulation_settings.clone()));

    let se_1 = se_wrapper.clone();
    let time_1 = time_wrapper.clone();

    let sim_settings = simulation_settings.clone();

    thread::spawn(move || {
        let redis_price_storage = RedisPriceStorage::new(&simulation_settings);

        let mut simulation = Simulation::new(
            DEFAULT_SEED,
            simulation_settings.clone(),
            redis_price_storage,
        );

        let logger = Logger::new();
        logger.setup_level(&log::LevelFilter::Debug);

        println!("{}", simulation_settings.to_string());

        {
            let mut se_inner = se_1.write().unwrap();
            let time_inner = time_1.read().unwrap();

            simulation
                .init(&mut se_inner, &time_inner)
                .unwrap_or_else(|e| {
                    error!("Failed to initialize the simulator: {}", e);
                    std::process::exit(1);
                });
        }

        let mut redis_storage: StorageRedisImpl = simulation_settings.clone().into();

        loop {
            {
                let mut se_inner = se_1.write().unwrap();
                let time_inner = time_1.read().unwrap();

                simulation
                    .run(&mut se_inner, &time_inner)
                    .unwrap_or_else(|e| {
                        error!("Failed to run the simulator: {}", e);
                        std::process::exit(1);
                    });
            }

            {
                let mut time_inner = time_1.write().unwrap();
                time_inner.tick();
            }

            {
                let se_inner = se_1.read().unwrap();
                let time_inner = time_1.read().unwrap();

                let simulation_state = crate::simulation::SimulationState {
                    se: se_inner.clone(),
                    time: time_inner.clone(),
                };

                save_simulation_state(&mut redis_storage, &simulation_state).unwrap_or_else(|e| {
                    error!("Failed to save simulation state: {}", e);
                    std::process::exit(1);
                });
            }

            let time_to_wait = {
                let time_inner = time_1.read().unwrap();
                time_inner.get_wait_millis()
            };

            std::thread::sleep(std::time::Duration::from_millis(time_to_wait));
        }
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(se_wrapper.clone()))
            .app_data(web::Data::new(time_wrapper.clone()))
            .app_data(web::Data::new(simulation_settings_wrapper.clone()))
            .service(get_health)
            .service(get_prometheus_metrics)
            .service(get_grafana_data)
    })
    .bind((
        sim_settings.address.clone(),
        sim_settings.port.clone().parse::<u16>().unwrap(),
    ))
    .map_err(|e| format!("failed to bind the server: {}", e))?
    .run()
    .await
    .map_err(|e| format!("Failed to run the server: {}", e))?;

    Ok(())
}
