-- Your SQL goes here
CREATE TABLE conditions (
	id uuid PRIMARY KEY,
	START timestamp NOT NULL, 
	STOP timestamp,
	PATIENT_ID uuid NOT NULL REFERENCES patients(id),
	ENCOUNTER_ID uuid NOT NULL REFERENCES encounters(id),
	SNOMED_CODE text,
	DESCRIPTION text
)
--
--2013-06-24,
--2013-07-02,
--c1f1fcaa-82fd-d5b7-3544-c8f9708b06a8,
--0b2794bd-ec2b-d34f-0610-2523b3b7fcf0,
--10509002,
--Acute bronchitis (disorder)
--
