-- Your SQL goes here
CREATE TABLE payer_transitions (
	id uuid PRIMARY KEY,
	PATIENT_ID uuid NOT NULL REFERENCES patients(id),
	MEMBER_ID uuid,
	START_YEAR date NOT NULL,
	END_YEAR date,
	PAYER_ID uuid NOT NULL REFERENCES payers(id),
	SECONDARY_PAYER uuid REFERENCES payers(id),
	OWNERSHIP text,
	OWNERNAME text
)
--s
--sb9c610cd-28a6-4636-ccb6-c7a0d2a4cb85,
--sbca22051-b39b-7591-74d4-47e10a94c52e,
--s2019-02-24T05:07:38Z,
--s2020-03-01T05:07:38Z,
--s7c4411ce-02f1-39b5-b9ec-dfbea9ad3c1a,
--s,
--sSelf,
--sDamon455 Langosh790
