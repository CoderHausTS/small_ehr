-- Your SQL goes here
CREATE TABLE encounters (
	
	id uuid NOT NULL PRIMARY KEY,
	START timestamp NOT NULL,
	STOP timestamp,
	PATIENT_ID uuid REFERENCES patients(id),
	ORGANIZATION_ID uuid REFERENCES organizations(id),
	PROVIDER_ID uuid REFERENCES providers(id),
	PAYER_ID uuid REFERENCES payers(id),
	ENCOUNTER_CLASS text,
    -- SNOMED encounter code
	SNOMED_CODE text,
	DESCRIPTION text,
	BASE_ENCOUNTER_COST money,
	TOTAL_CLAIM_COST money,
	PAYER_COVERAGE money,
	REASONCODE text,
	REASON_DESCRIPTION text
)
--s
--s748f8357-6cc7-551d-f31a-32fa2cf84126,
--s2019-02-17T05:07:38Z,
--s2019-02-17T05:22:38Z,
--sb9c610cd-28a6-4636-ccb6-c7a0d2a4cb85,
--sf7ae497d-8dc6-3721-9402-43b621a4e7d2,
--s82608ebb-037c-3cef-9d34-3736d69b29e8,
--s7c4411ce-02f1-39b5-b9ec-dfbea9ad3c1a,
--swellness,
--s410620009,
--sWell child visit (procedure),
--s129.16,
--s877.79,
--s833.90,
--s,
--s
