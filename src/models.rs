use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Hotel {
    pub id: Option<String>,
    pub name: String,
    pub location: String,
    pub stars: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Room {
    pub id: Option<String>,
    pub hotel_id: String,
    pub room_type: String,
    pub price: f64,
    pub status: String, // "available" / "occupied"
}

#[derive(Serialize, Deserialize)]
pub struct Guest {
    pub id: Option<String>,
    pub name: String,
    pub phone: String,
    pub email: String,
}


#[derive(Serialize, Deserialize)]
pub struct Booking {
    pub id: Option<String>,
    pub guest_id: String,
    pub room_id: String,
    pub hotel_id: String,
    pub check_in: String,
    pub check_out: String,
}


#[derive(Serialize, Deserialize)]
pub struct Payment {
    pub id: Option<String>,
    pub booking_id: String,
    pub amount: f64,
    pub method: String,
}
