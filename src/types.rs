use chrono::{DateTime, FixedOffset, Utc};
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Quote {
    #[serde(rename = "locationId")]
    location_id: String,
    pub(crate) stall: Option<String>,
    #[serde(rename = "quoteDate")]
    quote_date: DateTime<Utc>,
    #[serde(rename = "totalCost")]
    total_cost: Cost,
    #[serde(rename = "parkingAccountId")]
    parking_account_id: String,
    #[serde(rename = "parkingStartTime")]
    pub(crate) parking_start_time: DateTime<Utc>,
    #[serde(rename = "parkingExpiryTime")]
    parking_expiry_time: DateTime<Utc>,
    #[serde(rename = "parkingDurationAdjustment")]
    parking_duration_adjustment: String,
    #[serde(rename = "licensePlate")]
    license_plate: String,
    #[serde(rename = "quoteItems")]
    pub(crate) quote_items: Vec<QuoteItem>,
    #[serde(rename = "quoteId")]
    pub(crate) quote_id: String,
    #[serde(rename = "promotionApplied")]
    promotion_applied: Option<String>,
    profile: Profile,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cost {
    amount: f64,
    currency: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuoteItem {
    #[serde(rename = "quoteItemType")]
    quote_item_type: String,
    name: String,
    #[serde(rename = "costAmount")]
    cost_amount: Cost,
    #[serde(rename = "subQuoteItems")]
    sub_quote_items: Option<Vec<SubQuoteItem>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubQuoteItem {
    #[serde(rename = "quoteItemType")]
    quote_item_type: String,
    name: String,
    #[serde(rename = "costAmount")]
    cost_amount: Cost,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    #[serde(rename = "profileName")]
    profile_name: String,
    icon: Option<String>,
    #[serde(rename = "userMessages")]
    user_messages: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParkingOption {
    name: String,
    r#type: String,
    #[serde(rename = "rateOptionId")]
    pub(crate) rate_option_id: String,
    #[serde(rename = "effectiveMaxStayDuration")]
    effective_max_stay_duration: Duration,
    #[serde(rename = "maxStayStatus")]
    max_stay_status: String,
    #[serde(rename = "policyType")]
    policy_type: String,
    #[serde(rename = "maxStayEndTime")]
    max_stay_end_time: DateTime<FixedOffset>,
    #[serde(rename = "acceptedTimeUnits")]
    accepted_time_units: Vec<String>,
    #[serde(rename = "restrictionPeriods")]
    restriction_periods: Vec<RestrictionPeriod>,
    #[serde(rename = "availableTimeUnitsWithRestrictions")]
    available_time_units_with_restrictions: AvailableTimeUnitsWithRestrictions,
    #[serde(rename = "timeSteps")]
    time_steps: Option<String>,
    #[serde(rename = "isDefault")]
    is_default: bool,
    #[serde(rename = "isPreferred")]
    is_preferred: bool,
    #[serde(rename = "licensePlate")]
    license_plate: String,
    areas: Vec<String>,
    fps: Option<String>,
    #[serde(rename = "availablePromotions")]
    available_promotions: Vec<String>,
    #[serde(rename = "renewalParking")]
    renewal_parking: RenewalParking,
    #[serde(rename = "eligibilityEndDate")]
    eligibility_end_date: Option<String>,
    profile: Profile,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Duration {
    pub(crate) quantity: u16,
    #[serde(rename = "timeUnit")]
    pub(crate) time_unit: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RestrictionPeriod {
    #[serde(rename = "startTime")]
    start_time: DateTime<FixedOffset>,
    #[serde(rename = "endTime")]
    end_time: DateTime<FixedOffset>,
    #[serde(rename = "maxStay")]
    max_stay: Duration,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AvailableTimeUnitsWithRestrictions {
    minutes: TimeUnitRestriction,
    hours: TimeUnitRestriction,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeUnitRestriction {
    duration: Duration,
    #[serde(rename = "endTime")]
    end_time: DateTime<FixedOffset>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RenewalParking {
    #[serde(rename = "isAllowed")]
    is_allowed: bool,
    window: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostQuote {
    #[serde(rename = "licensePlate")]
    pub(crate) license_plate: String,
    #[serde(rename = "locationId")]
    pub(crate) location_id: String,
    pub(crate) stall: Option<String>,
    #[serde(rename = "rateOptionId")]
    pub(crate) rate_option_id: String,
    #[serde(rename = "startTime")]
    pub(crate) start_time: DateTime<Utc>,
    #[serde(rename = "quoteId")]
    pub(crate) quote_id: String,
    pub(crate) duration: Duration,
    #[serde(rename = "paymentMethod")]
    pub(crate) payment_method: PaymentMethod,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentMethod {
    #[serde(rename = "paymentMethodType")]
    pub(crate) payment_method_type: String,
    pub(crate) payload: PaymentPayload,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentPayload {
    #[serde(rename = "paymentAccountId")]
    pub(crate) payment_account_id: String,
    pub(crate) cvv: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetQuote {
    #[serde(rename = "licensePlate")]
    pub(crate) license_plate: String,
    #[serde(rename = "locationId")]
    pub(crate) location_id: String,
    #[serde(rename = "rateOptionId")]
    pub(crate) rate_option_id: String,
    #[serde(rename = "durationQuantity")]
    pub(crate) duration_quantity: u16,
    #[serde(rename = "timeUnit")]
    pub(crate) time_unit: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetRateOptions {
    #[serde(rename = "locationId")]
    pub(crate) location_id: String,
    #[serde(rename = "licensePlate")]
    pub(crate) license_plate: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetParkingSession {
    #[serde(rename = "periodType")]
    pub(crate) period_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Auth {
    pub(crate) token_type: String,
    pub(crate) access_token: String,
    expires_in: i32,
    refresh_token: String,
    scope: String,
}

#[derive(Deserialize)]
pub(crate) struct Account {
    pub(crate) id: String,
}
