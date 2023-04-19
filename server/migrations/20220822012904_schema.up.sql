-- CreateEnum
CREATE TYPE "user_role" AS ENUM ('ADMIN', 'ENGINEER', 'REGULATORY', 'OFFICE', 'OPERATOR');

-- CreateEnum
CREATE TYPE "facility_type" AS ENUM ('TM', 'WT', 'CT', 'DS', 'GS', 'MS', 'GP', 'IF', 'PL', 'WP', 'WS', 'BT');

-- CreateEnum
CREATE TYPE "site_type" AS ENUM ('Battery', 'Satellite', 'Well', 'GasPlant', 'Compressor');

-- CreateEnum
CREATE TYPE "methane_emission_source" AS ENUM ('pneumatic_instrument', 'pneumatic_pump', 'compressor_seal', 'tank');

-- CreateEnum
CREATE TYPE "methane_emission_category" AS ENUM ('Routine', 'Nonroutine', 'Fugitive');

-- CreateEnum
CREATE TYPE "compressor_type" AS ENUM ('Reciprocating', 'Centrifugal', 'Screw', 'Scroll');

-- CreateEnum
CREATE TYPE "seal_type" AS ENUM ('Rodpacking', 'Dry', 'Wet');

-- CreateEnum
CREATE TYPE "calculation_method" AS ENUM ('EQUATION', 'MEASURED');

-- CreateTable
CREATE TABLE "users" (
    "id" UUID NOT NULL,
    "email" TEXT NOT NULL,
    "password" TEXT NOT NULL,
    "first_name" TEXT NOT NULL,
    "last_name" TEXT NOT NULL,
    "role" "user_role" NOT NULL DEFAULT 'OPERATOR',

    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "facilities" (
    "id" UUID NOT NULL,
    "idpa" VARCHAR(12) NOT NULL,
    "name" TEXT NOT NULL,
    "type" "facility_type" NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "facilities_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "site" (
    "id" UUID NOT NULL,
    "facility_id" UUID NOT NULL,
    "fdc_rec_id" VARCHAR(32) NOT NULL,
    "name" TEXT NOT NULL,
    "type" "site_type" NOT NULL,
    "description" TEXT,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "site_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "pneumatic_instrument_type" (
    "id" UUID NOT NULL,
    "type" TEXT NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "pneumatic_instrument_type_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "device_manufacturer" (
    "id" UUID NOT NULL,
    "manufacturer" TEXT NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "device_manufacturer_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "pneumatic_instrument" (
    "id" UUID NOT NULL,
    "site_id" UUID NOT NULL,
    "type_id" UUID NOT NULL,
    "manufacturer_id" UUID NOT NULL,
    "model" TEXT,
    "serial_number" TEXT,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "pneumatic_instrument_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "pneumatic_instrument_change" (
    "id" UUID NOT NULL,
    "pneumatic_instrument_id" UUID NOT NULL,
    "date" DATE NOT NULL,
    "rate" DOUBLE PRECISION NOT NULL,
    "actuation_frequency" DOUBLE PRECISION NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "pneumatic_instrument_change_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "pneumatic_instrument_month_hours" (
    "id" UUID NOT NULL,
    "pneumatic_instrument_id" UUID NOT NULL,
    "month" DATE NOT NULL,
    "hours_on" DOUBLE PRECISION NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "pneumatic_instrument_month_hours_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "pneumatic_instrument_month_methane_emission_override" (
    "id" UUID NOT NULL,
    "pneumatic_instrument_id" UUID NOT NULL,
    "month" DATE NOT NULL,
    "gas_volume" DOUBLE PRECISION NOT NULL,
    "comment" TEXT,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "pneumatic_instrument_month_methane_emission_override_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "pneumatic_instrument_month_methane_emission" (
    "id" UUID NOT NULL,
    "source" "methane_emission_source" NOT NULL,
    "source_id" UUID NOT NULL,
    "category" "methane_emission_category" NOT NULL,
    "month" DATE NOT NULL,
    "gas_volume" DOUBLE PRECISION NOT NULL,
    "c1_volume" DOUBLE PRECISION NOT NULL,
    "co2_volume" DOUBLE PRECISION NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "pneumatic_instrument_month_methane_emission_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "pneumatic_pump" (
    "id" UUID NOT NULL,
    "site_id" UUID NOT NULL,
    "manufacturer_id" UUID NOT NULL,
    "model" TEXT,
    "serial_number" TEXT,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "pneumatic_pump_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "pneumatic_pump_change" (
    "id" UUID NOT NULL,
    "pneumatic_pump_id" UUID NOT NULL,
    "date" DATE NOT NULL,
    "rate" DOUBLE PRECISION NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "pneumatic_pump_change_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "pneumatic_pump_month_hours" (
    "id" UUID NOT NULL,
    "pneumatic_pump_id" UUID NOT NULL,
    "month" DATE NOT NULL,
    "hours_on" DOUBLE PRECISION NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "pneumatic_pump_month_hours_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "pneumatic_pump_month_methane_emission_override" (
    "id" UUID NOT NULL,
    "pneumatic_pump_id" UUID NOT NULL,
    "month" DATE NOT NULL,
    "gas_volume" DOUBLE PRECISION NOT NULL,
    "comment" TEXT,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "pneumatic_pump_month_methane_emission_override_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "compressor" (
    "id" UUID NOT NULL,
    "site_id" UUID NOT NULL,
    "fdc_rec_id" VARCHAR(32) NOT NULL,
    "name" TEXT NOT NULL,
    "serial_number" TEXT NOT NULL,
    "power" DOUBLE PRECISION NOT NULL,
    "throw_count" INTEGER,
    "install_date" DATE NOT NULL,
    "remove_date" DATE,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "compressor_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "compressor_seal" (
    "id" UUID NOT NULL,
    "controlled" BOOLEAN NOT NULL,
    "description" TEXT,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "compressor_seal_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "compressor_seal_test" (
    "id" UUID NOT NULL,
    "compressor_seal_id" UUID NOT NULL,
    "date" DATE NOT NULL,
    "rate" DOUBLE PRECISION NOT NULL,
    "equipment_used" TEXT NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "compressor_seal_test_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "compressor_month_hours" (
    "id" UUID NOT NULL,
    "compressor_id" UUID NOT NULL,
    "month" DATE NOT NULL,
    "pressurized_hours" DOUBLE PRECISION NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "compressor_month_hours_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "compressor_blowdown" (
    "id" UUID NOT NULL,
    "compressor_id" UUID NOT NULL,
    "date" DATE NOT NULL,
    "gas_volume" DOUBLE PRECISION NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "compressor_blowdown_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "compressor_seal_month_methane_emission_override" (
    "id" UUID NOT NULL,
    "compressor_seal_id" UUID NOT NULL,
    "month" DATE NOT NULL,
    "gas_volume" DOUBLE PRECISION NOT NULL,
    "comment" TEXT,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "compressor_seal_month_methane_emission_override_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "tank_farms" (
    "id" UUID NOT NULL,
    "facility_id" UUID NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "tank_farms_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "tank_farm_changes" (
    "id" UUID NOT NULL,
    "tank_farm_id" UUID NOT NULL,
    "date" DATE NOT NULL,
    "ia" BOOLEAN NOT NULL,
    "vru" BOOLEAN NOT NULL,
    "api_density" DOUBLE PRECISION NOT NULL,
    "temperature" DOUBLE PRECISION NOT NULL,
    "pressure" DOUBLE PRECISION NOT NULL,
    "calculation_method" "calculation_method" NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "tank_farm_changes_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "tank_farm_vent_factors_calculated" (
    "id" UUID NOT NULL,
    "tank_farm_id" UUID NOT NULL,
    "date" DATE NOT NULL,
    "vent_factor" DOUBLE PRECISION NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "tank_farm_vent_factors_calculated_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "tank_farm_month_oil_flow" (
    "id" UUID NOT NULL,
    "tank_farm_id" UUID NOT NULL,
    "month" DATE NOT NULL,
    "oil" DOUBLE PRECISION NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "tank_farm_month_oil_flow_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "tank_farm_month_methane_emission_override" (
    "id" UUID NOT NULL,
    "tank_farm_id" UUID NOT NULL,
    "month" DATE NOT NULL,
    "gas_volume" DOUBLE PRECISION NOT NULL,
    "comment" TEXT,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "tank_farm_month_methane_emission_override_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "gas_analyses" (
    "id" UUID NOT NULL,
    "facility_id" UUID NOT NULL,
    "date" DATE NOT NULL,
    "h2" DOUBLE PRECISION NOT NULL,
    "he" DOUBLE PRECISION NOT NULL,
    "n2" DOUBLE PRECISION NOT NULL,
    "co2" DOUBLE PRECISION NOT NULL,
    "h2s" DOUBLE PRECISION NOT NULL,
    "c1" DOUBLE PRECISION NOT NULL,
    "c2" DOUBLE PRECISION NOT NULL,
    "c3" DOUBLE PRECISION NOT NULL,
    "c4_i" DOUBLE PRECISION NOT NULL,
    "c4_n" DOUBLE PRECISION NOT NULL,
    "c5_i" DOUBLE PRECISION NOT NULL,
    "c5_n" DOUBLE PRECISION NOT NULL,
    "c6" DOUBLE PRECISION NOT NULL,
    "c7_plus" DOUBLE PRECISION NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "gas_analyses_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "gas_analysis_calculated_params" (
    "id" UUID NOT NULL,
    "gas_gravity" DOUBLE PRECISION NOT NULL,
    "higher_heating_value" DOUBLE PRECISION NOT NULL,
    "carbon_content" DOUBLE PRECISION NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "gas_analysis_calculated_params_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "users_email_key" ON "users"("email");

-- CreateIndex
CREATE UNIQUE INDEX "facilities_idpa_key" ON "facilities"("idpa");

-- CreateIndex
CREATE UNIQUE INDEX "site_fdc_rec_id_key" ON "site"("fdc_rec_id");

-- CreateIndex
CREATE UNIQUE INDEX "pneumatic_instrument_type_type_key" ON "pneumatic_instrument_type"("type");

-- CreateIndex
CREATE UNIQUE INDEX "device_manufacturer_manufacturer_key" ON "device_manufacturer"("manufacturer");

-- CreateIndex
CREATE UNIQUE INDEX "pneumatic_instrument_change_pneumatic_instrument_id_date_key" ON "pneumatic_instrument_change"("pneumatic_instrument_id", "date");

-- CreateIndex
CREATE UNIQUE INDEX "pneumatic_instrument_month_hours_pneumatic_instrument_id_mo_key" ON "pneumatic_instrument_month_hours"("pneumatic_instrument_id", "month");

-- CreateIndex
CREATE UNIQUE INDEX "pneumatic_instrument_month_methane_emission_override_pneuma_key" ON "pneumatic_instrument_month_methane_emission_override"("pneumatic_instrument_id", "month");

-- CreateIndex
CREATE UNIQUE INDEX "pneumatic_instrument_month_methane_emission_source_id_categ_key" ON "pneumatic_instrument_month_methane_emission"("source_id", "category", "month");

-- CreateIndex
CREATE UNIQUE INDEX "pneumatic_pump_change_pneumatic_pump_id_date_key" ON "pneumatic_pump_change"("pneumatic_pump_id", "date");

-- CreateIndex
CREATE UNIQUE INDEX "pneumatic_pump_month_hours_pneumatic_pump_id_month_key" ON "pneumatic_pump_month_hours"("pneumatic_pump_id", "month");

-- CreateIndex
CREATE UNIQUE INDEX "pneumatic_pump_month_methane_emission_override_pneumatic_pu_key" ON "pneumatic_pump_month_methane_emission_override"("pneumatic_pump_id", "month");

-- CreateIndex
CREATE UNIQUE INDEX "compressor_fdc_rec_id_key" ON "compressor"("fdc_rec_id");

-- CreateIndex
CREATE UNIQUE INDEX "compressor_serial_number_key" ON "compressor"("serial_number");

-- CreateIndex
CREATE UNIQUE INDEX "compressor_seal_test_compressor_seal_id_date_key" ON "compressor_seal_test"("compressor_seal_id", "date");

-- CreateIndex
CREATE UNIQUE INDEX "compressor_month_hours_compressor_id_month_key" ON "compressor_month_hours"("compressor_id", "month");

-- CreateIndex
CREATE UNIQUE INDEX "compressor_blowdown_compressor_id_date_key" ON "compressor_blowdown"("compressor_id", "date");

-- CreateIndex
CREATE UNIQUE INDEX "compressor_seal_month_methane_emission_override_compressor__key" ON "compressor_seal_month_methane_emission_override"("compressor_seal_id", "month");

-- CreateIndex
CREATE UNIQUE INDEX "tank_farms_facility_id_key" ON "tank_farms"("facility_id");

-- CreateIndex
CREATE UNIQUE INDEX "tank_farm_changes_tank_farm_id_date_key" ON "tank_farm_changes"("tank_farm_id", "date");

-- CreateIndex
CREATE UNIQUE INDEX "tank_farm_vent_factors_calculated_tank_farm_id_date_key" ON "tank_farm_vent_factors_calculated"("tank_farm_id", "date");

-- CreateIndex
CREATE UNIQUE INDEX "tank_farm_month_oil_flow_tank_farm_id_month_key" ON "tank_farm_month_oil_flow"("tank_farm_id", "month");

-- CreateIndex
CREATE UNIQUE INDEX "tank_farm_month_methane_emission_override_tank_farm_id_mont_key" ON "tank_farm_month_methane_emission_override"("tank_farm_id", "month");

-- CreateIndex
CREATE UNIQUE INDEX "gas_analyses_facility_id_date_key" ON "gas_analyses"("facility_id", "date");

-- AddForeignKey
ALTER TABLE "facilities" ADD CONSTRAINT "facilities_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "facilities" ADD CONSTRAINT "facilities_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "site" ADD CONSTRAINT "site_facility_id_fkey" FOREIGN KEY ("facility_id") REFERENCES "facilities"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "site" ADD CONSTRAINT "site_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "site" ADD CONSTRAINT "site_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_instrument_type" ADD CONSTRAINT "pneumatic_instrument_type_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_instrument_type" ADD CONSTRAINT "pneumatic_instrument_type_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "device_manufacturer" ADD CONSTRAINT "device_manufacturer_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "device_manufacturer" ADD CONSTRAINT "device_manufacturer_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_instrument" ADD CONSTRAINT "pneumatic_instrument_site_id_fkey" FOREIGN KEY ("site_id") REFERENCES "site"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_instrument" ADD CONSTRAINT "pneumatic_instrument_type_id_fkey" FOREIGN KEY ("type_id") REFERENCES "pneumatic_instrument_type"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_instrument" ADD CONSTRAINT "pneumatic_instrument_manufacturer_id_fkey" FOREIGN KEY ("manufacturer_id") REFERENCES "device_manufacturer"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_instrument" ADD CONSTRAINT "pneumatic_instrument_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_instrument" ADD CONSTRAINT "pneumatic_instrument_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_instrument_change" ADD CONSTRAINT "pneumatic_instrument_change_pneumatic_instrument_id_fkey" FOREIGN KEY ("pneumatic_instrument_id") REFERENCES "pneumatic_instrument"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_instrument_change" ADD CONSTRAINT "pneumatic_instrument_change_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_instrument_change" ADD CONSTRAINT "pneumatic_instrument_change_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_instrument_month_hours" ADD CONSTRAINT "pneumatic_instrument_month_hours_pneumatic_instrument_id_fkey" FOREIGN KEY ("pneumatic_instrument_id") REFERENCES "pneumatic_instrument"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_instrument_month_hours" ADD CONSTRAINT "pneumatic_instrument_month_hours_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_instrument_month_hours" ADD CONSTRAINT "pneumatic_instrument_month_hours_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_instrument_month_methane_emission_override" ADD CONSTRAINT "pneumatic_instrument_month_methane_emission_override_pneum_fkey" FOREIGN KEY ("pneumatic_instrument_id") REFERENCES "pneumatic_instrument"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_instrument_month_methane_emission_override" ADD CONSTRAINT "pneumatic_instrument_month_methane_emission_override_creat_fkey" FOREIGN KEY ("created_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_instrument_month_methane_emission_override" ADD CONSTRAINT "pneumatic_instrument_month_methane_emission_override_updat_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_instrument_month_methane_emission" ADD CONSTRAINT "pneumatic_instrument_month_methane_emission_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_instrument_month_methane_emission" ADD CONSTRAINT "pneumatic_instrument_month_methane_emission_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_pump" ADD CONSTRAINT "pneumatic_pump_site_id_fkey" FOREIGN KEY ("site_id") REFERENCES "site"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_pump" ADD CONSTRAINT "pneumatic_pump_manufacturer_id_fkey" FOREIGN KEY ("manufacturer_id") REFERENCES "device_manufacturer"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_pump" ADD CONSTRAINT "pneumatic_pump_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_pump" ADD CONSTRAINT "pneumatic_pump_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_pump_change" ADD CONSTRAINT "pneumatic_pump_change_pneumatic_pump_id_fkey" FOREIGN KEY ("pneumatic_pump_id") REFERENCES "pneumatic_pump"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_pump_change" ADD CONSTRAINT "pneumatic_pump_change_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_pump_change" ADD CONSTRAINT "pneumatic_pump_change_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_pump_month_hours" ADD CONSTRAINT "pneumatic_pump_month_hours_pneumatic_pump_id_fkey" FOREIGN KEY ("pneumatic_pump_id") REFERENCES "pneumatic_pump"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_pump_month_hours" ADD CONSTRAINT "pneumatic_pump_month_hours_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_pump_month_hours" ADD CONSTRAINT "pneumatic_pump_month_hours_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_pump_month_methane_emission_override" ADD CONSTRAINT "pneumatic_pump_month_methane_emission_override_pneumatic_p_fkey" FOREIGN KEY ("pneumatic_pump_id") REFERENCES "pneumatic_pump"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_pump_month_methane_emission_override" ADD CONSTRAINT "pneumatic_pump_month_methane_emission_override_created_by__fkey" FOREIGN KEY ("created_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_pump_month_methane_emission_override" ADD CONSTRAINT "pneumatic_pump_month_methane_emission_override_updated_by__fkey" FOREIGN KEY ("updated_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor" ADD CONSTRAINT "compressor_site_id_fkey" FOREIGN KEY ("site_id") REFERENCES "site"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor" ADD CONSTRAINT "compressor_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor" ADD CONSTRAINT "compressor_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_seal" ADD CONSTRAINT "compressor_seal_id_fkey" FOREIGN KEY ("id") REFERENCES "compressor"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_seal" ADD CONSTRAINT "compressor_seal_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_seal" ADD CONSTRAINT "compressor_seal_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_seal_test" ADD CONSTRAINT "compressor_seal_test_compressor_seal_id_fkey" FOREIGN KEY ("compressor_seal_id") REFERENCES "compressor_seal"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_seal_test" ADD CONSTRAINT "compressor_seal_test_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_seal_test" ADD CONSTRAINT "compressor_seal_test_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_month_hours" ADD CONSTRAINT "compressor_month_hours_compressor_id_fkey" FOREIGN KEY ("compressor_id") REFERENCES "compressor"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_month_hours" ADD CONSTRAINT "compressor_month_hours_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_month_hours" ADD CONSTRAINT "compressor_month_hours_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_blowdown" ADD CONSTRAINT "compressor_blowdown_compressor_id_fkey" FOREIGN KEY ("compressor_id") REFERENCES "compressor"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_blowdown" ADD CONSTRAINT "compressor_blowdown_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_blowdown" ADD CONSTRAINT "compressor_blowdown_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_seal_month_methane_emission_override" ADD CONSTRAINT "compressor_seal_month_methane_emission_override_compressor_fkey" FOREIGN KEY ("compressor_seal_id") REFERENCES "compressor_seal"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_seal_month_methane_emission_override" ADD CONSTRAINT "compressor_seal_month_methane_emission_override_created_by_fkey" FOREIGN KEY ("created_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_seal_month_methane_emission_override" ADD CONSTRAINT "compressor_seal_month_methane_emission_override_updated_by_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "tank_farms" ADD CONSTRAINT "tank_farms_facility_id_fkey" FOREIGN KEY ("facility_id") REFERENCES "facilities"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "tank_farms" ADD CONSTRAINT "tank_farms_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "tank_farms" ADD CONSTRAINT "tank_farms_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "tank_farm_changes" ADD CONSTRAINT "tank_farm_changes_tank_farm_id_fkey" FOREIGN KEY ("tank_farm_id") REFERENCES "tank_farms"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "tank_farm_changes" ADD CONSTRAINT "tank_farm_changes_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "tank_farm_changes" ADD CONSTRAINT "tank_farm_changes_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "tank_farm_vent_factors_calculated" ADD CONSTRAINT "tank_farm_vent_factors_calculated_tank_farm_id_fkey" FOREIGN KEY ("tank_farm_id") REFERENCES "tank_farms"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "tank_farm_vent_factors_calculated" ADD CONSTRAINT "tank_farm_vent_factors_calculated_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "tank_farm_vent_factors_calculated" ADD CONSTRAINT "tank_farm_vent_factors_calculated_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "tank_farm_month_oil_flow" ADD CONSTRAINT "tank_farm_month_oil_flow_tank_farm_id_fkey" FOREIGN KEY ("tank_farm_id") REFERENCES "tank_farms"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "tank_farm_month_oil_flow" ADD CONSTRAINT "tank_farm_month_oil_flow_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "tank_farm_month_oil_flow" ADD CONSTRAINT "tank_farm_month_oil_flow_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "tank_farm_month_methane_emission_override" ADD CONSTRAINT "tank_farm_month_methane_emission_override_tank_farm_id_fkey" FOREIGN KEY ("tank_farm_id") REFERENCES "tank_farms"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "tank_farm_month_methane_emission_override" ADD CONSTRAINT "tank_farm_month_methane_emission_override_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "tank_farm_month_methane_emission_override" ADD CONSTRAINT "tank_farm_month_methane_emission_override_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "gas_analyses" ADD CONSTRAINT "gas_analyses_facility_id_fkey" FOREIGN KEY ("facility_id") REFERENCES "facilities"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "gas_analyses" ADD CONSTRAINT "gas_analyses_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "gas_analyses" ADD CONSTRAINT "gas_analyses_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "gas_analysis_calculated_params" ADD CONSTRAINT "gas_analysis_calculated_params_id_fkey" FOREIGN KEY ("id") REFERENCES "gas_analyses"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "gas_analysis_calculated_params" ADD CONSTRAINT "gas_analysis_calculated_params_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "gas_analysis_calculated_params" ADD CONSTRAINT "gas_analysis_calculated_params_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
