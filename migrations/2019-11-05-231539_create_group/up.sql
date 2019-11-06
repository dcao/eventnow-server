-- Your SQL goes here
CREATE TABLE groups (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  description VARCHAR,
  profile_img VARCHAR,
  cover_img VARCHAR
)
