-- Your SQL goes here
CREATE TABLE events (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  description VARCHAR,
  cover_img VARCHAR,
  address VARCHAR NOT NULL,
  loc_name VARCHAR,
  lat DOUBLE PRECISION NOT NULL,
  long DOUBLE PRECISION NOT NULL
)
