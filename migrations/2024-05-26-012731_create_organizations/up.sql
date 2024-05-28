-- Your SQL goes here
CREATE TABLE organizations (
	
	id uuid PRIMARY KEY NOT NULL ,
	NAME text NOT NULL,
	ADDRESS text,
	CITY text,
	STATE text,
	ZIP text,
	LAT text,
	LON text,
	PHONE text,
	REVENUE money,
	UTILIZATION bigint
)
--s
--sef58ea08-d883-3957-8300-150554edc8fb,
--sHEALTHALLIANCE HOSPITALS  INC,
--s60 HOSPITAL ROAD,
--sLEOMINSTER,
--sMA,
--s01453,
--s42.520838,
--s-71.770876,
--s9784662000,
--s0.0,
--s1214
