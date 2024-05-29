-- Your SQL goes here
CREATE TABLE medications (
	id uuid PRIMARY KEY,
	START timestamp NOT NULL,
	STOP timestamp,
	PATIENT_ID uuid NOT NULL REFERENCES patients(id),
	PAYER_ID uuid NOT NULL REFERENCES payers(id),
	ENCOUNTER_ID uuid NOT NULL REFERENCES encounters(id),
	SNOMED_CODE text NOT NULL,
	DESCRIPTION text NOT NULL,
	BASE_COST money,
	PAYER_COVERAGE money,
	DISPENSES smallint,
	TOTAL_COST money,
	REASON_CODE text,
	REASON_DESCRIPTION text
)
--s
--s2020-02-17T10:40:32Z,
--s,
--sb9c610cd-28a6-4636-ccb6-c7a0d2a4cb85,
--s7c4411ce-02f1-39b5-b9ec-dfbea9ad3c1a,
--s01efcc52-15d6-51e9-faa2-bee069fcbe44,
--s1014676,
--scetirizine hydrochloride 5 MG Oral Tablet,
--s21.45,
--s0.00,
--s21,
--s450.45,
--s,
--s
