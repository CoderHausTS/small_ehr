-- Your SQL goes here
CREATE TABLE devices (
	START timestamp NOT NULL,
	STOP timestamp,
	PATIENT_ID uuid NOT NULL REFERENCES patients(id),
	ENCOUNTER_ID uuid NOT NULL REFERENCES encounters(id),
	SNOMED_CODE text,
	DESCRIPTION text,
	UDI text
)
--s
--s2017-09-25T17:26:19Z,
--s,
--sd1822991-eb4f-aafe-6a88-cd5279b4385b,
--sbd2f33cd-4a24-6e32-1cd2-bf075ba9ed77,
--s72506001,
--sImplantable defibrillator  device (physical object),
--s(01)32224845419831(11)170904(17)420919(10)707911340(21)5755382
--s
