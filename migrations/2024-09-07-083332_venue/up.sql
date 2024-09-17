-- Your SQL goes here

-- Venues
CREATE TABLE Venue (
    Id UUID,
    Name TEXT,
    ContactName VARCHAR(255),
    Address TEXT,
    City VARCHAR(100),
    PostalCode VARCHAR(20),
    Country VARCHAR(100),
    Phone VARCHAR(50),
    Latitude VARCHAR(20),
    Longitude VARCHAR(20),

    PRIMARY KEY(Id)
)

