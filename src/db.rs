use rusqlite::{Connection, Result};

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("hotel.db")?;

    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS hotels (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            location TEXT,
            stars INTEGER
        );

        CREATE TABLE IF NOT EXISTS rooms (
            id TEXT PRIMARY KEY,
            hotel_id TEXT NOT NULL,
            room_type TEXT,
            price REAL,
            status TEXT,
            FOREIGN KEY(hotel_id) REFERENCES hotels(id)
        );

        CREATE TABLE IF NOT EXISTS guests (
            id TEXT PRIMARY KEY,
            name TEXT,
            phone TEXT,
            email TEXT
        );

        CREATE TABLE IF NOT EXISTS bookings (
            id TEXT PRIMARY KEY,
            guest_id TEXT,
            room_id TEXT,
            hotel_id TEXT,
            check_in DATE,
            check_out DATE,
            FOREIGN KEY(guest_id) REFERENCES guests(id),
            FOREIGN KEY(room_id) REFERENCES rooms(id),
            FOREIGN KEY(hotel_id) REFERENCES hotels(id)
        );

        CREATE TABLE IF NOT EXISTS payments (
            id TEXT PRIMARY KEY,
            booking_id TEXT,
            amount REAL,
            method TEXT,
            FOREIGN KEY(booking_id) REFERENCES bookings(id)
        );
        "
    )?;

    Ok(conn)
}
