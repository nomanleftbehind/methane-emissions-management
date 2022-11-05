-- CreateEnum
CREATE TYPE "user_role" AS ENUM ('ADMIN', 'ENGINEER', 'REGULATORY', 'OFFICE', 'OPERATOR');

-- CreateEnum
CREATE TYPE "facility_type" AS ENUM ('TM', 'WT', 'CT', 'DS', 'GS', 'MS', 'GP', 'IF', 'PL', 'WP', 'WS', 'BT');

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
CREATE TABLE "controller_vent_rates" (
    "id" UUID NOT NULL,
    "date" DATE NOT NULL,
    "rate" DOUBLE PRECISION NOT NULL,
    "controller_id" UUID NOT NULL,
    "created_by_id" UUID NOT NULL,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_by_id" UUID NOT NULL,
    "updated_at" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "controller_vent_rates_pkey" PRIMARY KEY ("id")
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
CREATE UNIQUE INDEX "controller_vent_rates_controller_id_date_key" ON "controller_vent_rates"("controller_id", "date");

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
ALTER TABLE "controller_vent_rates" ADD CONSTRAINT "controller_vent_rates_controller_id_fkey" FOREIGN KEY ("controller_id") REFERENCES "controllers"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "controller_vent_rates" ADD CONSTRAINT "controller_vent_rates_created_by_id_fkey" FOREIGN KEY ("created_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "controller_vent_rates" ADD CONSTRAINT "controller_vent_rates_updated_by_id_fkey" FOREIGN KEY ("updated_by_id") REFERENCES "users"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
