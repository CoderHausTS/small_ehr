-- Your SQL goes here
CREATE TABLE immunizations (
	id uuid PRIMARY KEY,
	DATE timestamp NOT NULL,
	PATIENT_ID uuid NOT NULL REFERENCES patients(id),
	ENCOUNTER_ID uuid NOT NULL REFERENCES encounters(id),
	SNOMED_CODE text,
	DESCRIPTION text,
	BASE_COST money
)
--s
--s2019-02-17T05:07:38Z,
--sb9c610cd-28a6-4636-ccb6-c7a0d2a4cb85,
--s748f8357-6cc7-551d-f31a-32fa2cf84126,
--s08,
--sHepB  adolescent or pediatric,
--s140.52
