-- Your SQL goes here
CREATE TABLE memberships (
  id SERIAL PRIMARY KEY,
  user_id INTEGER REFERENCES users (id) NOT NULL,
  group_id INTEGER REFERENCES groups (id) NOT NULL,
  membership_role VARCHAR NOT NULL
)
