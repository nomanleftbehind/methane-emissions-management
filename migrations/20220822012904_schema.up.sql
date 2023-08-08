-- CreateEnum
CREATE TYPE "user_role" AS ENUM ('ADMIN', 'ENGINEER', 'REGULATORY', 'OFFICE', 'OPERATOR');

-- CreateEnum
CREATE TYPE "facility_type" AS ENUM ('TM', 'WT', 'CT', 'DS', 'GS', 'MS', 'GP', 'IF', 'PL', 'WP', 'WS', 'BT');

-- CreateEnum
CREATE TYPE "site_type" AS ENUM ('BATTERY', 'SATELLITE', 'WELL', 'GAS_PLANT', 'COMPRESSOR');

-- CreateEnum
CREATE TYPE "pneumatic_instrument_type" AS ENUM ('PRESSURE_CONTROLLER', 'TEMPERATURE_CONTROLLER', 'SWITCH', 'TRANSDUCER', 'POSITIONER', 'GENERIC_PNEUMATIC_INSTRUMENT');

-- CreateEnum
CREATE TYPE "compressor_type" AS ENUM ('RECIPROCATING', 'CENTRIFUGAL', 'SCREW', 'SCROLL');

-- CreateEnum
CREATE TYPE "control_device" AS ENUM ('FLARE', 'VAPOUR_RECOVERY_UNIT');

-- CreateEnum
CREATE TYPE "control_device_inactivity_reason" AS ENUM ('PLANNED_MAINTENANCE', 'UNPLANNED_MAINTENANCE', 'MALFUNCTION');

-- CreateEnum
CREATE TYPE "seal_type" AS ENUM ('RODPACKING', 'DRY', 'WET');

-- CreateEnum
CREATE TYPE "compressor_seal_testing_point" AS ENUM ('PISTON_ROD_PACKING', 'DISTANCE_PIECE', 'CRANKCASE', 'DRIVE_SHAFT_AND_COMPRESSOR_CASE_INTERFACE');

-- CreateEnum
CREATE TYPE "compressor_emission_type" AS ENUM ('NONCRANKCASE', 'CRANKCASE', 'EMISSION_SURVEY');

-- CreateEnum
CREATE TYPE "calculation_method" AS ENUM ('EQUATION', 'MEASURED');

-- CreateEnum
CREATE TYPE "methane_emission_source" AS ENUM ('PNEUMATIC_DEVICE', 'COMPRESSOR_SEAL', 'GLYCOL_DEHYDRATOR', 'DEFINED_VENT_GAS', 'PLANNED', 'UNPLANNED', 'FUGITIVE');

-- CreateEnum
CREATE TYPE "methane_emission_source_table" AS ENUM ('pneumatic_instrument', 'level_controller', 'pneumatic_pump', 'compressor_seal', 'compressor_blowdown', 'storage_tank');

-- CreateEnum
CREATE TYPE "methane_emission_category" AS ENUM ('ROUTINE', 'NONROUTINE', 'FUGITIVE');

-- CreateTable
CREATE TABLE "user" (
    "id" UUID NOT NULL,
    "email" TEXT NOT NULL,
    "password" TEXT NOT NULL,
    "first_name" TEXT NOT NULL,
    "last_name" TEXT NOT NULL,
    "role" "user_role" NOT NULL DEFAULT 'OPERATOR',

    CONSTRAINT "user_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "facility" (
    "id" UUID NOT NULL,
    "idpa" VARCHAR(12) NOT NULL,
    "name" TEXT NOT NULL,
    "type" "facility_type" NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "facility_pkey" PRIMARY KEY ("id")
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
    "type" "pneumatic_instrument_type" NOT NULL,
    "manufacturer_id" UUID NOT NULL,
    "model" TEXT,
    "serial_number" TEXT,
    "start_date" DATE NOT NULL,
    "end_date" DATE,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "pneumatic_instrument_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "pneumatic_instrument_emission_rate" (
    "id" UUID NOT NULL,
    "pneumatic_instrument_id" UUID NOT NULL,
    "date" DATE NOT NULL,
    "rate" DOUBLE PRECISION NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "pneumatic_instrument_emission_rate_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "pneumatic_instrument_controlled_characterization" (
    "id" UUID NOT NULL,
    "pneumatic_instrument_id" UUID NOT NULL,
    "start_date" DATE NOT NULL,
    "end_date" DATE,
    "control_device" "control_device" NOT NULL,
    "comment" TEXT,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "pneumatic_instrument_controlled_characterization_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "pneumatic_instrument_control_device_inactivity" (
    "id" UUID NOT NULL,
    "pneumatic_instrument_controlled_characterization_id" UUID NOT NULL,
    "start_date" DATE NOT NULL,
    "end_date" DATE,
    "reason" "control_device_inactivity_reason" NOT NULL,
    "comment" TEXT,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "pneumatic_instrument_control_device_inactivity_pkey" PRIMARY KEY ("id")
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
CREATE TABLE "level_controller" (
    "id" UUID NOT NULL,
    "site_id" UUID NOT NULL,
    "manufacturer_id" UUID NOT NULL,
    "model" TEXT,
    "serial_number" TEXT,
    "start_date" DATE NOT NULL,
    "end_date" DATE,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "level_controller_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "level_controller_emission_rate" (
    "id" UUID NOT NULL,
    "level_controller_id" UUID NOT NULL,
    "date" DATE NOT NULL,
    "rate" DOUBLE PRECISION NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "level_controller_emission_rate_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "level_controller_actuation_frequency" (
    "id" UUID NOT NULL,
    "level_controller_id" UUID NOT NULL,
    "date" DATE NOT NULL,
    "actuation_frequency" DOUBLE PRECISION NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "level_controller_actuation_frequency_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "level_controller_controlled_characterization" (
    "id" UUID NOT NULL,
    "level_controller_id" UUID NOT NULL,
    "start_date" DATE NOT NULL,
    "end_date" DATE,
    "control_device" "control_device" NOT NULL,
    "comment" TEXT,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "level_controller_controlled_characterization_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "level_controller_control_device_inactivity" (
    "id" UUID NOT NULL,
    "level_controller_controlled_characterization_id" UUID NOT NULL,
    "start_date" DATE NOT NULL,
    "end_date" DATE,
    "reason" "control_device_inactivity_reason" NOT NULL,
    "comment" TEXT,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "level_controller_control_device_inactivity_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "level_controller_month_hours" (
    "id" UUID NOT NULL,
    "level_controller_id" UUID NOT NULL,
    "month" DATE NOT NULL,
    "hours_on" DOUBLE PRECISION NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "level_controller_month_hours_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "level_controller_month_methane_emission_override" (
    "id" UUID NOT NULL,
    "level_controller_id" UUID NOT NULL,
    "month" DATE NOT NULL,
    "gas_volume" DOUBLE PRECISION NOT NULL,
    "comment" TEXT,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "level_controller_month_methane_emission_override_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "pneumatic_pump" (
    "id" UUID NOT NULL,
    "site_id" UUID NOT NULL,
    "manufacturer_id" UUID NOT NULL,
    "model" TEXT,
    "serial_number" TEXT,
    "start_date" DATE NOT NULL,
    "end_date" DATE,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "pneumatic_pump_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "pneumatic_pump_emission_rate" (
    "id" UUID NOT NULL,
    "pneumatic_pump_id" UUID NOT NULL,
    "date" DATE NOT NULL,
    "rate" DOUBLE PRECISION NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "pneumatic_pump_emission_rate_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "pneumatic_pump_controlled_characterization" (
    "id" UUID NOT NULL,
    "pneumatic_pump_id" UUID NOT NULL,
    "start_date" DATE NOT NULL,
    "end_date" DATE,
    "control_device" "control_device" NOT NULL,
    "comment" TEXT,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "pneumatic_pump_controlled_characterization_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "pneumatic_pump_control_device_inactivity" (
    "id" UUID NOT NULL,
    "pneumatic_pump_controlled_characterization_id" UUID NOT NULL,
    "start_date" DATE NOT NULL,
    "end_date" DATE,
    "reason" "control_device_inactivity_reason" NOT NULL,
    "comment" TEXT,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "pneumatic_pump_control_device_inactivity_pkey" PRIMARY KEY ("id")
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
    "type" "compressor_type" NOT NULL,
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
    "type" "seal_type" NOT NULL,
    "description" TEXT,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "compressor_seal_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "survey_equipment" (
    "id" UUID NOT NULL,
    "make" TEXT NOT NULL,
    "model" TEXT NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "survey_equipment_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "compressor_seal_test" (
    "id" UUID NOT NULL,
    "compressor_seal_id" UUID NOT NULL,
    "start_date" DATE NOT NULL,
    "end_date" DATE,
    "rate" DOUBLE PRECISION NOT NULL,
    "testing_point" "compressor_seal_testing_point" NOT NULL,
    "survey_equipment_id" UUID NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "compressor_seal_test_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "compressor_controlled_characterization" (
    "id" UUID NOT NULL,
    "compressor_id" UUID NOT NULL,
    "start_date" DATE NOT NULL,
    "end_date" DATE,
    "control_device" "control_device" NOT NULL,
    "comment" TEXT,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "compressor_controlled_characterization_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "compressor_control_device_inactivity" (
    "id" UUID NOT NULL,
    "compressor_controlled_characterization_id" UUID NOT NULL,
    "start_date" DATE NOT NULL,
    "end_date" DATE,
    "reason" "control_device_inactivity_reason" NOT NULL,
    "comment" TEXT,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "compressor_control_device_inactivity_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "compressor_emission_survey" (
    "id" UUID NOT NULL,
    "compressor_id" UUID NOT NULL,
    "start_date" DATE NOT NULL,
    "end_date" DATE,
    "rate" DOUBLE PRECISION NOT NULL,
    "survey_point" TEXT NOT NULL,
    "leak_duration" DOUBLE PRECISION,
    "survey_equipment_id" UUID NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "compressor_emission_survey_pkey" PRIMARY KEY ("id")
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
CREATE TABLE "compressor_blowdown_override" (
    "id" UUID NOT NULL,
    "compressor_id" UUID NOT NULL,
    "date" DATE NOT NULL,
    "gas_volume" DOUBLE PRECISION NOT NULL,
    "comment" TEXT,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "compressor_blowdown_override_pkey" PRIMARY KEY ("id")
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
CREATE TABLE "storage_tank" (
    "id" UUID NOT NULL,
    "site_id" UUID NOT NULL,
    "start_date" DATE NOT NULL,
    "end_date" DATE,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "storage_tank_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "storage_tank_change" (
    "id" UUID NOT NULL,
    "storage_tank_id" UUID NOT NULL,
    "date" DATE NOT NULL,
    "ia" BOOLEAN NOT NULL,
    "api_density" DOUBLE PRECISION NOT NULL,
    "temperature" DOUBLE PRECISION NOT NULL,
    "pressure" DOUBLE PRECISION NOT NULL,
    "calculation_method" "calculation_method" NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "storage_tank_change_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "storage_tank_controlled_characterization" (
    "id" UUID NOT NULL,
    "storage_tank_id" UUID NOT NULL,
    "start_date" DATE NOT NULL,
    "end_date" DATE,
    "control_device" "control_device" NOT NULL,
    "comment" TEXT,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "storage_tank_controlled_characterization_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "storage_tank_control_device_inactivity" (
    "id" UUID NOT NULL,
    "storage_tank_controlled_characterization_id" UUID NOT NULL,
    "start_date" DATE NOT NULL,
    "end_date" DATE,
    "reason" "control_device_inactivity_reason" NOT NULL,
    "comment" TEXT,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "storage_tank_control_device_inactivity_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "storage_tank_emission_survey" (
    "id" UUID NOT NULL,
    "storage_tank_id" UUID NOT NULL,
    "start_date" DATE NOT NULL,
    "end_date" DATE,
    "rate" DOUBLE PRECISION NOT NULL,
    "survey_point" TEXT NOT NULL,
    "leak_duration" DOUBLE PRECISION,
    "survey_equipment_id" UUID NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "storage_tank_emission_survey_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "storage_tank_gas_in_solution_factor_calculated" (
    "id" UUID NOT NULL,
    "storage_tank_id" UUID NOT NULL,
    "date" DATE NOT NULL,
    "gis_factor" DOUBLE PRECISION NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "storage_tank_gas_in_solution_factor_calculated_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "storage_tank_month_liquid_hydrocarbon_entering" (
    "id" UUID NOT NULL,
    "storage_tank_id" UUID NOT NULL,
    "month" DATE NOT NULL,
    "liquid_hydrocarbon_volume" DOUBLE PRECISION NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "storage_tank_month_liquid_hydrocarbon_entering_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "storage_tank_month_methane_emission_override" (
    "id" UUID NOT NULL,
    "storage_tank_id" UUID NOT NULL,
    "month" DATE NOT NULL,
    "gas_volume" DOUBLE PRECISION NOT NULL,
    "comment" TEXT,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "storage_tank_month_methane_emission_override_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "gas_analysis" (
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

    CONSTRAINT "gas_analysis_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "gas_analysis_calculated_param" (
    "id" UUID NOT NULL,
    "gas_gravity" DOUBLE PRECISION NOT NULL,
    "higher_heating_value" DOUBLE PRECISION NOT NULL,
    "carbon_content" DOUBLE PRECISION NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "gas_analysis_calculated_param_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "month_methane_emission" (
    "id" UUID NOT NULL,
    "facility_id" UUID NOT NULL,
    "site_id" UUID NOT NULL,
    "source_table" "methane_emission_source_table" NOT NULL,
    "source_table_id" UUID NOT NULL,
    "category" "methane_emission_category" NOT NULL,
    "source" "methane_emission_source" NOT NULL,
    "month" DATE NOT NULL,
    "gas_volume" DOUBLE PRECISION NOT NULL,
    "c1_volume" DOUBLE PRECISION NOT NULL,
    "co2_volume" DOUBLE PRECISION NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "month_methane_emission_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "user_email_key" ON "user"("email");

-- CreateIndex
CREATE UNIQUE INDEX "facility_idpa_key" ON "facility"("idpa");

-- CreateIndex
CREATE UNIQUE INDEX "site_fdc_rec_id_key" ON "site"("fdc_rec_id");

-- CreateIndex
CREATE UNIQUE INDEX "device_manufacturer_manufacturer_key" ON "device_manufacturer"("manufacturer");

-- CreateIndex
CREATE UNIQUE INDEX "pneumatic_instrument_emission_rate_pneumatic_instrument_id__key" ON "pneumatic_instrument_emission_rate"("pneumatic_instrument_id", "date");

-- CreateIndex
CREATE UNIQUE INDEX "pneumatic_instrument_controlled_characterization_pneumatic__key" ON "pneumatic_instrument_controlled_characterization"("pneumatic_instrument_id", "start_date");

-- CreateIndex
CREATE UNIQUE INDEX "pneumatic_instrument_control_device_inactivity_pneumatic_in_key" ON "pneumatic_instrument_control_device_inactivity"("pneumatic_instrument_controlled_characterization_id", "start_date");

-- CreateIndex
CREATE UNIQUE INDEX "pneumatic_instrument_month_hours_pneumatic_instrument_id_mo_key" ON "pneumatic_instrument_month_hours"("pneumatic_instrument_id", "month");

-- CreateIndex
CREATE UNIQUE INDEX "pneumatic_instrument_month_methane_emission_override_pneuma_key" ON "pneumatic_instrument_month_methane_emission_override"("pneumatic_instrument_id", "month");

-- CreateIndex
CREATE UNIQUE INDEX "level_controller_emission_rate_level_controller_id_date_key" ON "level_controller_emission_rate"("level_controller_id", "date");

-- CreateIndex
CREATE UNIQUE INDEX "level_controller_actuation_frequency_level_controller_id_da_key" ON "level_controller_actuation_frequency"("level_controller_id", "date");

-- CreateIndex
CREATE UNIQUE INDEX "level_controller_controlled_characterization_level_controll_key" ON "level_controller_controlled_characterization"("level_controller_id", "start_date");

-- CreateIndex
CREATE UNIQUE INDEX "level_controller_control_device_inactivity_level_controller_key" ON "level_controller_control_device_inactivity"("level_controller_controlled_characterization_id", "start_date");

-- CreateIndex
CREATE UNIQUE INDEX "level_controller_month_hours_level_controller_id_month_key" ON "level_controller_month_hours"("level_controller_id", "month");

-- CreateIndex
CREATE UNIQUE INDEX "level_controller_month_methane_emission_override_level_cont_key" ON "level_controller_month_methane_emission_override"("level_controller_id", "month");

-- CreateIndex
CREATE UNIQUE INDEX "pneumatic_pump_emission_rate_pneumatic_pump_id_date_key" ON "pneumatic_pump_emission_rate"("pneumatic_pump_id", "date");

-- CreateIndex
CREATE UNIQUE INDEX "pneumatic_pump_controlled_characterization_pneumatic_pump_i_key" ON "pneumatic_pump_controlled_characterization"("pneumatic_pump_id", "start_date");

-- CreateIndex
CREATE UNIQUE INDEX "pneumatic_pump_control_device_inactivity_pneumatic_pump_con_key" ON "pneumatic_pump_control_device_inactivity"("pneumatic_pump_controlled_characterization_id", "start_date");

-- CreateIndex
CREATE UNIQUE INDEX "pneumatic_pump_month_hours_pneumatic_pump_id_month_key" ON "pneumatic_pump_month_hours"("pneumatic_pump_id", "month");

-- CreateIndex
CREATE UNIQUE INDEX "pneumatic_pump_month_methane_emission_override_pneumatic_pu_key" ON "pneumatic_pump_month_methane_emission_override"("pneumatic_pump_id", "month");

-- CreateIndex
CREATE UNIQUE INDEX "compressor_fdc_rec_id_key" ON "compressor"("fdc_rec_id");

-- CreateIndex
CREATE UNIQUE INDEX "compressor_serial_number_key" ON "compressor"("serial_number");

-- CreateIndex
CREATE UNIQUE INDEX "survey_equipment_make_model_key" ON "survey_equipment"("make", "model");

-- CreateIndex
CREATE UNIQUE INDEX "compressor_seal_test_compressor_seal_id_start_date_testing__key" ON "compressor_seal_test"("compressor_seal_id", "start_date", "testing_point");

-- CreateIndex
CREATE UNIQUE INDEX "compressor_controlled_characterization_compressor_id_start__key" ON "compressor_controlled_characterization"("compressor_id", "start_date");

-- CreateIndex
CREATE UNIQUE INDEX "compressor_control_device_inactivity_compressor_controlled__key" ON "compressor_control_device_inactivity"("compressor_controlled_characterization_id", "start_date");

-- CreateIndex
CREATE UNIQUE INDEX "compressor_month_hours_compressor_id_month_key" ON "compressor_month_hours"("compressor_id", "month");

-- CreateIndex
CREATE UNIQUE INDEX "compressor_blowdown_compressor_id_date_key" ON "compressor_blowdown"("compressor_id", "date");

-- CreateIndex
CREATE UNIQUE INDEX "compressor_blowdown_override_compressor_id_date_key" ON "compressor_blowdown_override"("compressor_id", "date");

-- CreateIndex
CREATE UNIQUE INDEX "compressor_seal_month_methane_emission_override_compressor__key" ON "compressor_seal_month_methane_emission_override"("compressor_seal_id", "month");

-- CreateIndex
CREATE UNIQUE INDEX "storage_tank_change_storage_tank_id_date_key" ON "storage_tank_change"("storage_tank_id", "date");

-- CreateIndex
CREATE UNIQUE INDEX "storage_tank_controlled_characterization_storage_tank_id_st_key" ON "storage_tank_controlled_characterization"("storage_tank_id", "start_date");

-- CreateIndex
CREATE UNIQUE INDEX "storage_tank_control_device_inactivity_storage_tank_control_key" ON "storage_tank_control_device_inactivity"("storage_tank_controlled_characterization_id", "start_date");

-- CreateIndex
CREATE UNIQUE INDEX "storage_tank_gas_in_solution_factor_calculated_storage_tank_key" ON "storage_tank_gas_in_solution_factor_calculated"("storage_tank_id", "date");

-- CreateIndex
CREATE UNIQUE INDEX "storage_tank_month_liquid_hydrocarbon_entering_storage_tank_key" ON "storage_tank_month_liquid_hydrocarbon_entering"("storage_tank_id", "month");

-- CreateIndex
CREATE UNIQUE INDEX "storage_tank_month_methane_emission_override_storage_tank_i_key" ON "storage_tank_month_methane_emission_override"("storage_tank_id", "month");

-- CreateIndex
CREATE UNIQUE INDEX "gas_analysis_facility_id_date_key" ON "gas_analysis"("facility_id", "date");

-- CreateIndex
CREATE UNIQUE INDEX "month_methane_emission_source_table_source_table_id_categor_key" ON "month_methane_emission"("source_table", "source_table_id", "category", "source", "month");

-- AddForeignKey
ALTER TABLE "facility" ADD CONSTRAINT "facility_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "facility" ADD CONSTRAINT "facility_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "site" ADD CONSTRAINT "site_facility_id_fkey" FOREIGN KEY ("facility_id") REFERENCES "facility"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "site" ADD CONSTRAINT "site_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "site" ADD CONSTRAINT "site_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "device_manufacturer" ADD CONSTRAINT "device_manufacturer_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "device_manufacturer" ADD CONSTRAINT "device_manufacturer_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_instrument" ADD CONSTRAINT "pneumatic_instrument_site_id_fkey" FOREIGN KEY ("site_id") REFERENCES "site"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_instrument" ADD CONSTRAINT "pneumatic_instrument_manufacturer_id_fkey" FOREIGN KEY ("manufacturer_id") REFERENCES "device_manufacturer"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_instrument" ADD CONSTRAINT "pneumatic_instrument_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_instrument" ADD CONSTRAINT "pneumatic_instrument_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_instrument_emission_rate" ADD CONSTRAINT "pneumatic_instrument_emission_rate_pneumatic_instrument_id_fkey" FOREIGN KEY ("pneumatic_instrument_id") REFERENCES "pneumatic_instrument"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_instrument_emission_rate" ADD CONSTRAINT "pneumatic_instrument_emission_rate_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_instrument_emission_rate" ADD CONSTRAINT "pneumatic_instrument_emission_rate_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_instrument_controlled_characterization" ADD CONSTRAINT "pneumatic_instrument_controlled_characterization_pneumatic_fkey" FOREIGN KEY ("pneumatic_instrument_id") REFERENCES "pneumatic_instrument"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_instrument_controlled_characterization" ADD CONSTRAINT "pneumatic_instrument_controlled_characterization_created_b_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_instrument_controlled_characterization" ADD CONSTRAINT "pneumatic_instrument_controlled_characterization_updated_b_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_instrument_control_device_inactivity" ADD CONSTRAINT "pneumatic_instrument_control_device_inactivity_pneumatic_i_fkey" FOREIGN KEY ("pneumatic_instrument_controlled_characterization_id") REFERENCES "pneumatic_instrument_controlled_characterization"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_instrument_control_device_inactivity" ADD CONSTRAINT "pneumatic_instrument_control_device_inactivity_created_by__fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_instrument_control_device_inactivity" ADD CONSTRAINT "pneumatic_instrument_control_device_inactivity_updated_by__fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_instrument_month_hours" ADD CONSTRAINT "pneumatic_instrument_month_hours_pneumatic_instrument_id_fkey" FOREIGN KEY ("pneumatic_instrument_id") REFERENCES "pneumatic_instrument"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_instrument_month_hours" ADD CONSTRAINT "pneumatic_instrument_month_hours_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_instrument_month_hours" ADD CONSTRAINT "pneumatic_instrument_month_hours_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_instrument_month_methane_emission_override" ADD CONSTRAINT "pneumatic_instrument_month_methane_emission_override_pneum_fkey" FOREIGN KEY ("pneumatic_instrument_id") REFERENCES "pneumatic_instrument"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_instrument_month_methane_emission_override" ADD CONSTRAINT "pneumatic_instrument_month_methane_emission_override_creat_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_instrument_month_methane_emission_override" ADD CONSTRAINT "pneumatic_instrument_month_methane_emission_override_updat_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "level_controller" ADD CONSTRAINT "level_controller_site_id_fkey" FOREIGN KEY ("site_id") REFERENCES "site"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "level_controller" ADD CONSTRAINT "level_controller_manufacturer_id_fkey" FOREIGN KEY ("manufacturer_id") REFERENCES "device_manufacturer"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "level_controller" ADD CONSTRAINT "level_controller_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "level_controller" ADD CONSTRAINT "level_controller_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "level_controller_emission_rate" ADD CONSTRAINT "level_controller_emission_rate_level_controller_id_fkey" FOREIGN KEY ("level_controller_id") REFERENCES "level_controller"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "level_controller_emission_rate" ADD CONSTRAINT "level_controller_emission_rate_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "level_controller_emission_rate" ADD CONSTRAINT "level_controller_emission_rate_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "level_controller_actuation_frequency" ADD CONSTRAINT "level_controller_actuation_frequency_level_controller_id_fkey" FOREIGN KEY ("level_controller_id") REFERENCES "level_controller"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "level_controller_actuation_frequency" ADD CONSTRAINT "level_controller_actuation_frequency_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "level_controller_actuation_frequency" ADD CONSTRAINT "level_controller_actuation_frequency_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "level_controller_controlled_characterization" ADD CONSTRAINT "level_controller_controlled_characterization_level_control_fkey" FOREIGN KEY ("level_controller_id") REFERENCES "level_controller"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "level_controller_controlled_characterization" ADD CONSTRAINT "level_controller_controlled_characterization_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "level_controller_controlled_characterization" ADD CONSTRAINT "level_controller_controlled_characterization_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "level_controller_control_device_inactivity" ADD CONSTRAINT "level_controller_control_device_inactivity_level_controlle_fkey" FOREIGN KEY ("level_controller_controlled_characterization_id") REFERENCES "level_controller_controlled_characterization"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "level_controller_control_device_inactivity" ADD CONSTRAINT "level_controller_control_device_inactivity_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "level_controller_control_device_inactivity" ADD CONSTRAINT "level_controller_control_device_inactivity_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "level_controller_month_hours" ADD CONSTRAINT "level_controller_month_hours_level_controller_id_fkey" FOREIGN KEY ("level_controller_id") REFERENCES "level_controller"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "level_controller_month_hours" ADD CONSTRAINT "level_controller_month_hours_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "level_controller_month_hours" ADD CONSTRAINT "level_controller_month_hours_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "level_controller_month_methane_emission_override" ADD CONSTRAINT "level_controller_month_methane_emission_override_level_con_fkey" FOREIGN KEY ("level_controller_id") REFERENCES "level_controller"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "level_controller_month_methane_emission_override" ADD CONSTRAINT "level_controller_month_methane_emission_override_created_b_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "level_controller_month_methane_emission_override" ADD CONSTRAINT "level_controller_month_methane_emission_override_updated_b_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_pump" ADD CONSTRAINT "pneumatic_pump_site_id_fkey" FOREIGN KEY ("site_id") REFERENCES "site"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_pump" ADD CONSTRAINT "pneumatic_pump_manufacturer_id_fkey" FOREIGN KEY ("manufacturer_id") REFERENCES "device_manufacturer"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_pump" ADD CONSTRAINT "pneumatic_pump_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_pump" ADD CONSTRAINT "pneumatic_pump_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_pump_emission_rate" ADD CONSTRAINT "pneumatic_pump_emission_rate_pneumatic_pump_id_fkey" FOREIGN KEY ("pneumatic_pump_id") REFERENCES "pneumatic_pump"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_pump_emission_rate" ADD CONSTRAINT "pneumatic_pump_emission_rate_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_pump_emission_rate" ADD CONSTRAINT "pneumatic_pump_emission_rate_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_pump_controlled_characterization" ADD CONSTRAINT "pneumatic_pump_controlled_characterization_pneumatic_pump__fkey" FOREIGN KEY ("pneumatic_pump_id") REFERENCES "pneumatic_pump"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_pump_controlled_characterization" ADD CONSTRAINT "pneumatic_pump_controlled_characterization_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_pump_controlled_characterization" ADD CONSTRAINT "pneumatic_pump_controlled_characterization_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_pump_control_device_inactivity" ADD CONSTRAINT "pneumatic_pump_control_device_inactivity_pneumatic_pump_co_fkey" FOREIGN KEY ("pneumatic_pump_controlled_characterization_id") REFERENCES "pneumatic_pump_controlled_characterization"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_pump_control_device_inactivity" ADD CONSTRAINT "pneumatic_pump_control_device_inactivity_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_pump_control_device_inactivity" ADD CONSTRAINT "pneumatic_pump_control_device_inactivity_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_pump_month_hours" ADD CONSTRAINT "pneumatic_pump_month_hours_pneumatic_pump_id_fkey" FOREIGN KEY ("pneumatic_pump_id") REFERENCES "pneumatic_pump"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_pump_month_hours" ADD CONSTRAINT "pneumatic_pump_month_hours_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_pump_month_hours" ADD CONSTRAINT "pneumatic_pump_month_hours_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_pump_month_methane_emission_override" ADD CONSTRAINT "pneumatic_pump_month_methane_emission_override_pneumatic_p_fkey" FOREIGN KEY ("pneumatic_pump_id") REFERENCES "pneumatic_pump"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_pump_month_methane_emission_override" ADD CONSTRAINT "pneumatic_pump_month_methane_emission_override_created_by__fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "pneumatic_pump_month_methane_emission_override" ADD CONSTRAINT "pneumatic_pump_month_methane_emission_override_updated_by__fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor" ADD CONSTRAINT "compressor_site_id_fkey" FOREIGN KEY ("site_id") REFERENCES "site"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor" ADD CONSTRAINT "compressor_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor" ADD CONSTRAINT "compressor_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_seal" ADD CONSTRAINT "compressor_seal_id_fkey" FOREIGN KEY ("id") REFERENCES "compressor"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_seal" ADD CONSTRAINT "compressor_seal_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_seal" ADD CONSTRAINT "compressor_seal_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "survey_equipment" ADD CONSTRAINT "survey_equipment_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "survey_equipment" ADD CONSTRAINT "survey_equipment_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_seal_test" ADD CONSTRAINT "compressor_seal_test_compressor_seal_id_fkey" FOREIGN KEY ("compressor_seal_id") REFERENCES "compressor_seal"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_seal_test" ADD CONSTRAINT "compressor_seal_test_survey_equipment_id_fkey" FOREIGN KEY ("survey_equipment_id") REFERENCES "survey_equipment"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_seal_test" ADD CONSTRAINT "compressor_seal_test_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_seal_test" ADD CONSTRAINT "compressor_seal_test_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_controlled_characterization" ADD CONSTRAINT "compressor_controlled_characterization_compressor_id_fkey" FOREIGN KEY ("compressor_id") REFERENCES "compressor"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_controlled_characterization" ADD CONSTRAINT "compressor_controlled_characterization_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_controlled_characterization" ADD CONSTRAINT "compressor_controlled_characterization_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_control_device_inactivity" ADD CONSTRAINT "compressor_control_device_inactivity_compressor_controlled_fkey" FOREIGN KEY ("compressor_controlled_characterization_id") REFERENCES "compressor_controlled_characterization"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_control_device_inactivity" ADD CONSTRAINT "compressor_control_device_inactivity_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_control_device_inactivity" ADD CONSTRAINT "compressor_control_device_inactivity_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_emission_survey" ADD CONSTRAINT "compressor_emission_survey_compressor_id_fkey" FOREIGN KEY ("compressor_id") REFERENCES "compressor"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_emission_survey" ADD CONSTRAINT "compressor_emission_survey_survey_equipment_id_fkey" FOREIGN KEY ("survey_equipment_id") REFERENCES "survey_equipment"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_emission_survey" ADD CONSTRAINT "compressor_emission_survey_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_emission_survey" ADD CONSTRAINT "compressor_emission_survey_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_month_hours" ADD CONSTRAINT "compressor_month_hours_compressor_id_fkey" FOREIGN KEY ("compressor_id") REFERENCES "compressor"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_month_hours" ADD CONSTRAINT "compressor_month_hours_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_month_hours" ADD CONSTRAINT "compressor_month_hours_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_blowdown" ADD CONSTRAINT "compressor_blowdown_compressor_id_fkey" FOREIGN KEY ("compressor_id") REFERENCES "compressor"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_blowdown" ADD CONSTRAINT "compressor_blowdown_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_blowdown" ADD CONSTRAINT "compressor_blowdown_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_blowdown_override" ADD CONSTRAINT "compressor_blowdown_override_compressor_id_fkey" FOREIGN KEY ("compressor_id") REFERENCES "compressor"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_blowdown_override" ADD CONSTRAINT "compressor_blowdown_override_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_blowdown_override" ADD CONSTRAINT "compressor_blowdown_override_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_seal_month_methane_emission_override" ADD CONSTRAINT "compressor_seal_month_methane_emission_override_compressor_fkey" FOREIGN KEY ("compressor_seal_id") REFERENCES "compressor_seal"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_seal_month_methane_emission_override" ADD CONSTRAINT "compressor_seal_month_methane_emission_override_created_by_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_seal_month_methane_emission_override" ADD CONSTRAINT "compressor_seal_month_methane_emission_override_updated_by_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "storage_tank" ADD CONSTRAINT "storage_tank_site_id_fkey" FOREIGN KEY ("site_id") REFERENCES "site"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "storage_tank" ADD CONSTRAINT "storage_tank_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "storage_tank" ADD CONSTRAINT "storage_tank_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "storage_tank_change" ADD CONSTRAINT "storage_tank_change_storage_tank_id_fkey" FOREIGN KEY ("storage_tank_id") REFERENCES "storage_tank"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "storage_tank_change" ADD CONSTRAINT "storage_tank_change_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "storage_tank_change" ADD CONSTRAINT "storage_tank_change_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "storage_tank_controlled_characterization" ADD CONSTRAINT "storage_tank_controlled_characterization_storage_tank_id_fkey" FOREIGN KEY ("storage_tank_id") REFERENCES "storage_tank"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "storage_tank_controlled_characterization" ADD CONSTRAINT "storage_tank_controlled_characterization_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "storage_tank_controlled_characterization" ADD CONSTRAINT "storage_tank_controlled_characterization_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "storage_tank_control_device_inactivity" ADD CONSTRAINT "storage_tank_control_device_inactivity_storage_tank_contro_fkey" FOREIGN KEY ("storage_tank_controlled_characterization_id") REFERENCES "storage_tank_controlled_characterization"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "storage_tank_control_device_inactivity" ADD CONSTRAINT "storage_tank_control_device_inactivity_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "storage_tank_control_device_inactivity" ADD CONSTRAINT "storage_tank_control_device_inactivity_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "storage_tank_emission_survey" ADD CONSTRAINT "storage_tank_emission_survey_storage_tank_id_fkey" FOREIGN KEY ("storage_tank_id") REFERENCES "storage_tank"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "storage_tank_emission_survey" ADD CONSTRAINT "storage_tank_emission_survey_survey_equipment_id_fkey" FOREIGN KEY ("survey_equipment_id") REFERENCES "survey_equipment"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "storage_tank_emission_survey" ADD CONSTRAINT "storage_tank_emission_survey_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "storage_tank_emission_survey" ADD CONSTRAINT "storage_tank_emission_survey_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "storage_tank_gas_in_solution_factor_calculated" ADD CONSTRAINT "storage_tank_gas_in_solution_factor_calculated_storage_tan_fkey" FOREIGN KEY ("storage_tank_id") REFERENCES "storage_tank"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "storage_tank_gas_in_solution_factor_calculated" ADD CONSTRAINT "storage_tank_gas_in_solution_factor_calculated_created_by__fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "storage_tank_gas_in_solution_factor_calculated" ADD CONSTRAINT "storage_tank_gas_in_solution_factor_calculated_updated_by__fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "storage_tank_month_liquid_hydrocarbon_entering" ADD CONSTRAINT "storage_tank_month_liquid_hydrocarbon_entering_storage_tan_fkey" FOREIGN KEY ("storage_tank_id") REFERENCES "storage_tank"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "storage_tank_month_liquid_hydrocarbon_entering" ADD CONSTRAINT "storage_tank_month_liquid_hydrocarbon_entering_created_by__fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "storage_tank_month_liquid_hydrocarbon_entering" ADD CONSTRAINT "storage_tank_month_liquid_hydrocarbon_entering_updated_by__fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "storage_tank_month_methane_emission_override" ADD CONSTRAINT "storage_tank_month_methane_emission_override_storage_tank__fkey" FOREIGN KEY ("storage_tank_id") REFERENCES "storage_tank"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "storage_tank_month_methane_emission_override" ADD CONSTRAINT "storage_tank_month_methane_emission_override_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "storage_tank_month_methane_emission_override" ADD CONSTRAINT "storage_tank_month_methane_emission_override_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "gas_analysis" ADD CONSTRAINT "gas_analysis_facility_id_fkey" FOREIGN KEY ("facility_id") REFERENCES "facility"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "gas_analysis" ADD CONSTRAINT "gas_analysis_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "gas_analysis" ADD CONSTRAINT "gas_analysis_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "gas_analysis_calculated_param" ADD CONSTRAINT "gas_analysis_calculated_param_id_fkey" FOREIGN KEY ("id") REFERENCES "gas_analysis"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "gas_analysis_calculated_param" ADD CONSTRAINT "gas_analysis_calculated_param_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "gas_analysis_calculated_param" ADD CONSTRAINT "gas_analysis_calculated_param_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "month_methane_emission" ADD CONSTRAINT "month_methane_emission_facility_id_fkey" FOREIGN KEY ("facility_id") REFERENCES "facility"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "month_methane_emission" ADD CONSTRAINT "month_methane_emission_site_id_fkey" FOREIGN KEY ("site_id") REFERENCES "site"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "month_methane_emission" ADD CONSTRAINT "month_methane_emission_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "month_methane_emission" ADD CONSTRAINT "month_methane_emission_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
