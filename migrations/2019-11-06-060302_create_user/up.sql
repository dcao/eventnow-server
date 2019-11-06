-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR NOT NULL,
  fullname VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  profile_img VARCHAR
)
