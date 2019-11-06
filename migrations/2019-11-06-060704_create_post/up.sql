-- Your SQL goes here
CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  title VARCHAR,
  body VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL,
  user_id INTEGER REFERENCES users (id) NOT NULL,
  group_id INTEGER REFERENCES groups (id) NOT NULL,
  event_id INTEGER REFERENCES events (id)
)
