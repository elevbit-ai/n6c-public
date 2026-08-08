use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use specter_common::*;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub type SharedState = Arc<RwLock<AppState>>;

#[derive(Clone)]
pub struct AppState {
    pub sensors: Vec<SensorInfo>,
    pub radios: Vec<RadioInfo>,
    pub events: Vec<RFEvent>,
    pub alerts: Vec<Alert>,
    pub measurements: Vec<SpectrumMeasurement>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            sensors: Vec::new(),
            radios: Vec::new(),
            events: Vec::new(),
            alerts: Vec::new(),
            measurements: Vec::new(),
        }
    }
}

pub fn create_router(state: SharedState) -> Router {
    Router::new()
        .route("/api/v1/sensors", get(list_sensors))
        .route("/api/v1/sensors/{id}", get(get_sensor))
        .route("/api/v1/spectrum/current", get(get_current_spectrum))
        .route("/api/v1/events", get(list_events))
        .route("/api/v1/radios", get(list_radios))
        .route("/api/v1/radios/{id}/channel-change", post(change_channel))
        .route("/api/v1/channels", get(list_channels))
        .route("/api/v1/alerts", get(list_alerts))
        .route("/api/v1/audit", get(list_audit))
        .route("/api/v1/health", get(health_check))
        .with_state(state)
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION"),
        "service": "specter-core"
    }))
}

async fn list_sensors(
    State(state): State<SharedState>,
) -> Result<Json<Vec<SensorInfo>>, StatusCode> {
    let data = state.read().await;
    Ok(Json(data.sensors.clone()))
}

async fn get_sensor(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> Result<Json<SensorInfo>, StatusCode> {
    let data = state.read().await;
    data.sensors
        .iter()
        .find(|s| s.id == id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn get_current_spectrum(
    State(state): State<SharedState>,
) -> Result<Json<Vec<SpectrumMeasurement>>, StatusCode> {
    let data = state.read().await;
    Ok(Json(data.measurements.clone()))
}

async fn list_events(
    State(state): State<SharedState>,
) -> Result<Json<Vec<RFEvent>>, StatusCode> {
    let data = state.read().await;
    Ok(Json(data.events.clone()))
}

async fn list_radios(
    State(state): State<SharedState>,
) -> Result<Json<Vec<RadioInfo>>, StatusCode> {
    let data = state.read().await;
    Ok(Json(data.radios.clone()))
}

async fn list_channels(
    State(state): State<SharedState>,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    let data = state.read().await;
    let channels: Vec<serde_json::Value> = data
        .radios
        .iter()
        .flat_map(|r| {
            r.allowed_channels.iter().map(move |ch| {
                serde_json::json!({
                    "radio_id": r.id,
                    "channel": ch,
                    "frequency_hz": r.current_frequency_hz,
                })
            })
        })
        .collect();
    Ok(Json(channels))
}

async fn list_alerts(
    State(state): State<SharedState>,
) -> Result<Json<Vec<Alert>>, StatusCode> {
    let data = state.read().await;
    Ok(Json(data.alerts.clone()))
}

async fn list_audit(
    State(state): State<SharedState>,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    let data = state.read().await;
    let audit: Vec<serde_json::Value> = data
        .events
        .iter()
        .map(|e| {
            serde_json::json!({
                "id": e.id,
                "timestamp": e.timestamp,
                "type": e.event_type,
                "confidence": e.confidence,
                "description": e.description,
            })
        })
        .collect();
    Ok(Json(audit))
}

#[derive(Deserialize)]
pub struct ChannelChangeRequest {
    pub target_channel: u32,
    pub reason: String,
}

#[derive(Serialize)]
pub struct ChannelChangeResponse {
    pub success: bool,
    pub message: String,
    pub change_id: Option<Uuid>,
}

async fn change_channel(
    State(state): State<SharedState>,
    Path(radio_id): Path<Uuid>,
    Json(req): Json<ChannelChangeRequest>,
) -> Result<Json<ChannelChangeResponse>, StatusCode> {
    let data = state.read().await;
    let radio = data
        .radios
        .iter()
        .find(|r| r.id == radio_id)
        .ok_or(StatusCode::NOT_FOUND)?;
    if !radio.allowed_channels.contains(&req.target_channel) {
        return Ok(Json(ChannelChangeResponse {
            success: false,
            message: format!("Channel {} not in allowlist", req.target_channel),
            change_id: None,
        }));
    }
    let change_id = Uuid::new_v4();
    tracing::info!(
        "Channel change requested: radio={}, from={}, to={}, reason={}",
        radio_id,
        radio.current_channel,
        req.target_channel,
        req.reason
    );
    Ok(Json(ChannelChangeResponse {
        success: true,
        message: "Channel change accepted".to_string(),
        change_id: Some(change_id),
    }))
}
