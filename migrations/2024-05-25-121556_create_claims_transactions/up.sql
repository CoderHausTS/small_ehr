-- Your SQL goes here
CREATE TABLE claims_transactions (
	id uuid NOT NULL PRIMARY KEY,
	CLAIM_ID uuid NOT NULL REFERENCES claims(id),
	CHARGE_ID bigint NOT NULL,
	PATIENT_ID uuid NOT NULL REFERENCES patients(id),
	TYPE text NOT NULL,
	AMOUNT money,
	PAYEMENT_METHOD text,
	FROM_DATE timestamp,
	TO_DATE timestamp,
	PLACE_OF_SERVICE uuid NOT NULL REFERENCES organizations(id),
	PROCEDURE_CODE text NOT NULL,
	MODIFIERS text[],
	DIAGNOSIS_REFERENCE text[],
	UNITS smallint,
	DEPARTMENT_ID smallint,
	NOTES text,
	UNIT_AMOUNT money,
	TRANSFER_OUT_ID smallint,
	TRANSFER_TYPE text,
	PAYMENTS money,
	ADJUSTMENTS money,
	TRANSFERS money,
	OUTSTANDING money,
	APPOINTMENT_ID uuid REFERENCES encounters(id),
	LINENOTE text,
	PATIENT_INSURANCE_ID uuid, -- REFERENCES payer_transitions(member_id),
	FEE_SCHEDULE_ID smallint,
	PROVIDER_ID uuid NOT NULL REFERENCES providers(id),
	SUPERVISING_PROVIDER_ID uuid REFERENCES providers(id)
)
-- 
-- 5c58de0b-3ba9-401d-2ce7-a4785ff35c15,
-- e413105d-6f23-34ef-724a-b42adab9df22,
-- 0,
-- d-28a6-4636-ccb6-c7a0d2a4cb85,
-- CHARGE,
-- 129.16,
-- ,
-- 2019-02-17T05:07:38Z,
-- 2019-02-17T05:22:38Z,
-- f7ae497d-8dc6-3721-9402-43b621a4e7d2,
-- 410620009,
-- ,
-- ,
-- 1,
-- ,
-- ,
-- ,
-- 1,
-- 10,
-- Well child visit (procedure),
-- 129.16,
-- ,
-- 1,
-- ,
-- ,
-- ,
-- ,
-- 748f8357-6cc7-551d-f31a-32fa2cf84126,
-- ,
-- ,
-- 1,
-- 82608ebb-037c-3cef-9d34-3736d69b29e8,
-- 82608ebb-037c-3cef-9d34-3736d69b29e8
-- 
