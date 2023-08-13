#!/bin/sh
echo "\
application:\n\
  port: 8081\n\
  host: 0.0.0.0\n\
  hmac_secret: \"super-long-and-secret-random-key-needed-to-verify-message-integrity\"\n\
  session_cookie_name: \"emissionsapp_session\"\n\
database:\n\
  host: \""${POSTGRES_HOST}\""\n\
  port: 5432\n\
  username: \""${POSTGRES_USER}\""\n\
  password: \""${POSTGRES_PASSWORD}\""\n\
  database_name: \"emissions\"\n\
  require_ssl: false\n\
fdc_database:\n\
  host: \"localhost\"\n\
  port: 1433\n\
  database: \"test\"\n\
  username: \"username\"\n\
  password: \"password\"\n\
  trust_certificate: true # on production, it is not a good idea to do this\n\
default_gas_params:\n\
  c1: 0.82\n\
  co2: 0.0067\n\
  gas_gravity: 0.7\n\
email_client:\n\
  base_url: \"localhost\"\n\
  sender_email: \"test@gmail.com\"\n\
  authorization_token: \"my-secret-token\"\n\
  timeout_milliseconds: 10000\n\
redis_uri: \"redis://127.0.0.1:6379\"" > configuration/base.yaml

./methane_emissions_management_server --dir client/dist