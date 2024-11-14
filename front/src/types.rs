use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub(crate) struct AppContext {
    pub(crate) api_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Accounts {
    pub(crate) accounts: Vec<Config>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct Config {
    pub(crate) name: String,
    pub(crate) plate: String,
    pub(crate) lot: i32,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Parking {
    pub(crate) name: String,
    pub(crate) duration: i16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct RateOption {
    #[serde(rename = "rateOptionId")]
    rate_option_id: String,
    #[serde(rename = "type")]
    r#type: String,
}

impl RateOption {
    fn default() -> RateOption {
        Self {
            rate_option_id: "".to_string(),
            r#type: "".to_string(),
        }
    }
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

impl TotalCost {
    fn default() -> TotalCost {
        Self {
            amount: 0.0,
            currency: "".to_string(),
        }
    }
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

impl ParkedVehicle {
    fn default() -> ParkedVehicle {
        Self {
            country_code: "".to_string(),
            id: 0,
            jurisdiction: "".to_string(),
            license_plate: "".to_string(),
            r#type: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct ParkingSession {
    #[serde(rename = "couponApplied")]
    coupon_applied: Option<String>,
    #[serde(rename = "expireTime")]
    pub(crate) expire_time: String,
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
    pub(crate) start_time: String,
    #[serde(rename = "totalCost")]
    total_cost: TotalCost,
    pub(crate) vehicle: ParkedVehicle,
}

impl ParkingSession {
    pub(crate) fn default() -> ParkingSession {
        Self {
            coupon_applied: None,
            expire_time: "".to_string(),
            fps_applies: false,
            is_extendable: false,
            is_renewable: false,
            is_stoppable: false,
            location_id: "".to_string(),
            max_stay_state: "".to_string(),
            parking_session_id: "".to_string(),
            rate_option: RateOption::default(),
            renewable_after: None,
            segments: vec![],
            stall: None,
            start_time: "".to_string(),
            total_cost: TotalCost::default(),
            vehicle: ParkedVehicle::default(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct RenewSession {
    #[serde(rename = "nextCheck")]
    pub(crate) next_check: String,
    pub(crate) duration: i16,
}