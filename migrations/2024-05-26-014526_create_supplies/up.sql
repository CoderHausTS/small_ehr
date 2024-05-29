-- Your SQL goes here
CREATE TABLE supplies (
	id uuid PRIMARY KEY,
	DATE date NOT NULL,
	PATIENT_ID uuid NOT NULL REFERENCES patients(id) ,
	ENCOUNTER_ID uuid NOT NULL REFERENCES encounters(id),
	SNOMED_CODE text NOT NULL,
	DESCRIPTION text NOT NULL,
	QUANTITY smallint NOT NULL
)
--s
--s2021-02-08,
--s8fa5a097-1970-9370-4193-a7baa3d235b5,
--sb6358095-06f1-cc6a-ac12-3d46550a254a,
--s409534002,
--sDisposable air-purifying respirator (physical object),
--s2
