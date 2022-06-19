CREATE TABLE IF NOT EXISTS "valid_roles" ("role" VARCHAR(64) PRIMARY KEY);

INSERT INTO
  "valid_roles" ("role")
VALUES
  ('ADMIN'),
  ('ENGINEER'),
  ('USER');

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS "users" (
  "id" uuid DEFAULT uuid_generate_v4 () NOT NULL PRIMARY KEY,
  "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  -- 
  "first_name" VARCHAR(50) NOT NULL,
  "last_name" VARCHAR(50) NOT NULL,
  "username" VARCHAR(30) NOT NULL UNIQUE,
  "location" VARCHAR(180) DEFAULT NULL,
  "email" VARCHAR(128) NOT NULL,
  "hash" VARCHAR(122) NOT NULL,
  "role" VARCHAR(64) REFERENCES "valid_roles" ("role") ON DELETE RESTRICT ON UPDATE CASCADE DEFAULT 'USER' NOT NULL
);

CREATE UNIQUE INDEX "users_email_index" ON "users" ("email");

CREATE TABLE IF NOT EXISTS "controllers" (
  "id" uuid DEFAULT uuid_generate_v4 () NOT NULL PRIMARY KEY,
  "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "updated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  -- "created_by_id" SERIAL REFERENCES "users" ("id") ON DELETE RESTRICT ON UPDATE CASCADE NOT NULL,
  "created_by_id" uuid REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE NOT NULL,
  /*
   Diesel won't generate joinable! macro if foreign key is referenced twice.
   That's why we commented out 'updated_by' field for now.
   */
  -- "updated_by" SERIAL REFERENCES users (id) ON UPDATE CASCADE NOT NULL,
  "manufacturer" VARCHAR(50),
  "model" VARCHAR(50),
  "serial_number" VARCHAR(50),
  "function" VARCHAR(50)
);

CREATE UNIQUE INDEX "controllers_serial_number_index" ON "controllers" ("serial_number");

INSERT INTO
  "users" (
    "first_name",
    "last_name",
    "username",
    "location",
    "email",
    "hash",
    "role"
  )
VALUES
  (
    'John',
    'Smith',
    'johnny',
    'Indianapolis',
    'john.smith@example.com',
    --hash for actual password 1234
    '$argon2id$v=19$m=4096,t=192,p=4$ZXQuqZnXGKcoV+XLm0hvi9UBY7EEZ1hvFeQgilwCzHc$kYftFsEAWi1iIuW3yAtGvfvvFsDNdEOn4hIkKkXr8RU',
    'ADMIN'
  ),
  (
    'Jane',
    'Smith',
    'janny',
    'Moab',
    'jane.smith@example.com',
    --hash for actual password 0000
    '$argon2id$v=19$m=4096,t=192,p=4$neIneMlNSrmTftaZh4HeFsczKUvX9kMYYZxeaIUZudE$y3erK8FrYskHVGiR7++VuZzwjLXLdLCsFOjrREs6ScA',
    'USER'
  ),
  (
    'Alex',
    'Smith',
    'lex',
    'Idaho Falls',
    'alex.smith@example.com',
    --hash for actual password 1212
    '$argon2id$v=19$m=4096,t=192,p=4$wpE8nmwz3NdSWl2R7gNCvd+6Xv26/pO20K4CBqK3hGQ$A69ioT1OB/6cEz99WVqSy38EPBpvTBCACouF3w+rKRY',
    'ENGINEER'
  );

WITH "controllers_ins" (
  "manufacturer",
  "model",
  "serial_number",
  "function",
  "email"
) AS (
  VALUES
    (
      'Fisher',
      'L2',
      '20295960',
      'level controller',
      'alex.smith@example.com'
    ),
    (
      'Bruin',
      '5100 3/8"',
      '51-10234',
      'chemical pump',
      'john.smith@example.com'
    ),
    (
      'Norriseal',
      '1001A',
      '726141',
      'pressure controller',
      'jane.smith@example.com'
    )
)
INSERT INTO
  "controllers" (
    "manufacturer",
    "model",
    "serial_number",
    "function",
    "created_by_id"
  )
SELECT
  i."manufacturer",
  i."model",
  i."serial_number",
  i."function",
  -- CAST(i."date" AS timestamp without time zone),
  u.id
FROM
  "users" u
  JOIN controllers_ins i ON i.email = u.email;