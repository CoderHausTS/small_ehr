-- Your SQL goes here
CREATE TABLE careplans (
	Id uuid PRIMARY KEY NOT NULL,
	START date,
	STOP date,
	PATIENT uuid REFERENCES patients(id),
	ENCOUNTER uuid REFERENCES encounters(id),
    -- snomed-ct
	CODE text,
	DESCRIPTION text,
	REASONCODE text,
	REASONDESCRIPTION text
)

-- 6d10e8ad-cdf8-db60-ff71-688eae2861c2,2020-02-05,,b9c610cd-28a6-4636-ccb6-c7a0d2a4cb85,fb15e123-fea7-c
-- 4bd-ec2b-d34f-0610-2523b3b7fcf0,53950000,Respiratory therapy,10509002,Acute bronchitis (disorder)

