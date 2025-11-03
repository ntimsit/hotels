use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use rusqlite::Connection;
use serde_json::json;
use uuid::Uuid;
use crate::models::{Hotel, Room, Guest, Booking, Payment};

//---Hotels---

//creates an hotel 
#[post("/hotels")]
async fn create_hotel(data: web::Json<Hotel>) -> impl Responder {
    let conn = Connection::open("hotel.db").unwrap();
    let id = Uuid::new_v4().to_string();

    conn.execute(
        "INSERT INTO hotels (id, name, location, stars) VALUES (?1, ?2, ?3, ?4)",
        (&id, &data.name, &data.location, &data.stars),
    ).unwrap();

    HttpResponse::Ok().json(json!({
        "status": "hotel added",
        "id": id
    }))
}

//returns all hotels in DB
#[get("/hotels")]
async fn get_hotels() -> impl Responder {
    let conn = Connection::open("hotel.db").unwrap();
    let mut stmt = conn.prepare("SELECT id, name, location, stars FROM hotels").unwrap();
    let hotels_iter = stmt
        .query_map([], |row| {
            Ok(Hotel {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                location: row.get(2)?,
                stars: row.get(3)?,
            })
        }).unwrap();

    let hotels: Vec<Hotel> = hotels_iter.map(|h| h.unwrap()).collect();
    HttpResponse::Ok().json(hotels)
}

//return hotel by ID
#[get("/hotels/{id}")]
async fn get_hotel_by_id(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    let conn = Connection::open("hotel.db").unwrap();
    let mut stmt = conn.prepare("SELECT id, name, location, stars FROM hotels WHERE id = ?1").unwrap();
    let hotel = stmt.query_row([id], |row| {
        Ok(Hotel {
            id: Some(row.get(0)?),
            name: row.get(1)?,
            location: row.get(2)?,
            stars: row.get(3)?,
        })
    });

    match hotel {
        Ok(h) => HttpResponse::Ok().json(h),
        Err(_) => HttpResponse::NotFound().json(json!({"error": "hotel not found"})),
    }
}

//returns highest rated hotel in DB
#[get("/hotels/highest-rated")]
async fn get_highest_rated_hotel() -> impl Responder {
    println!("🔥 get_highest_rated_hotel called!");

    let conn = Connection::open("hotel.db").unwrap();

    let mut stmt = conn
        .prepare("SELECT id, name, location, stars FROM hotels ORDER BY stars DESC LIMIT 1")
        .unwrap();

    let result = stmt.query_map([], |row| {
        Ok(Hotel {
            id: Some(row.get(0)?),
            name: row.get(1)?,
            location: row.get(2)?,
            stars: row.get(3)?,
        })
    }).unwrap();

    let hotels: Vec<Hotel> = result.map(|h| h.unwrap()).collect();

    if let Some(top_hotel) = hotels.first() {
        HttpResponse::Ok().json(top_hotel)
    } else {
        HttpResponse::Ok().json(json!({"message": "No hotels found"}))
    }
}


//updetes an hotel by a certain ID
#[put("/hotels/{id}")]
async fn update_hotel(path: web::Path<String>, data: web::Json<Hotel>) -> impl Responder {
    let id = path.into_inner();
    let conn = Connection::open("hotel.db").unwrap();
    conn.execute(
        "UPDATE hotels SET name = ?1, location = ?2, stars = ?3 WHERE id = ?4",
        (&data.name, &data.location, &data.stars, id),
    ).unwrap();
    HttpResponse::Ok().json(json!({"status": "hotel updated"}))
}

//deletes an hotel by ID from DB
#[delete("/hotels/{id}")]
async fn delete_hotel(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    let conn = Connection::open("hotel.db").unwrap();
    conn.execute("DELETE FROM hotels WHERE id = ?1", [&id]).unwrap();
    HttpResponse::Ok().json(json!({"status": "hotel deleted"}))
}

//---rooms---

//creates a room in DB
#[post("/rooms")]
async fn create_room(data: web::Json<Room>) -> impl Responder {
    let conn = Connection::open("hotel.db").unwrap();
    let id = Uuid::new_v4().to_string();

    conn.execute(
        "INSERT INTO rooms (id, hotel_id, room_type, price, status)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        (&id, &data.hotel_id, &data.room_type, &data.price, &data.status),
    ).unwrap();

    HttpResponse::Ok().json(json!({"status": "room added", "id": id}))
}


//returns all rooms in DB
#[get("/rooms")]
async fn get_rooms() -> impl Responder {
    let conn = Connection::open("hotel.db").unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, hotel_id, room_type, price, status FROM rooms"
    ).unwrap();

    let rooms_iter = stmt.query_map([], |row| {
        Ok(Room {
            id: Some(row.get(0)?),
            hotel_id: row.get(1)?,
            room_type: row.get(2)?,
            price: row.get(3)?,
            status: row.get(4)?,
        })
    }).unwrap();

    let rooms: Vec<Room> = rooms_iter.map(|r| r.unwrap()).collect();
    HttpResponse::Ok().json(rooms)
}


//returns a room by ID
#[get("/rooms/{id}")]
async fn get_room_by_id(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    let conn = Connection::open("hotel.db").unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, hotel_id, room_type, price, status FROM rooms WHERE id = ?1"
    ).unwrap();

    let room = stmt.query_row([id], |row| {
        Ok(Room {
            id: Some(row.get(0)?),
            hotel_id: row.get(1)?,
            room_type: row.get(2)?,
            price: row.get(3)?,
            status: row.get(4)?,
        })
    });

    match room {
        Ok(r) => HttpResponse::Ok().json(r),
        Err(_) => HttpResponse::NotFound().json(json!({"error": "room not found"})),
    }
}

//updates a certain room by ID
#[put("/rooms/{id}")]
async fn update_room(path: web::Path<String>, data: web::Json<Room>) -> impl Responder {
    let id = path.into_inner();
    let conn = Connection::open("hotel.db").unwrap();

    conn.execute(
        "UPDATE rooms SET hotel_id = ?1, room_type = ?2, price = ?3, status = ?4 WHERE id = ?5",
        (&data.hotel_id, &data.room_type, &data.price, &data.status, &id),
    ).unwrap();

    HttpResponse::Ok().json(json!({"status": "room updated"}))
}

//deletes a room by ID
#[delete("/rooms/{id}")]
async fn delete_room(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    let conn = Connection::open("hotel.db").unwrap();

    conn.execute("DELETE FROM rooms WHERE id = ?1", [&id]).unwrap();
    HttpResponse::Ok().json(json!({"status": "room deleted"}))
}


//returns number of available room
#[get("/rooms/available/count")]
async fn count_available_rooms() -> impl Responder {
    let conn = Connection::open("hotel.db").unwrap();
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM rooms WHERE status = 'available'").unwrap();
    let count: i64 = stmt.query_row([], |row| row.get(0)).unwrap();
    HttpResponse::Ok().json(json!({ "available_rooms": count }))
}

//---guests---


//creates a guest in DB
#[post("/guests")]
async fn create_guest(data: web::Json<Guest>) -> impl Responder {
    let conn = Connection::open("hotel.db").unwrap();
    let id = Uuid::new_v4().to_string();

    conn.execute(
        "INSERT INTO guests (id, name, phone, email)
         VALUES (?1, ?2, ?3, ?4)",
        (&id, &data.name, &data.phone, &data.email),
    ).unwrap();

    HttpResponse::Ok().json(json!({"status": "guest added", "id": id}))
}

//returns guests in DB
#[get("/guests")]
async fn get_guests() -> impl Responder {
    let conn = Connection::open("hotel.db").unwrap();
    let mut stmt = conn.prepare("SELECT id, name, phone, email FROM guests").unwrap();

    let guests_iter = stmt.query_map([], |row| {
        Ok(Guest {
            id: Some(row.get(0)?),
            name: row.get(1)?,
            phone: row.get(2)?,
            email: row.get(3)?,
        })
    }).unwrap();

    let guests: Vec<Guest> = guests_iter.map(|g| g.unwrap()).collect();
    HttpResponse::Ok().json(guests)
}

//returns a guest by ID
#[get("/guests/{id}")]
async fn get_guest_by_id(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    let conn = Connection::open("hotel.db").unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, name, phone, email FROM guests WHERE id = ?1"
    ).unwrap();

    let guest = stmt.query_row([id], |row| {
        Ok(Guest {
            id: Some(row.get(0)?),
            name: row.get(1)?,
            phone: row.get(2)?,
            email: row.get(3)?,
        })
    });

    match guest {
        Ok(g) => HttpResponse::Ok().json(g),
        Err(_) => HttpResponse::NotFound().json(json!({"error": "guest not found"})),
    }
}

//updates a guest by ID
#[put("/guests/{id}")]
async fn update_guest(path: web::Path<String>, data: web::Json<Guest>) -> impl Responder {
    let id = path.into_inner();
    let conn = Connection::open("hotel.db").unwrap();

    conn.execute(
        "UPDATE guests SET name = ?1, phone = ?2, email = ?3 WHERE id = ?4",
        (&data.name, &data.phone, &data.email, &id),
    ).unwrap();

    HttpResponse::Ok().json(json!({"status": "guest updated"}))
}

//deletes a guest by ID
#[delete("/guests/{id}")]
async fn delete_guest(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    let conn = Connection::open("hotel.db").unwrap();

    conn.execute("DELETE FROM guests WHERE id = ?1", [&id]).unwrap();
    HttpResponse::Ok().json(json!({"status": "guest deleted"}))
}


//return guest with most bookings
#[get("/guests/top")]
async fn get_guest_with_most_bookings() -> impl Responder {
    let conn = Connection::open("hotel.db").unwrap();

    let sql = "
        SELECT g.id, g.name, COUNT(b.id) AS total_bookings
        FROM guests g
        LEFT JOIN bookings b ON g.id = b.guest_id
        GROUP BY g.id
        ORDER BY total_bookings DESC
        LIMIT 1
    ";

    let mut stmt = conn.prepare(sql).unwrap();

    let result = stmt.query_row([], |row| {
        Ok(json!({
            "id": row.get::<_, String>(0)?,
            "name": row.get::<_, String>(1)?,
            "total_bookings": row.get::<_, i64>(2)?
        }))
    });

    match result {
        Ok(guest) => HttpResponse::Ok().json(guest),
        Err(_) => HttpResponse::Ok().json(json!({"message": "no guests found"})),
    }
}


//---bookings---

//creates a booking in DB
#[post("/bookings")]
async fn create_booking(data: web::Json<Booking>) -> impl Responder {
    let conn = Connection::open("hotel.db").unwrap();
    let id = Uuid::new_v4().to_string();

    conn.execute(
        "INSERT INTO bookings (id, guest_id, room_id, hotel_id, check_in, check_out)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        (&id, &data.guest_id, &data.room_id, &data.hotel_id, &data.check_in, &data.check_out),
    ).unwrap();

    HttpResponse::Ok().json(json!({"status": "booking added", "id": id}))
}

//returns all bookings in DB
#[get("/bookings")]
async fn get_bookings() -> impl Responder {
    let conn = Connection::open("hotel.db").unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, guest_id, room_id, hotel_id, check_in, check_out FROM bookings"
    ).unwrap();

    let bookings_iter = stmt.query_map([], |row| {
        Ok(Booking {
            id: Some(row.get(0)?),
            guest_id: row.get(1)?,
            room_id: row.get(2)?,
            hotel_id: row.get(3)?,
            check_in: row.get(4)?,
            check_out: row.get(5)?,
        })
    }).unwrap();

    let bookings: Vec<Booking> = bookings_iter.map(|b| b.unwrap()).collect();
    HttpResponse::Ok().json(bookings)
}

//returns a booking by ID
#[get("/bookings/{id}")]
async fn get_booking_by_id(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    let conn = Connection::open("hotel.db").unwrap();
    let mut stmt = conn.prepare(
        "SELECT id, guest_id, room_id, hotel_id, check_in, check_out FROM bookings WHERE id = ?1"
    ).unwrap();

    let booking = stmt.query_row([id], |row| {
        Ok(Booking {
            id: Some(row.get(0)?),
            guest_id: row.get(1)?,
            room_id: row.get(2)?,
            hotel_id: row.get(3)?,
            check_in: row.get(4)?,
            check_out: row.get(5)?,
        })
    });

    match booking {
        Ok(b) => HttpResponse::Ok().json(b),
        Err(_) => HttpResponse::NotFound().json(json!({"error": "booking not found"})),
    }
}

//updates a booking by ID
#[put("/bookings/{id}")]
async fn update_booking(path: web::Path<String>, data: web::Json<Booking>) -> impl Responder {
    let id = path.into_inner();
    let conn = Connection::open("hotel.db").unwrap();

    conn.execute(
        "UPDATE bookings SET guest_id = ?1, room_id = ?2, hotel_id = ?3, check_in = ?4, check_out = ?5 WHERE id = ?6",
        (&data.guest_id, &data.room_id, &data.hotel_id, &data.check_in, &data.check_out, &id),
    ).unwrap();

    HttpResponse::Ok().json(json!({"status": "booking updated"}))
}


//deletes a booking by ID
#[delete("/bookings/{id}")]
async fn delete_booking(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    let conn = Connection::open("hotel.db").unwrap();

    conn.execute("DELETE FROM bookings WHERE id = ?1", [&id]).unwrap();
    HttpResponse::Ok().json(json!({"status": "booking deleted"}))
}


//returns average stay duration (in days)
#[get("/analytics/bookings/average_stay")]
async fn get_average_stay_duration() -> impl Responder {
    let conn = Connection::open("hotel.db").unwrap();
    let mut stmt = conn.prepare(
        "SELECT AVG(julianday(check_out) - julianday(check_in)) AS avg_stay FROM bookings"
    ).unwrap();

    let avg_stay: Option<f64> = stmt.query_row([], |row| row.get(0)).ok();

    HttpResponse::Ok().json(json!({
        "average_stay_days": avg_stay.unwrap_or(0.0)
    }))
}


//returns the last hotel (or current) a guest stayed at
#[get("/analytics/bookings/guest/{guest_id}/current_or_last_hotel")]
async fn get_current_or_last_hotel_by_guest(path: web::Path<String>) -> impl Responder {
    let guest_id = path.into_inner();
    let conn = Connection::open("hotel.db").unwrap();

    let sql = "
        SELECT h.id, h.name, h.location, h.stars
        FROM bookings b
        JOIN hotels h ON b.hotel_id = h.id
        WHERE b.guest_id = ?1
        ORDER BY
            CASE
                WHEN DATE('now') BETWEEN b.check_in AND b.check_out THEN 0
                ELSE 1
            END,
            b.check_out DESC
        LIMIT 1
    ";

    let mut stmt = conn.prepare(sql).unwrap();
    let hotel = stmt.query_row([guest_id], |row| {
        Ok(json!({
            "id": row.get::<_, String>(0)?,
            "name": row.get::<_, String>(1)?,
            "location": row.get::<_, String>(2)?,
            "stars": row.get::<_, i32>(3)?
        }))
    });

    match hotel {
        Ok(h) => HttpResponse::Ok().json(h),
        Err(_) => HttpResponse::Ok().json(json!({"message": "no current or previous hotel found"})),
    }
}

//---payments---
//creates a payment in DB
#[post("/payments")]
async fn create_payment(data: web::Json<Payment>) -> impl Responder {
    let conn = Connection::open("hotel.db").unwrap();
    let id = Uuid::new_v4().to_string();

    conn.execute(
        "INSERT INTO payments (id, booking_id, amount, method)
         VALUES (?1, ?2, ?3, ?4)",
        (&id, &data.booking_id, &data.amount, &data.method),
    ).unwrap();

    HttpResponse::Ok().json(json!({"status": "payment added", "id": id}))
}

//returns all payments in DB
#[get("/payments")]
async fn get_payments() -> impl Responder {
    let conn = Connection::open("hotel.db").unwrap();
    let mut stmt = conn.prepare("SELECT id, booking_id, amount, method FROM payments").unwrap();

    let payments_iter = stmt.query_map([], |row| {
        Ok(Payment {
            id: Some(row.get(0)?),
            booking_id: row.get(1)?,
            amount: row.get(2)?,
            method: row.get(3)?,
        })
    }).unwrap();

    let payments: Vec<Payment> = payments_iter.map(|p| p.unwrap()).collect();
    HttpResponse::Ok().json(payments)
}

//returns a payment by ID
#[get("/payments/{id}")]
async fn get_payment_by_id(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    let conn = Connection::open("hotel.db").unwrap();

    let mut stmt = conn.prepare("SELECT id, booking_id, amount, method FROM payments WHERE id = ?1").unwrap();

    let payment = stmt.query_row([id], |row| {
        Ok(Payment {
            id: Some(row.get(0)?),
            booking_id: row.get(1)?,
            amount: row.get(2)?,
            method: row.get(3)?,
        })
    });

    match payment {
        Ok(p) => HttpResponse::Ok().json(p),
        Err(_) => HttpResponse::NotFound().json(json!({"error": "payment not found"})),
    }
}

//updates a payment by ID
#[put("/payments/{id}")]
async fn update_payment(path: web::Path<String>, data: web::Json<Payment>) -> impl Responder {
    let id = path.into_inner();
    let conn = Connection::open("hotel.db").unwrap();

    conn.execute(
        "UPDATE payments SET booking_id = ?1, amount = ?2, method = ?3 WHERE id = ?4",
        (&data.booking_id, &data.amount, &data.method, &id),
    ).unwrap();

    HttpResponse::Ok().json(json!({"status": "payment updated"}))
}

//deletes a payment by id
#[delete("/payments/{id}")]
async fn delete_payment(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    let conn = Connection::open("hotel.db").unwrap();

    conn.execute("DELETE FROM payments WHERE id = ?1", [&id]).unwrap();
    HttpResponse::Ok().json(json!({"status": "payment deleted"}))
}

//returns total payments per booking
#[get("/analytics/payments/total_per_booking")]
async fn get_total_paid_per_booking() -> impl Responder {
    let conn = Connection::open("hotel.db").unwrap();

    let mut stmt = conn.prepare(
        "SELECT booking_id, SUM(amount) AS total_paid FROM payments GROUP BY booking_id"
    ).unwrap();

    let mut rows = stmt.query([]).unwrap();
    let mut result = Vec::new();

    while let Some(row) = rows.next().unwrap() {
        result.push(json!({
            "booking_id": row.get::<_, String>(0).unwrap(),
            "total_paid": row.get::<_, f64>(1).unwrap(),
        }));
    }

    HttpResponse::Ok().json(result)
}





pub fn config(cfg: &mut web::ServiceConfig) {
    //hotels
    cfg.service(create_hotel)
       .service(get_hotels)
       .service(get_highest_rated_hotel)
       .service(get_hotel_by_id)
       .service(update_hotel)
       .service(delete_hotel)
       
       // Rooms
        .service(create_room)
        .service(get_rooms)
        .service(get_room_by_id)
        .service(update_room)
        .service(delete_room)
        .service(count_available_rooms)



        // Guests
        .service(create_guest)
        .service(get_guests)
        .service(get_guest_by_id)
        .service(update_guest)
        .service(delete_guest)
        .service(get_guest_with_most_bookings)


        // Bookings
        .service(create_booking)
        .service(get_bookings)
        .service(get_booking_by_id)
        .service(update_booking)
        .service(get_average_stay_duration)
        .service(get_current_or_last_hotel_by_guest)
        .service(delete_booking)


        //payments
        .service(create_payment)
        .service(get_payments)
        .service(get_payment_by_id)
        .service(update_payment)
        .service(delete_payment)
        .service(get_total_paid_per_booking);
       
       
}



