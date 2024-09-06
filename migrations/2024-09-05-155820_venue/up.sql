-- Your SQL goes here

-- Venues
CREATE TABLE Venue (
    VenueID UUID PRIMARY KEY,
    Name TEXT,
    ContactName VARCHAR(255),
    Address TEXT,
    City VARCHAR(100),
    PostalCode VARCHAR(20),
    Country VARCHAR(100),
    Phone VARCHAR(50),
    Latitude Int,
    Longitude Int
)

