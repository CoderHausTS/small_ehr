-- This file should undo anything in `up.sql`
ALTER TABLE "allergies" DROP COLUMN "patient_id";
ALTER TABLE "allergies" DROP COLUMN "encounter_id";
ALTER TABLE "allergies" ADD COLUMN "patient" UUID NOT NULL;
ALTER TABLE "allergies" ADD COLUMN "encounter" UUID NOT NULL;

ALTER TABLE "careplans" DROP COLUMN "patient_id";
ALTER TABLE "careplans" DROP COLUMN "encounter_id";
ALTER TABLE "careplans" ADD COLUMN "patient" UUID;
ALTER TABLE "careplans" ADD COLUMN "encounter" UUID;

ALTER TABLE "claims" DROP COLUMN "outstanding_primary_payor_amount";
ALTER TABLE "claims" DROP COLUMN "outstanding_secondary_payor_amount";
ALTER TABLE "claims" DROP COLUMN "outstanding_patient_amount";
ALTER TABLE "claims" ADD COLUMN "outstanding_primary_payor_amount" MONEY;
ALTER TABLE "claims" ADD COLUMN "outstanding_secondary_payor_amount" MONEY;
ALTER TABLE "claims" ADD COLUMN "outstanding_patient_amount" MONEY;

ALTER TABLE "claims_transactions" DROP COLUMN "place_of_service_id";
ALTER TABLE "claims_transactions" DROP COLUMN "amount";
ALTER TABLE "claims_transactions" DROP COLUMN "unit_amount";
ALTER TABLE "claims_transactions" DROP COLUMN "payments";
ALTER TABLE "claims_transactions" DROP COLUMN "adjustments";
ALTER TABLE "claims_transactions" DROP COLUMN "transfers";
ALTER TABLE "claims_transactions" DROP COLUMN "outstanding";
ALTER TABLE "claims_transactions" ADD COLUMN "place_of_service" UUID NOT NULL;
ALTER TABLE "claims_transactions" ADD COLUMN "amount" MONEY;
ALTER TABLE "claims_transactions" ADD COLUMN "unit_amount" MONEY;
ALTER TABLE "claims_transactions" ADD COLUMN "payments" MONEY;
ALTER TABLE "claims_transactions" ADD COLUMN "adjustments" MONEY;
ALTER TABLE "claims_transactions" ADD COLUMN "transfers" MONEY;
ALTER TABLE "claims_transactions" ADD COLUMN "outstanding" MONEY;



ALTER TABLE "encounters" DROP COLUMN "base_encounter_cost";
ALTER TABLE "encounters" DROP COLUMN "total_claim_cost";
ALTER TABLE "encounters" DROP COLUMN "payer_coverage";
ALTER TABLE "encounters" ADD COLUMN "base_encounter_cost" MONEY;
ALTER TABLE "encounters" ADD COLUMN "total_claim_cost" MONEY;
ALTER TABLE "encounters" ADD COLUMN "payer_coverage" MONEY;


ALTER TABLE "immunizations" DROP COLUMN "base_cost";
ALTER TABLE "immunizations" ADD COLUMN "base_cost" MONEY;

ALTER TABLE "medications" DROP COLUMN "base_cost";
ALTER TABLE "medications" DROP COLUMN "payer_coverage";
ALTER TABLE "medications" DROP COLUMN "total_cost";
ALTER TABLE "medications" ADD COLUMN "base_cost" MONEY;
ALTER TABLE "medications" ADD COLUMN "payer_coverage" MONEY;
ALTER TABLE "medications" ADD COLUMN "total_cost" MONEY;


ALTER TABLE "organizations" DROP COLUMN "revenue";
ALTER TABLE "organizations" DROP COLUMN "utilization";
ALTER TABLE "organizations" ADD COLUMN "revenue" MONEY;
ALTER TABLE "organizations" ADD COLUMN "utilization" INT8;



ALTER TABLE "payers" DROP COLUMN "amount_covered";
ALTER TABLE "payers" DROP COLUMN "amount_uncovered";
ALTER TABLE "payers" DROP COLUMN "revenue";
ALTER TABLE "payers" ADD COLUMN "amount_covered" MONEY;
ALTER TABLE "payers" ADD COLUMN "amount_uncovered" MONEY;
ALTER TABLE "payers" ADD COLUMN "revenue" MONEY;

ALTER TABLE "procedures" DROP COLUMN "base_cost";
ALTER TABLE "procedures" ADD COLUMN "base_cost" MONEY;



