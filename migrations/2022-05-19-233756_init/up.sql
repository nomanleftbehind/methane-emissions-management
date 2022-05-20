CREATE TABLE thermostat_status (
    id SERIAL PRIMARY KEY,
    status BOOLEAN NOT NULL,
    timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO
    thermostat_status(status)
VALUES
    (false);

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "user" (
    "id" uuid DEFAULT uuid_generate_v4 () NOT NULL PRIMARY KEY,
    "first_name" TEXT NOT NULL,
    "last_name" TEXT NOT NULL,
    "email" TEXT NOT NULL,
    "password" TEXT NOT NULL,
    "role" TEXT NOT NULL DEFAULT 'Operator'
);

CREATE TABLE license_change (
    "id" uuid DEFAULT uuid_generate_v4 () NOT NULL PRIMARY KEY,
    "status" TEXT NOT NULL DEFAULT 'Operating',
    "substance" TEXT NOT NULL DEFAULT 'Oil Well Effluent',
    "date" TIMESTAMP(3) NOT NULL,
    "comment" TEXT,
    "link_to_documentation" TEXT,
    "created_by_id" uuid REFERENCES "user"(id) ON DELETE RESTRICT ON UPDATE CASCADE NOT NULL,
    "created_at" TIMESTAMP NOT NULL DEFAULT NOW(),
    "updated_by_id" uuid REFERENCES "user"(id) ON DELETE RESTRICT ON UPDATE CASCADE NOT NULL
);

INSERT INTO
    "user" (
        "first_name",
        "last_name",
        "email",
        "password",
        "role"
    )
VALUES
    (
        'John',
        'Smith',
        'john.smith@example.com',
        '2222',
        'Engineeer'
    ),
    (
        'Jane',
        'Smith',
        'jane.smith@example.com',
        '0000',
        'Consultant'
    ),
    (
        'Alex',
        'Smith',
        'alex.smith@example.com',
        '1111',
        'Office'
    );

WITH license_change_ins (
    "status",
    "substance",
    "date",
    "comment",
    "first_name"
) AS (
    VALUES
        (
            'Operated',
            'Water',
            '2016-03-02 12:05:00',
            'first entry',
            'John'
        ),
        (
            'Discontinued',
            'Oil',
            '2018-12-16 00:00:00',
            'second entry',
            'Alex'
        ),
        (
            'Active',
            'Salt Water',
            '2022-04-29 01:13:05',
            'third entry',
            'Jane'
        )
)
INSERT INTO
    license_change (
        "status",
        "substance",
        "date",
        "comment",
        "created_by_id",
        "updated_by_id"
    )
SELECT
    i."status",
    i."substance",
    CAST(i."date" AS timestamp without time zone),
    i."comment",
    u.id,
    u.id
FROM
    "user" u
    JOIN license_change_ins i ON i.first_name = u.first_name;