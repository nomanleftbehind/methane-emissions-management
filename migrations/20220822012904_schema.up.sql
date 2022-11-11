-- CreateEnum
CREATE TYPE "user_role" AS ENUM ('ADMIN', 'ENGINEER', 'REGULATORY', 'OFFICE', 'OPERATOR');

-- CreateEnum
CREATE TYPE "facility_type" AS ENUM ('TM', 'WT', 'CT', 'DS', 'GS', 'MS', 'GP', 'IF', 'PL', 'WP', 'WS', 'BT');

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
CREATE TABLE "controllers" (
    "id" UUID NOT NULL,
    "fdc_rec_id" VARCHAR(32) NOT NULL,
    "manufacturer_id" UUID NOT NULL,
    "model" TEXT,
    "serial_number" TEXT,
    "function_id" UUID,
    "facility_id" UUID NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "controllers_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "controller_functions" (
    "id" UUID NOT NULL,
    "function" TEXT NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "controller_functions_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "controller_manufacturers" (
    "id" UUID NOT NULL,
    "manufacturer" TEXT NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "controller_manufacturers_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "controller_changes" (
    "id" UUID NOT NULL,
    "date" DATE NOT NULL,
    "rate" DOUBLE PRECISION NOT NULL,
    "controller_id" UUID NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "controller_changes_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "compressors" (
    "id" UUID NOT NULL,
    "fdc_rec_id" VARCHAR(32) NOT NULL,
    "facility_id" UUID NOT NULL,
    "name" TEXT NOT NULL,
    "serial_number" TEXT NOT NULL,
    "install_date" DATE NOT NULL,
    "remove_date" DATE,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "compressors_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "compressor_changes" (
    "id" UUID NOT NULL,
    "compressor_id" UUID NOT NULL,
    "date" DATE NOT NULL,
    "calculation_method" "calculation_method" NOT NULL,
    "number_of_throws" INTEGER NOT NULL,
    "vent_rate_per_hour" DOUBLE PRECISION NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "compressor_changes_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "compressor_month_vent" (
    "id" UUID NOT NULL,
    "date" DATE NOT NULL,
    "pressurized_hours" DOUBLE PRECISION NOT NULL,
    "rate" DOUBLE PRECISION NOT NULL,
    "compressor_id" UUID NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "compressor_month_vent_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "users_email_key" ON "users"("email");

-- CreateIndex
CREATE UNIQUE INDEX "facilities_idpa_key" ON "facilities"("idpa");

-- CreateIndex
CREATE UNIQUE INDEX "controller_functions_function_key" ON "controller_functions"("function");

-- CreateIndex
CREATE UNIQUE INDEX "controller_manufacturers_manufacturer_key" ON "controller_manufacturers"("manufacturer");

-- CreateIndex
CREATE UNIQUE INDEX "controller_changes_controller_id_date_key" ON "controller_changes"("controller_id", "date");

-- CreateIndex
CREATE UNIQUE INDEX "compressors_serial_number_key" ON "compressors"("serial_number");

-- CreateIndex
CREATE UNIQUE INDEX "compressor_changes_compressor_id_date_key" ON "compressor_changes"("compressor_id", "date");

-- CreateIndex
CREATE UNIQUE INDEX "compressor_month_vent_compressor_id_date_key" ON "compressor_month_vent"("compressor_id", "date");

-- AddForeignKey
ALTER TABLE "facilities" ADD CONSTRAINT "facilities_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "facilities" ADD CONSTRAINT "facilities_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "controllers" ADD CONSTRAINT "controllers_manufacturer_id_fkey" FOREIGN KEY ("manufacturer_id") REFERENCES "controller_manufacturers"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "controllers" ADD CONSTRAINT "controllers_function_id_fkey" FOREIGN KEY ("function_id") REFERENCES "controller_functions"("id") ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "controllers" ADD CONSTRAINT "controllers_facility_id_fkey" FOREIGN KEY ("facility_id") REFERENCES "facilities"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "controllers" ADD CONSTRAINT "controllers_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "controllers" ADD CONSTRAINT "controllers_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "controller_functions" ADD CONSTRAINT "controller_functions_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "controller_functions" ADD CONSTRAINT "controller_functions_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "controller_manufacturers" ADD CONSTRAINT "controller_manufacturers_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "controller_manufacturers" ADD CONSTRAINT "controller_manufacturers_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "controller_changes" ADD CONSTRAINT "controller_changes_controller_id_fkey" FOREIGN KEY ("controller_id") REFERENCES "controllers"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "controller_changes" ADD CONSTRAINT "controller_changes_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "controller_changes" ADD CONSTRAINT "controller_changes_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressors" ADD CONSTRAINT "compressors_facility_id_fkey" FOREIGN KEY ("facility_id") REFERENCES "facilities"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressors" ADD CONSTRAINT "compressors_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressors" ADD CONSTRAINT "compressors_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_changes" ADD CONSTRAINT "compressor_changes_compressor_id_fkey" FOREIGN KEY ("compressor_id") REFERENCES "compressors"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_changes" ADD CONSTRAINT "compressor_changes_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_changes" ADD CONSTRAINT "compressor_changes_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_month_vent" ADD CONSTRAINT "compressor_month_vent_compressor_id_fkey" FOREIGN KEY ("compressor_id") REFERENCES "compressors"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_month_vent" ADD CONSTRAINT "compressor_month_vent_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "compressor_month_vent" ADD CONSTRAINT "compressor_month_vent_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
