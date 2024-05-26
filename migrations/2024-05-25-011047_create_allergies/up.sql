-- Your SQL goes here
CREATE TABLE allergies {
	START date,
	STOP date,
	PATIENT TEXT NOT NULL <connect to patient>,
	ENCOUNTER TEXT NOT NULL <connect to encounter>,
	CODE <something not null to something,
	SYSTEM <not sure,
	DESCRIPTION,
	TYPE,
	CATEGORY,
	REACTION1,
	DESCRIPTION1,
	SEVERITY1,
	REACTION2,
	DESCRIPTION2,
	SEVERITY2,
}

-- need to finish filling that out. Need to change reaction, description, severity
-- to hold multiple items. Connected to ENUM?
-- 2020-02-17,,b9c610cd-28a6-4636-ccb6-c7a0d2a4cb85,01efcc52-15d6-51e9-faa2-bee069fcbe44,264287008,Unkno
-- wn,Animal dander (substance),allergy,environment,878820003,Rhinoconjunctivitis (disorder),MODERATE,27
-- 1807003,Eruption of skin (disorder),MILD
