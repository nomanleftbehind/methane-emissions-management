-- CreateEnum
CREATE TYPE "user_role" AS ENUM ('ADMIN', 'ENGINEER', 'REGULATORY', 'OFFICE', 'OPERATOR');

-- CreateEnum
CREATE TYPE "facility_type" AS ENUM ('TM', 'WT', 'CT', 'DS', 'GS', 'MS', 'GP', 'IF', 'PL', 'WP', 'WS', 'BT');

-- CreateEnum
CREATE TYPE "site_type" AS ENUM ('BATTERY', 'SATELLITE', 'WELL', 'GAS_PLANT', 'COMPRESSOR');

-- CreateEnum
CREATE TYPE "pneumatic_device_type" AS ENUM ('PRESSURE_CONTROLLER', 'TEMPERATURE_CONTROLLER', 'SWITCH', 'TRANSDUCER', 'POSITIONER', 'PNEUMATIC_PUMP', 'GENERIC_PNEUMATIC_INSTRUMENT');

-- CreateEnum
CREATE TYPE "compressor_type" AS ENUM ('RECIPROCATING', 'CENTRIFUGAL', 'SCREW', 'SCROLL');

-- CreateEnum
CREATE TYPE "controlled_characterization" AS ENUM ('CONTROLLED', 'UNCONTROLLED');

-- CreateEnum
CREATE TYPE "seal_type" AS ENUM ('RODPACKING', 'DRY', 'WET');

-- CreateEnum
CREATE TYPE "compressor_seal_testing_point" AS ENUM ('PISTON_ROD_PACKING', 'DISTANCE_PIECE', 'CRANKCASE', 'DRIVE_SHAFT_AND_COMPRESSOR_CASE_INTERFACE');

-- CreateEnum
CREATE TYPE "calculation_method" AS ENUM ('EQUATION', 'MEASURED');

-- CreateEnum
CREATE TYPE "methane_emission_source" AS ENUM ('PNEUMATIC_DEVICE', 'COMPRESSOR_SEAL', 'GLYCOL_DEHYDRATOR', 'DEFINED_VENT_GAS', 'PLANNED', 'UNPLANNED', 'FUGITIVE');

-- CreateEnum
CREATE TYPE "methane_emission_source_table" AS ENUM ('non_level_controller', 'level_controller', 'compressor_seal', 'compressor_blowdown', 'tank');

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
CREATE TABLE "non_level_controller" (
    "id" UUID NOT NULL,
    "site_id" UUID NOT NULL,
    "type" "pneumatic_device_type" NOT NULL,
    "manufacturer_id" UUID NOT NULL,
    "model" TEXT,
    "serial_number" TEXT,
    "start_date" DATE NOT NULL,
    "end_date" DATE,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "non_level_controller_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "non_level_controller_change" (
    "id" UUID NOT NULL,
    "non_level_controller_id" UUID NOT NULL,
    "date" DATE NOT NULL,
    "rate" DOUBLE PRECISION NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "non_level_controller_change_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "non_level_controller_month_hours" (
    "id" UUID NOT NULL,
    "non_level_controller_id" UUID NOT NULL,
    "month" DATE NOT NULL,
    "hours_on" DOUBLE PRECISION NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "non_level_controller_month_hours_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "non_level_controller_month_methane_emission_override" (
    "id" UUID NOT NULL,
    "non_level_controller_id" UUID NOT NULL,
    "month" DATE NOT NULL,
    "gas_volume" DOUBLE PRECISION NOT NULL,
    "comment" TEXT,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "non_level_controller_month_methane_emission_override_pkey" PRIMARY KEY ("id")
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
CREATE TABLE "level_controller_change" (
    "id" UUID NOT NULL,
    "level_controller_id" UUID NOT NULL,
    "date" DATE NOT NULL,
    "rate" DOUBLE PRECISION NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "level_controller_change_pkey" PRIMARY KEY ("id")
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
    "date" DATE NOT NULL,
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
    "date" DATE NOT NULL,
    "controlled_characterization" "controlled_characterization" NOT NULL,
    "comment" TEXT,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "compressor_controlled_characterization_pkey" PRIMARY KEY ("id")
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
CREATE TABLE "tank" (
    "id" UUID NOT NULL,
    "site_id" UUID NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "tank_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "tank_change" (
    "id" UUID NOT NULL,
    "tank_id" UUID NOT NULL,
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

    CONSTRAINT "tank_change_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "tank_emission_factor_calculated" (
    "id" UUID NOT NULL,
    "tank_id" UUID NOT NULL,
    "date" DATE NOT NULL,
    "emission_factor" DOUBLE PRECISION NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "tank_emission_factor_calculated_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "tank_month_oil_flow" (
    "id" UUID NOT NULL,
    "tank_id" UUID NOT NULL,
    "month" DATE NOT NULL,
    "oil" DOUBLE PRECISION NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "tank_month_oil_flow_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "tank_month_methane_emission_override" (
    "id" UUID NOT NULL,
    "tank_id" UUID NOT NULL,
    "month" DATE NOT NULL,
    "gas_volume" DOUBLE PRECISION NOT NULL,
    "comment" TEXT,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "tank_month_methane_emission_override_pkey" PRIMARY KEY ("id")
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
CREATE UNIQUE INDEX "non_level_controller_change_non_level_controller_id_date_key" ON "non_level_controller_change"("non_level_controller_id", "date");

-- CreateIndex
CREATE UNIQUE INDEX "non_level_controller_month_hours_non_level_controller_id_mo_key" ON "non_level_controller_month_hours"("non_level_controller_id", "month");

-- CreateIndex
CREATE UNIQUE INDEX "non_level_controller_month_methane_emission_override_non_le_key" ON "non_level_controller_month_methane_emission_override"("non_level_controller_id", "month");

-- CreateIndex
CREATE UNIQUE INDEX "level_controller_change_level_controller_id_date_key" ON "level_controller_change"("level_controller_id", "date");

-- CreateIndex
CREATE UNIQUE INDEX "level_controller_actuation_frequency_level_controller_id_da_key" ON "level_controller_actuation_frequency"("level_controller_id", "date");

-- CreateIndex
CREATE UNIQUE INDEX "level_controller_month_hours_level_controller_id_month_key" ON "level_controller_month_hours"("level_controller_id", "month");

-- CreateIndex
CREATE UNIQUE INDEX "level_controller_month_methane_emission_override_level_cont_key" ON "level_controller_month_methane_emission_override"("level_controller_id", "month");

-- CreateIndex
CREATE UNIQUE INDEX "compressor_fdc_rec_id_key" ON "compressor"("fdc_rec_id");

-- CreateIndex
CREATE UNIQUE INDEX "compressor_serial_number_key" ON "compressor"("serial_number");

-- CreateIndex
CREATE UNIQUE INDEX "survey_equipment_make_model_key" ON "survey_equipment"("make", "model");

-- CreateIndex
CREATE UNIQUE INDEX "compressor_seal_test_compressor_seal_id_date_testing_point_key" ON "compressor_seal_test"("compressor_seal_id", "date", "testing_point");

-- CreateIndex
CREATE UNIQUE INDEX "compressor_controlled_characterization_compressor_id_date_key" ON "compressor_controlled_characterization"("compressor_id", "date");

-- CreateIndex
CREATE UNIQUE INDEX "compressor_month_hours_compressor_id_month_key" ON "compressor_month_hours"("compressor_id", "month");

-- CreateIndex
CREATE UNIQUE INDEX "compressor_blowdown_compressor_id_date_key" ON "compressor_blowdown"("compressor_id", "date");

-- CreateIndex
CREATE UNIQUE INDEX "compressor_blowdown_override_compressor_id_date_key" ON "compressor_blowdown_override"("compressor_id", "date");

-- CreateIndex
CREATE UNIQUE INDEX "compressor_seal_month_methane_emission_override_compressor__key" ON "compressor_seal_month_methane_emission_override"("compressor_seal_id", "month");

-- CreateIndex
CREATE UNIQUE INDEX "tank_change_tank_id_date_key" ON "tank_change"("tank_id", "date");

-- CreateIndex
CREATE UNIQUE INDEX "tank_emission_factor_calculated_tank_id_date_key" ON "tank_emission_factor_calculated"("tank_id", "date");

-- CreateIndex
CREATE UNIQUE INDEX "tank_month_oil_flow_tank_id_month_key" ON "tank_month_oil_flow"("tank_id", "month");

-- CreateIndex
CREATE UNIQUE INDEX "tank_month_methane_emission_override_tank_id_month_key" ON "tank_month_methane_emission_override"("tank_id", "month");

-- CreateIndex
CREATE UNIQUE INDEX "gas_analysis_facility_id_date_key" ON "gas_analysis"("facility_id", "date");

-- CreateIndex
CREATE UNIQUE INDEX "month_methane_emission_source_table_id_category_source_mont_key" ON "month_methane_emission"("source_table_id", "category", "source", "month");

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
ALTER TABLE "non_level_controller" ADD CONSTRAINT "non_level_controller_site_id_fkey" FOREIGN KEY ("site_id") REFERENCES "site"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "non_level_controller" ADD CONSTRAINT "non_level_controller_manufacturer_id_fkey" FOREIGN KEY ("manufacturer_id") REFERENCES "device_manufacturer"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "non_level_controller" ADD CONSTRAINT "non_level_controller_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "non_level_controller" ADD CONSTRAINT "non_level_controller_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "non_level_controller_change" ADD CONSTRAINT "non_level_controller_change_non_level_controller_id_fkey" FOREIGN KEY ("non_level_controller_id") REFERENCES "non_level_controller"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "non_level_controller_change" ADD CONSTRAINT "non_level_controller_change_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "non_level_controller_change" ADD CONSTRAINT "non_level_controller_change_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "non_level_controller_month_hours" ADD CONSTRAINT "non_level_controller_month_hours_non_level_controller_id_fkey" FOREIGN KEY ("non_level_controller_id") REFERENCES "non_level_controller"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "non_level_controller_month_hours" ADD CONSTRAINT "non_level_controller_month_hours_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "non_level_controller_month_hours" ADD CONSTRAINT "non_level_controller_month_hours_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "non_level_controller_month_methane_emission_override" ADD CONSTRAINT "non_level_controller_month_methane_emission_override_non_l_fkey" FOREIGN KEY ("non_level_controller_id") REFERENCES "non_level_controller"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "non_level_controller_month_methane_emission_override" ADD CONSTRAINT "non_level_controller_month_methane_emission_override_creat_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "non_level_controller_month_methane_emission_override" ADD CONSTRAINT "non_level_controller_month_methane_emission_override_updat_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "level_controller" ADD CONSTRAINT "level_controller_site_id_fkey" FOREIGN KEY ("site_id") REFERENCES "site"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "level_controller" ADD CONSTRAINT "level_controller_manufacturer_id_fkey" FOREIGN KEY ("manufacturer_id") REFERENCES "device_manufacturer"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "level_controller" ADD CONSTRAINT "level_controller_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "level_controller" ADD CONSTRAINT "level_controller_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "level_controller_change" ADD CONSTRAINT "level_controller_change_level_controller_id_fkey" FOREIGN KEY ("level_controller_id") REFERENCES "level_controller"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "level_controller_change" ADD CONSTRAINT "level_controller_change_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "level_controller_change" ADD CONSTRAINT "level_controller_change_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "level_controller_actuation_frequency" ADD CONSTRAINT "level_controller_actuation_frequency_level_controller_id_fkey" FOREIGN KEY ("level_controller_id") REFERENCES "level_controller"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "level_controller_actuation_frequency" ADD CONSTRAINT "level_controller_actuation_frequency_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "level_controller_actuation_frequency" ADD CONSTRAINT "level_controller_actuation_frequency_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

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
ALTER TABLE "tank" ADD CONSTRAINT "tank_site_id_fkey" FOREIGN KEY ("site_id") REFERENCES "site"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "tank" ADD CONSTRAINT "tank_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "tank" ADD CONSTRAINT "tank_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "tank_change" ADD CONSTRAINT "tank_change_tank_id_fkey" FOREIGN KEY ("tank_id") REFERENCES "tank"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "tank_change" ADD CONSTRAINT "tank_change_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "tank_change" ADD CONSTRAINT "tank_change_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "tank_emission_factor_calculated" ADD CONSTRAINT "tank_emission_factor_calculated_tank_id_fkey" FOREIGN KEY ("tank_id") REFERENCES "tank"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "tank_emission_factor_calculated" ADD CONSTRAINT "tank_emission_factor_calculated_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "tank_emission_factor_calculated" ADD CONSTRAINT "tank_emission_factor_calculated_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "tank_month_oil_flow" ADD CONSTRAINT "tank_month_oil_flow_tank_id_fkey" FOREIGN KEY ("tank_id") REFERENCES "tank"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "tank_month_oil_flow" ADD CONSTRAINT "tank_month_oil_flow_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "tank_month_oil_flow" ADD CONSTRAINT "tank_month_oil_flow_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "tank_month_methane_emission_override" ADD CONSTRAINT "tank_month_methane_emission_override_tank_id_fkey" FOREIGN KEY ("tank_id") REFERENCES "tank"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "tank_month_methane_emission_override" ADD CONSTRAINT "tank_month_methane_emission_override_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "tank_month_methane_emission_override" ADD CONSTRAINT "tank_month_methane_emission_override_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "user"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

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
