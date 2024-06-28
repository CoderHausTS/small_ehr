-- Your SQL goes here
ALTER TABLE "allergies" DROP COLUMN "patient";
ALTER TABLE "allergies" DROP COLUMN "encounter";
ALTER TABLE "allergies" ADD COLUMN "patient_id" UUID NOT NULL;
ALTER TABLE "allergies" ADD COLUMN "encounter_id" UUID NOT NULL;

ALTER TABLE "careplans" DROP COLUMN "patient";
ALTER TABLE "careplans" DROP COLUMN "encounter";
ALTER TABLE "careplans" ADD COLUMN "patient_id" UUID;
ALTER TABLE "careplans" ADD COLUMN "encounter_id" UUID;

ALTER TABLE "claims" DROP COLUMN "outstanding_primary_payor_amount";
ALTER TABLE "claims" DROP COLUMN "outstanding_secondary_payor_amount";
ALTER TABLE "claims" DROP COLUMN "outstanding_patient_amount";
ALTER TABLE "claims" ADD COLUMN "outstanding_primary_payor_amount" NUMERIC;
ALTER TABLE "claims" ADD COLUMN "outstanding_secondary_payor_amount" NUMERIC;
ALTER TABLE "claims" ADD COLUMN "outstanding_patient_amount" NUMERIC;

ALTER TABLE "claims_transactions" DROP COLUMN "place_of_service";
ALTER TABLE "claims_transactions" DROP COLUMN "amount";
ALTER TABLE "claims_transactions" DROP COLUMN "unit_amount";
ALTER TABLE "claims_transactions" DROP COLUMN "payments";
ALTER TABLE "claims_transactions" DROP COLUMN "adjustments";
ALTER TABLE "claims_transactions" DROP COLUMN "transfers";
ALTER TABLE "claims_transactions" DROP COLUMN "outstanding";
ALTER TABLE "claims_transactions" ADD COLUMN "place_of_service_id" UUID NOT NULL;
ALTER TABLE "claims_transactions" ADD COLUMN "amount" NUMERIC;
ALTER TABLE "claims_transactions" ADD COLUMN "unit_amount" NUMERIC;
ALTER TABLE "claims_transactions" ADD COLUMN "payments" NUMERIC;
ALTER TABLE "claims_transactions" ADD COLUMN "adjustments" NUMERIC;
ALTER TABLE "claims_transactions" ADD COLUMN "transfers" NUMERIC;
ALTER TABLE "claims_transactions" ADD COLUMN "outstanding" NUMERIC;



ALTER TABLE "encounters" DROP COLUMN "base_encounter_cost";
ALTER TABLE "encounters" DROP COLUMN "total_claim_cost";
ALTER TABLE "encounters" DROP COLUMN "payer_coverage";
ALTER TABLE "encounters" ADD COLUMN "base_encounter_cost" NUMERIC;
ALTER TABLE "encounters" ADD COLUMN "total_claim_cost" NUMERIC;
ALTER TABLE "encounters" ADD COLUMN "payer_coverage" NUMERIC;


ALTER TABLE "immunizations" DROP COLUMN "base_cost";
ALTER TABLE "immunizations" ADD COLUMN "base_cost" NUMERIC;

ALTER TABLE "medications" DROP COLUMN "base_cost";
ALTER TABLE "medications" DROP COLUMN "payer_coverage";
ALTER TABLE "medications" DROP COLUMN "total_cost";
ALTER TABLE "medications" ADD COLUMN "base_cost" NUMERIC;
ALTER TABLE "medications" ADD COLUMN "payer_coverage" NUMERIC;
ALTER TABLE "medications" ADD COLUMN "total_cost" NUMERIC;


ALTER TABLE "organizations" DROP COLUMN "revenue";
ALTER TABLE "organizations" DROP COLUMN "utilization";
ALTER TABLE "organizations" ADD COLUMN "revenue" NUMERIC;
ALTER TABLE "organizations" ADD COLUMN "utilization" INTEGER;



ALTER TABLE "payers" DROP COLUMN "amount_covered";
ALTER TABLE "payers" DROP COLUMN "amount_uncovered";
ALTER TABLE "payers" DROP COLUMN "revenue";
ALTER TABLE "payers" ADD COLUMN "amount_covered" NUMERIC;
ALTER TABLE "payers" ADD COLUMN "amount_uncovered" NUMERIC;
ALTER TABLE "payers" ADD COLUMN "revenue" NUMERIC;

ALTER TABLE "procedures" DROP COLUMN "base_cost";
ALTER TABLE "procedures" ADD COLUMN "base_cost" NUMERIC;



