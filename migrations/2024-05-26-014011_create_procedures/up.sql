-- Your SQL goes here
CREATE TABLE procedures (
	id uuid PRIMARY KEY,
	START timestamp NOT NULL,
	STOP timestamp,
	PATIENT_ID uuid NOT NULL REFERENCES patients(id),
	ENCOUNTER_ID uuid NOT NULL REFERENCES encounters(id),
	SNOMED_CODE text NOT NULL,
	DESCRIPTION text NOT NULL,
	BASE_COST money,
	REASON_CODE text,
	REASON_DESCRIPTION text
)
--s
--s2019-02-17T05:07:38Z,
--s2019-02-17T05:22:38Z,
--sb9c610cd-28a6-4636-ccb6-c7a0d2a4cb85,
--s748f8357-6cc7-551d-f31a-32fa2cf84126,
--s430193006,
--sMedication Reconciliation (procedure),
--s608.11,
--s,
--s
