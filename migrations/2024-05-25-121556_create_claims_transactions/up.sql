-- Your SQL goes here
CREATE TABLE claims_transactions {
	ID,
	CLAIMID,
	CHARGEID,
	PATIENTID,
	TYPE,
	AMOUNT,
	METHOD,
	FROMDATE,
	TODATE,
	PLACEOFSERVICE,
	PROCEDURECODE,
	MODIFIER1,
	
	MODIFIER2,
	DIAGNOSISREF1,
	DIAGNOSISREF2,
	DIAGNOSISREF3,
	DIAGNOSISREF4,
	UNITS,
	DEPARTMENTID,
	NOTES,
	
	UNITAMOUNT,
	TRANSFEROUTID,
	TRANSFERTYPE,
	PAYMENTS,
	ADJUSTMENTS,
	TRANSFERS,
	OUTSTANDING,
	APPOINTMENTID,
	LINENOTE,
	
	PATIENTINSURANCEID,
	FEESCHEDULEID,
	PROVIDERID,
	SUPERVISINGPROVIDERID
}
-- 
-- 5c58de0b-3ba9-401d-2ce7-a4785ff35c15,
-- e413105d-6f23-34ef-724a-b42adab9df22,
-- 0,
-- d-28a6-4636-ccb6-c7a0d2a4cb85,
-- CHARGE,
-- 129.16,
-- ,
-- 2019-02-17T05:07:38Z,
-- 2019-02-17T05:22:38Z,
-- f7ae497d-8dc6-3721-9402-43b621a4e7d2,
-- 410620009,
-- ,
-- ,
-- 1,
-- ,
-- ,
-- ,
-- 1,
-- 10,
-- Well child visit (procedure),
-- 129.16,
-- ,
-- 1,
-- ,
-- ,
-- ,
-- ,
-- 748f8357-6cc7-551d-f31a-32fa2cf84126,
-- ,
-- ,
-- 1,
-- 82608ebb-037c-3cef-9d34-3736d69b29e8,
-- 82608ebb-037c-3cef-9d34-3736d69b29e8
-- 