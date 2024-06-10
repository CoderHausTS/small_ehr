-- Your SQL goes here
CREATE TABLE claims (
	id uuid PRIMARY KEY NOT NULL,
	PATIENT_ID uuid REFERENCES patients(id) NOT NULL,
	PROVIDER_ID uuid REFERENCES providers(id) NOT NULL,
	PRIMARY_PATIENT_INSURANCE_ID uuid REFERENCES payers(id),
	SECONDARY_PATIENT_INSURANCE_ID uuid REFERENCES payers(id),
	DEPARTMENT_ID uuid,
	PATIENT_DEPARTMENT_ID uuid,
    -- snomed code for dx
	DIAGNOSIS text[],
	REFERRING_PROVIDER_ID uuid REFERENCES providers(id),
	APPOINTMENT_ID uuid REFERENCES encounters(id),
	CURRENT_ILLNESS_DATE timestamp NOT NULL,
	SERVICE_DATE timestamp NOT NULL,
	SUPERVISING_PROVIDER_ID uuid REFERENCES providers(id),
	PRIMARY_PAYOR_STATUS text,
	SECONDARY_PAYOR_STATUS text,
	PATIENT_CLAIM_STATUS text,
	OUTSTANDING_PRIMARY_PAYOR_AMOUNT money,
	OUTSTANDING_SECONDARY_PAYOR_AMOUNT money,
	OUTSTANDING_PATIENT_AMOUNT money,
	LAST_BILLED_DATE_PRIMARY_PAYOR timestamp,
	LAST_BILLED_DATE_SECONDARY_PAYOR timestamp,
	LAST_BILLED_DATE_PATIENT timestamp,
	-- 1 is profee, 2 is hospital
	HEALTHCARE_CLAIM_TYPE_ID smallint[]
	
)


-- e413105d-6f23-34ef-724a-b42adab9df22,
-- b9c610cd-28a6-4636-ccb6-c7a0d2a4cb85,
-- 82608ebb-037c-3cef-9d34-3736d69b29e8 
-- 7c4411ce-02f1-39b5-b9ec-dfbea9ad3c1a,
-- 0,
-- 10,
-- 10,
-- 410620009,
-- ,
--,
--,
--,
--,
--,
--,
--,
--748f8357-6cc7-551d-f31a-32fa2cf84126,
--2019-02-17T05:07:38Z,
--2019-02-17T05:07:38Z,
--82608ebb-037c-3cef-9d34-3736d69b29e8,
--CLOSED,
--,
--CLOSED,
--0,
--,
--0,
--2019-02-17T05:22:38Z,
--,
--2019-02-17T05:22:38Z,
--1,
--0
