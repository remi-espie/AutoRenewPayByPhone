use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct RateOption {
    #[serde(rename = "rateOptionId")]
    rate_option_id: String,
    #[serde(rename = "type")]
    r#type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Segment {
    #[serde(rename = "chargeableTimeUnitType")]
    chargeable_time_unit_type: i32,
    #[serde(rename = "chargeableTimeUnitsParked")]
    chargeable_time_units_parked: i32,
    cost: f64,
    fees: f64,
    #[serde(rename = "freeTimeUnitType")]
    free_time_unit_type: i32,
    #[serde(rename = "freeTimeUnitsApplied")]
    free_time_units_applied: i32,
    #[serde(rename = "parkingEnd")]
    parking_end: DateTime<Utc>,
    #[serde(rename = "parkingStart")]
    parking_start: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct TotalCost {
    amount: f64,
    currency: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct ParkedVehicle {
    #[serde(rename = "countryCode")]
    country_code: String,
    id: i32,
    jurisdiction: String,
    #[serde(rename = "licensePlate")]
    pub(crate) license_plate: String,
    #[serde(rename = "type")]
    r#type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct ParkingSession {
    #[serde(rename = "couponApplied")]
    coupon_applied: Option<String>,
    #[serde(rename = "expireTime")]
    expire_time: String,
    #[serde(rename = "fpsApplies")]
    fps_applies: bool,
    #[serde(rename = "isExtendable")]
    is_extendable: bool,
    #[serde(rename = "isRenewable")]
    is_renewable: bool,
    #[serde(rename = "isStoppable")]
    is_stoppable: bool,
    #[serde(rename = "locationId")]
    location_id: String,
    #[serde(rename = "maxStayState")]
    max_stay_state: String,
    #[serde(rename = "parkingSessionId")]
    parking_session_id: String,
    #[serde(rename = "rateOption")]
    rate_option: RateOption,
    #[serde(rename = "renewableAfter")]
    renewable_after: Option<String>,
    segments: Vec<Segment>,
    stall: Option<String>,
    #[serde(rename = "startTime")]
    start_time: String,
    #[serde(rename = "totalCost")]
    total_cost: TotalCost,
    pub(crate) vehicle: ParkedVehicle,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vehicle {
    #[serde(rename = "vehicleId")]
    vehicle_id: String,
    #[serde(rename = "legacyVehicleId")]
    legacy_vehicle_id: i64,
    #[serde(rename = "licensePlate")]
    license_plate: String,
    country: String,
    jurisdiction: String,
    #[serde(rename = "type")]
    r#type: String,
}
