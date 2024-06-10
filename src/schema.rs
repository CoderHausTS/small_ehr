// @generated automatically by Diesel CLI.

diesel::table! {
    allergies (id) {
        id -> Uuid,
        start -> Nullable<Date>,
        stop -> Nullable<Date>,
        patient_id -> Uuid,
        encounter_id -> Uuid,
        code -> Nullable<Text>,
        system -> Nullable<Text>,
        description -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Nullable<Text>,
        category -> Nullable<Text>,
        snomed -> Nullable<Array<Nullable<Text>>>,
    }
}

diesel::table! {
    careplans (id) {
        id -> Uuid,
        start -> Nullable<Date>,
        stop -> Nullable<Date>,
        patient_id -> Nullable<Uuid>,
        encounter_id -> Nullable<Uuid>,
        code -> Nullable<Text>,
        description -> Nullable<Text>,
        reasoncode -> Nullable<Text>,
        reasondescription -> Nullable<Text>,
    }
}

diesel::table! {
    claims (id) {
        id -> Uuid,
        patient_id -> Uuid,
        provider_id -> Uuid,
        primary_patient_insurance_id -> Nullable<Uuid>,
        secondary_patient_insurance_id -> Nullable<Uuid>,
        department_id -> Nullable<Uuid>,
        patient_department_id -> Nullable<Uuid>,
        diagnosis -> Nullable<Array<Nullable<Text>>>,
        referring_provider_id -> Nullable<Uuid>,
        appointment_id -> Nullable<Uuid>,
        current_illness_date -> Timestamp,
        service_date -> Timestamp,
        supervising_provider_id -> Nullable<Uuid>,
        primary_payor_status -> Nullable<Text>,
        secondary_payor_status -> Nullable<Text>,
        patient_claim_status -> Nullable<Text>,
        outstanding_primary_payor_amount -> Nullable<Money>,
        outstanding_secondary_payor_amount -> Nullable<Money>,
        outstanding_patient_amount -> Nullable<Money>,
        last_billed_date_primary_payor -> Nullable<Timestamp>,
        last_billed_date_secondary_payor -> Nullable<Timestamp>,
        last_billed_date_patient -> Nullable<Timestamp>,
        healthcare_claim_type_id -> Nullable<Array<Nullable<Int2>>>,
    }
}

diesel::table! {
    claims_transactions (id) {
        id -> Uuid,
        claim_id -> Uuid,
        charge_id -> Int8,
        patient_id -> Uuid,
        #[sql_name = "type"]
        type_ -> Text,
        amount -> Nullable<Money>,
        payement_method -> Nullable<Text>,
        from_date -> Nullable<Timestamp>,
        to_date -> Nullable<Timestamp>,
        place_of_service_id -> Uuid,
        procedure_code -> Text,
        modifiers -> Nullable<Array<Nullable<Text>>>,
        diagnosis_reference -> Nullable<Array<Nullable<Text>>>,
        units -> Nullable<Int2>,
        department_id -> Nullable<Int2>,
        notes -> Nullable<Text>,
        unit_amount -> Nullable<Money>,
        transfer_out_id -> Nullable<Int2>,
        transfer_type -> Nullable<Text>,
        payments -> Nullable<Money>,
        adjustments -> Nullable<Money>,
        transfers -> Nullable<Money>,
        outstanding -> Nullable<Money>,
        appointment_id -> Nullable<Uuid>,
        linenote -> Nullable<Text>,
        patient_insurance_id -> Nullable<Uuid>,
        fee_schedule_id -> Nullable<Int2>,
        provider_id -> Uuid,
        supervising_provider_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    conditions (id) {
        id -> Uuid,
        start -> Timestamp,
        stop -> Nullable<Timestamp>,
        patient_id -> Uuid,
        encounter_id -> Uuid,
        snomed_code -> Nullable<Text>,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    devices (id) {
        id -> Uuid,
        start -> Timestamp,
        stop -> Nullable<Timestamp>,
        patient_id -> Uuid,
        encounter_id -> Uuid,
        snomed_code -> Nullable<Text>,
        description -> Nullable<Text>,
        udi -> Nullable<Text>,
    }
}

diesel::table! {
    encounters (id) {
        id -> Uuid,
        start -> Timestamp,
        stop -> Nullable<Timestamp>,
        patient_id -> Nullable<Uuid>,
        organization_id -> Nullable<Uuid>,
        provider_id -> Nullable<Uuid>,
        payer_id -> Nullable<Uuid>,
        encounter_class -> Nullable<Text>,
        snomed_code -> Nullable<Text>,
        description -> Nullable<Text>,
        base_encounter_cost -> Nullable<Money>,
        total_claim_cost -> Nullable<Money>,
        payer_coverage -> Nullable<Money>,
        reasoncode -> Nullable<Text>,
        reason_description -> Nullable<Text>,
    }
}

diesel::table! {
    imaging_studies (id) {
        id -> Uuid,
        date -> Timestamp,
        patient_id -> Uuid,
        encounter_id -> Uuid,
        series_uid -> Nullable<Text>,
        bodysite_code -> Nullable<Text>,
        bodysite_description -> Nullable<Text>,
        modality_code -> Nullable<Text>,
        modality_description -> Nullable<Text>,
        instance_uid -> Nullable<Text>,
        sop_code -> Nullable<Text>,
        sop_description -> Nullable<Text>,
        procedure_code -> Nullable<Text>,
    }
}

diesel::table! {
    immunizations (id) {
        id -> Uuid,
        date -> Timestamp,
        patient_id -> Uuid,
        encounter_id -> Uuid,
        snomed_code -> Nullable<Text>,
        description -> Nullable<Text>,
        base_cost -> Nullable<Money>,
    }
}

diesel::table! {
    medications (id) {
        id -> Uuid,
        start -> Timestamp,
        stop -> Nullable<Timestamp>,
        patient_id -> Uuid,
        payer_id -> Uuid,
        encounter_id -> Uuid,
        snomed_code -> Text,
        description -> Text,
        base_cost -> Nullable<Money>,
        payer_coverage -> Nullable<Money>,
        dispenses -> Nullable<Int2>,
        total_cost -> Nullable<Money>,
        reason_code -> Nullable<Text>,
        reason_description -> Nullable<Text>,
    }
}

diesel::table! {
    observations (id) {
        id -> Uuid,
        date -> Timestamp,
        patient_id -> Uuid,
        encounter_id -> Uuid,
        category -> Nullable<Text>,
        code -> Nullable<Text>,
        description -> Nullable<Text>,
        value -> Nullable<Text>,
        units -> Nullable<Text>,
        #[sql_name = "type"]
        type_ -> Nullable<Text>,
    }
}

diesel::table! {
    organizations (id) {
        id -> Uuid,
        name -> Text,
        address -> Nullable<Text>,
        city -> Nullable<Text>,
        state -> Nullable<Text>,
        zip -> Nullable<Text>,
        lat -> Nullable<Text>,
        lon -> Nullable<Text>,
        phone -> Nullable<Text>,
        revenue -> Nullable<Money>,
        utilization -> Nullable<Int8>,
    }
}

diesel::table! {
    patients (id) {
        id -> Uuid,
        birthdate -> Date,
        deathdate -> Nullable<Date>,
        ssn -> Nullable<Text>,
        drivers -> Nullable<Text>,
        passport -> Nullable<Text>,
        prefix -> Nullable<Text>,
        first -> Nullable<Text>,
        last -> Nullable<Text>,
        suffix -> Nullable<Text>,
        maiden -> Nullable<Text>,
        marital -> Nullable<Text>,
        race -> Nullable<Text>,
        ethnicity -> Nullable<Text>,
        gender -> Nullable<Text>,
        birthplace -> Nullable<Text>,
        address -> Nullable<Text>,
        city -> Nullable<Text>,
        state -> Nullable<Text>,
        county -> Nullable<Text>,
        zip -> Nullable<Text>,
        lat -> Nullable<Text>,
        lon -> Nullable<Text>,
        healthcare_expenses -> Nullable<Text>,
        healthcare_coverage -> Nullable<Text>,
    }
}

diesel::table! {
    payer_transitions (id) {
        id -> Uuid,
        patient_id -> Uuid,
        member_id -> Nullable<Uuid>,
        start_year -> Date,
        end_year -> Nullable<Date>,
        payer_id -> Uuid,
        secondary_payer -> Nullable<Uuid>,
        ownership -> Nullable<Text>,
        ownername -> Nullable<Text>,
    }
}

diesel::table! {
    payers (id) {
        id -> Uuid,
        name -> Text,
        address -> Nullable<Text>,
        city -> Nullable<Text>,
        state_headquartered -> Nullable<Text>,
        zip -> Nullable<Text>,
        phone -> Nullable<Text>,
        amount_covered -> Nullable<Money>,
        amount_uncovered -> Nullable<Money>,
        revenue -> Nullable<Money>,
        covered_encounters -> Nullable<Int8>,
        uncovered_encounters -> Nullable<Int8>,
        covered_medications -> Nullable<Int8>,
        uncovered_medications -> Nullable<Int8>,
        covered_procedures -> Nullable<Int8>,
        uncovered_procedures -> Nullable<Int8>,
        covered_immunizations -> Nullable<Int8>,
        uncovered_immunizations -> Nullable<Int8>,
        unique_customers -> Nullable<Int8>,
        qols_avg -> Nullable<Numeric>,
        member_months -> Nullable<Int8>,
    }
}

diesel::table! {
    procedures (id) {
        id -> Uuid,
        start -> Timestamp,
        stop -> Nullable<Timestamp>,
        patient_id -> Uuid,
        encounter_id -> Uuid,
        snomed_code -> Text,
        description -> Text,
        base_cost -> Nullable<Money>,
        reason_code -> Nullable<Text>,
        reason_description -> Nullable<Text>,
    }
}

diesel::table! {
    providers (id) {
        id -> Uuid,
        organization_id -> Uuid,
        name -> Nullable<Text>,
        gender -> Nullable<Text>,
        speciality -> Nullable<Text>,
        address -> Nullable<Text>,
        city -> Nullable<Text>,
        state -> Nullable<Text>,
        zip -> Nullable<Text>,
        lat -> Nullable<Text>,
        lon -> Nullable<Text>,
        utilization -> Nullable<Int2>,
    }
}

diesel::table! {
    supplies (id) {
        id -> Uuid,
        date -> Date,
        patient_id -> Uuid,
        encounter_id -> Uuid,
        snomed_code -> Text,
        description -> Text,
        quantity -> Int2,
    }
}

diesel::joinable!(allergies -> encounters (encounter_id));
diesel::joinable!(allergies -> patients (patient_id));
diesel::joinable!(careplans -> encounters (encounter_id));
diesel::joinable!(careplans -> patients (patient_id));
diesel::joinable!(claims -> encounters (appointment_id));
diesel::joinable!(claims -> patients (patient_id));
diesel::joinable!(claims_transactions -> claims (claim_id));
diesel::joinable!(claims_transactions -> encounters (appointment_id));
diesel::joinable!(claims_transactions -> organizations (place_of_service_id));
diesel::joinable!(claims_transactions -> patients (patient_id));
diesel::joinable!(conditions -> encounters (encounter_id));
diesel::joinable!(conditions -> patients (patient_id));
diesel::joinable!(devices -> encounters (encounter_id));
diesel::joinable!(devices -> patients (patient_id));
diesel::joinable!(encounters -> organizations (organization_id));
diesel::joinable!(encounters -> patients (patient_id));
diesel::joinable!(encounters -> payers (payer_id));
diesel::joinable!(encounters -> providers (provider_id));
diesel::joinable!(imaging_studies -> encounters (encounter_id));
diesel::joinable!(imaging_studies -> patients (patient_id));
diesel::joinable!(immunizations -> encounters (encounter_id));
diesel::joinable!(immunizations -> patients (patient_id));
diesel::joinable!(medications -> encounters (encounter_id));
diesel::joinable!(medications -> patients (patient_id));
diesel::joinable!(medications -> payers (payer_id));
diesel::joinable!(observations -> encounters (encounter_id));
diesel::joinable!(observations -> patients (patient_id));
diesel::joinable!(payer_transitions -> patients (patient_id));
diesel::joinable!(procedures -> encounters (encounter_id));
diesel::joinable!(procedures -> patients (patient_id));
diesel::joinable!(providers -> organizations (organization_id));
diesel::joinable!(supplies -> encounters (encounter_id));
diesel::joinable!(supplies -> patients (patient_id));

diesel::allow_tables_to_appear_in_same_query!(
    allergies,
    careplans,
    claims,
    claims_transactions,
    conditions,
    devices,
    encounters,
    imaging_studies,
    immunizations,
    medications,
    observations,
    organizations,
    patients,
    payer_transitions,
    payers,
    procedures,
    providers,
    supplies,
);
