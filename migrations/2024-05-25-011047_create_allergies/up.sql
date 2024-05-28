-- Your SQL goes here
CREATE TABLE allergies (
	START date,
	STOP date,
	PATIENT uuid NOT NULL REFERENCES patients(id),
	ENCOUNTER uuid NOT NULL REFERENCES encounters(id), 
	CODE text,
	SYSTEM text,
	DESCRIPTION text,
	TYPE text,
	CATEGORY text,
    -- reaction (snomed code), description, severity MILD MODERATE SEVERE
	SNOMED text[][][]
)

-- need to finish filling that out. Need to change reaction, description, severity
-- to hold multiple items. Connected to ENUM?
-- 
-- 2020-02-17
-- 
-- b9c610cd-28a6-4636-ccb6-c7a0d2a4cb85
-- 01efcc52-15d6-51e9-faa2-bee069fcbe44
-- 264287008
-- Unkno wn
-- Animal dander (substance)
-- allergy
-- environment
-- 878820003
-- Rhinoconjunctivitis (disorder)
-- MODERATE
-- 27 1807003
-- Eruption of skin (disorder)
-- MILD
