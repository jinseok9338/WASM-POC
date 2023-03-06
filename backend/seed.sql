CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255),
  phone_number BIGINT,
  created_at TIMESTAMP,
  updated_at TIMESTAMP,
  profile_image_url VARCHAR(255),
  tester BOOLEAN,
  last_signed_in TIMESTAMP
);

INSERT INTO users (name, phone_number, created_at, updated_at, profile_image_url, tester, last_signed_in)
VALUES ('Jinseok Seo', 1034213305, NOW(), NOW(), 'https://www.peanuts.com/sites/default/files/sn-color.jpg', true, null);