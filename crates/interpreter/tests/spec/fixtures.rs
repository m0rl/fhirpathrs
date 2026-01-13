use super::*;

thread_local! {
    pub static APPOINTMENT_EXAMPLEREQ: Value =
        Value::object(HashMap::from([
            ("resourceType".to_string(), Value::String("Appointment".to_string())),
            ("id".to_string(), Value::String("examplereq".to_string())),
            ("text".to_string(), Value::object(HashMap::from([
                    ("status".to_string(), Value::String("generated".to_string())),
                ]))),
            ("identifier".to_string(), Value::collection(vec![
                    Value::object(HashMap::from([
                        ("system".to_string(), Value::String("http://example.org/sampleappointment-identifier".to_string())),
                        ("value".to_string(), Value::String("123".to_string())),
                    ])),
                ])),
            ("status".to_string(), Value::String("proposed".to_string())),
            ("serviceCategory".to_string(), Value::collection(vec![
                    Value::object(HashMap::from([
                        ("coding".to_string(), Value::collection(vec![
                                Value::object(HashMap::from([
                                    ("system".to_string(), Value::String("http://example.org/service-category".to_string())),
                                    ("code".to_string(), Value::String("gp".to_string())),
                                    ("display".to_string(), Value::String("General Practice".to_string())),
                                ])),
                            ])),
                    ])),
                ])),
            ("specialty".to_string(), Value::collection(vec![
                    Value::object(HashMap::from([
                        ("coding".to_string(), Value::collection(vec![
                                Value::object(HashMap::from([
                                    ("system".to_string(), Value::String("http://snomed.info/sct".to_string())),
                                    ("code".to_string(), Value::String("394814009".to_string())),
                                    ("display".to_string(), Value::String("General practice".to_string())),
                                ])),
                            ])),
                    ])),
                ])),
            ("appointmentType".to_string(), Value::object(HashMap::from([
                    ("coding".to_string(), Value::collection(vec![
                            Value::object(HashMap::from([
                                ("system".to_string(), Value::String("http://terminology.hl7.org/CodeSystem/v2-0276".to_string())),
                                ("code".to_string(), Value::String("WALKIN".to_string())),
                                ("display".to_string(), Value::String("A previously unscheduled walk-in visit".to_string())),
                            ])),
                        ])),
                ]))),
            ("reason".to_string(), Value::collection(vec![
                    Value::object(HashMap::from([
                        ("concept".to_string(), Value::object(HashMap::from([
                                ("coding".to_string(), Value::collection(vec![
                                        Value::object(HashMap::from([
                                            ("system".to_string(), Value::String("http://snomed.info/sct".to_string())),
                                            ("code".to_string(), Value::String("413095006".to_string())),
                                        ])),
                                    ])),
                                ("text".to_string(), Value::String("Clinical Review".to_string())),
                            ]))),
                    ])),
                ])),
            ("description".to_string(), Value::String("Discussion on the results of your recent MRI".to_string())),
            ("minutesDuration".to_string(), Value::Number(15.0)),
            ("slot".to_string(), Value::collection(vec![
                    Value::object(HashMap::from([
                        ("reference".to_string(), Value::String("Slot/example".to_string())),
                    ])),
                ])),
            ("created".to_string(), Value::from_date_str("2015-12-02").expect("date: 2015-12-02")),
            ("note".to_string(), Value::collection(vec![
                    Value::object(HashMap::from([
                        ("text".to_string(), Value::String("Further expand on the results of the MRI and determine the next actions that may be appropriate.".to_string())),
                    ])),
                ])),
            ("participant".to_string(), Value::collection(vec![
                    Value::object(HashMap::from([
                        ("actor".to_string(), Value::object(HashMap::from([
                                ("reference".to_string(), Value::String("Patient/example".to_string())),
                                ("display".to_string(), Value::String("Peter James Chalmers".to_string())),
                            ]))),
                        ("required".to_string(), Value::Boolean(true)),
                        ("status".to_string(), Value::String("needs-action".to_string())),
                    ])),
                    Value::object(HashMap::from([
                        ("type".to_string(), Value::collection(vec![
                                Value::object(HashMap::from([
                                    ("coding".to_string(), Value::collection(vec![
                                            Value::object(HashMap::from([
                                                ("system".to_string(), Value::String("http://terminology.hl7.org/CodeSystem/v3-ParticipationType".to_string())),
                                                ("code".to_string(), Value::String("ATND".to_string())),
                                            ])),
                                        ])),
                                ])),
                            ])),
                        ("required".to_string(), Value::Boolean(true)),
                        ("status".to_string(), Value::String("needs-action".to_string())),
                    ])),
                    Value::object(HashMap::from([
                        ("actor".to_string(), Value::object(HashMap::from([
                                ("reference".to_string(), Value::String("Location/1".to_string())),
                                ("display".to_string(), Value::String("South Wing, second floor".to_string())),
                            ]))),
                        ("required".to_string(), Value::Boolean(true)),
                        ("status".to_string(), Value::String("accepted".to_string())),
                    ])),
                ])),
            ("requestedPeriod".to_string(), Value::collection(vec![
                    Value::object(HashMap::from([
                        ("start".to_string(), Value::from_date_str("2016-06-02").expect("date: 2016-06-02")),
                        ("end".to_string(), Value::from_date_str("2016-06-09").expect("date: 2016-06-09")),
                    ])),
                ])),
        ]));

    pub static CODESYSTEM_EXAMPLE: Value =
        Value::object(HashMap::from([
            ("resourceType".to_string(), Value::String("CodeSystem".to_string())),
            ("id".to_string(), Value::String("example".to_string())),
            ("meta".to_string(), Value::object(HashMap::from([
                    ("profile".to_string(), Value::collection(vec![
                            Value::String("http://hl7.org/fhir/StructureDefinition/shareablecodesystem".to_string()),
                        ])),
                ]))),
            ("text".to_string(), Value::object(HashMap::from([
                    ("status".to_string(), Value::String("generated".to_string())),
                ]))),
            ("url".to_string(), Value::String("http://hl7.org/fhir/CodeSystem/example".to_string())),
            ("identifier".to_string(), Value::collection(vec![
                    Value::object(HashMap::from([
                        ("system".to_string(), Value::String("http://acme.com/identifiers/codesystems".to_string())),
                        ("value".to_string(), Value::String("internal-cholesterol-inl".to_string())),
                    ])),
                ])),
            ("version".to_string(), Value::String("20160128".to_string())),
            ("name".to_string(), Value::String("ACMECholCodesBlood".to_string())),
            ("title".to_string(), Value::String("ACME Codes for Cholesterol in Serum/Plasma".to_string())),
            ("status".to_string(), Value::String("draft".to_string())),
            ("experimental".to_string(), Value::Boolean(true)),
            ("date".to_string(), Value::from_date_str("2016-01-28").expect("date: 2016-01-28")),
            ("publisher".to_string(), Value::String("Acme Co".to_string())),
            ("contact".to_string(), Value::collection(vec![
                    Value::object(HashMap::from([
                        ("name".to_string(), Value::String("FHIR project team".to_string())),
                        ("telecom".to_string(), Value::collection(vec![
                                Value::object(HashMap::from([
                                    ("system".to_string(), Value::String("url".to_string())),
                                    ("value".to_string(), Value::String("http://hl7.org/fhir".to_string())),
                                ])),
                            ])),
                    ])),
                ])),
            ("description".to_string(), Value::String("This is an example code system that includes all the ACME codes for serum/plasma cholesterol from v2.36.".to_string())),
            ("caseSensitive".to_string(), Value::Boolean(true)),
            ("content".to_string(), Value::String("complete".to_string())),
            ("filter".to_string(), Value::collection(vec![
                    Value::object(HashMap::from([
                        ("code".to_string(), Value::String("acme-plasma".to_string())),
                        ("description".to_string(), Value::String("An internal filter used to select codes that are only used with plasma".to_string())),
                        ("operator".to_string(), Value::collection(vec![
                                Value::String("=".to_string()),
                            ])),
                        ("value".to_string(), Value::String("the value of this filter is either 'true' or 'false'".to_string())),
                    ])),
                ])),
            ("concept".to_string(), Value::collection(vec![
                    Value::object(HashMap::from([
                        ("code".to_string(), Value::String("chol-mass".to_string())),
                        ("display".to_string(), Value::String("SChol (mmol/L)".to_string())),
                        ("definition".to_string(), Value::String("Serum Cholesterol, in mmol/L".to_string())),
                        ("designation".to_string(), Value::collection(vec![
                                Value::object(HashMap::from([
                                    ("use".to_string(), Value::object(HashMap::from([
                                            ("system".to_string(), Value::String("http://acme.com/config/fhir/codesystems/internal".to_string())),
                                            ("code".to_string(), Value::String("internal-label".to_string())),
                                        ]))),
                                    ("value".to_string(), Value::String("From ACME POC Testing".to_string())),
                                ])),
                            ])),
                    ])),
                    Value::object(HashMap::from([
                        ("code".to_string(), Value::String("chol-mass".to_string())),
                        ("display".to_string(), Value::String("SChol (mg/L)".to_string())),
                        ("definition".to_string(), Value::String("Serum Cholesterol, in mg/L".to_string())),
                        ("designation".to_string(), Value::collection(vec![
                                Value::object(HashMap::from([
                                    ("use".to_string(), Value::object(HashMap::from([
                                            ("system".to_string(), Value::String("http://acme.com/config/fhir/codesystems/internal".to_string())),
                                            ("code".to_string(), Value::String("internal-label".to_string())),
                                        ]))),
                                    ("value".to_string(), Value::String("From Paragon Labs".to_string())),
                                ])),
                            ])),
                    ])),
                    Value::object(HashMap::from([
                        ("code".to_string(), Value::String("chol".to_string())),
                        ("display".to_string(), Value::String("SChol".to_string())),
                        ("definition".to_string(), Value::String("Serum Cholesterol".to_string())),
                        ("designation".to_string(), Value::collection(vec![
                                Value::object(HashMap::from([
                                    ("use".to_string(), Value::object(HashMap::from([
                                            ("system".to_string(), Value::String("http://acme.com/config/fhir/codesystems/internal".to_string())),
                                            ("code".to_string(), Value::String("internal-label".to_string())),
                                        ]))),
                                    ("value".to_string(), Value::String("Obdurate Labs uses this with both kinds of units...".to_string())),
                                ])),
                            ])),
                    ])),
                ])),
        ]));

    pub static CONCEPTMAP_EXAMPLE: Value =
        Value::object(HashMap::from([
            ("resourceType".to_string(), Value::String("ConceptMap".to_string())),
            ("id".to_string(), Value::String("101".to_string())),
            ("text".to_string(), Value::object(HashMap::from([
                    ("status".to_string(), Value::String("generated".to_string())),
                ]))),
            ("url".to_string(), Value::String("http://hl7.org/fhir/ConceptMap/101".to_string())),
            ("identifier".to_string(), Value::object(HashMap::from([
                    ("system".to_string(), Value::String("urn:ietf:rfc:3986".to_string())),
                    ("value".to_string(), Value::String("urn:uuid:53cd62ee-033e-414c-9f58-3ca97b5ffc3b".to_string())),
                ]))),
            ("version".to_string(), Value::String("4.0.0".to_string())),
            ("name".to_string(), Value::String("FHIR-v3-Address-Use".to_string())),
            ("title".to_string(), Value::String("FHIR/v3 Address Use Mapping".to_string())),
            ("status".to_string(), Value::String("draft".to_string())),
            ("experimental".to_string(), Value::Boolean(true)),
            ("date".to_string(), Value::from_date_str("2012-06-13").expect("date: 2012-06-13")),
            ("publisher".to_string(), Value::String("HL7, Inc".to_string())),
            ("contact".to_string(), Value::collection(vec![
                    Value::object(HashMap::from([
                        ("name".to_string(), Value::String("FHIR project team (example)".to_string())),
                        ("telecom".to_string(), Value::collection(vec![
                                Value::object(HashMap::from([
                                    ("system".to_string(), Value::String("url".to_string())),
                                    ("value".to_string(), Value::String("http://hl7.org/fhir".to_string())),
                                ])),
                            ])),
                    ])),
                ])),
            ("description".to_string(), Value::String("A mapping between the FHIR and HL7 v3 AddressUse Code systems".to_string())),
            ("useContext".to_string(), Value::collection(vec![
                    Value::object(HashMap::from([
                        ("code".to_string(), Value::object(HashMap::from([
                                ("system".to_string(), Value::String("http://terminology.hl7.org/CodeSystem/usage-context-type".to_string())),
                                ("code".to_string(), Value::String("venue".to_string())),
                            ]))),
                        ("valueCodeableConcept".to_string(), Value::object(HashMap::from([
                                ("text".to_string(), Value::String("for CCDA Usage".to_string())),
                            ]))),
                    ])),
                ])),
            ("jurisdiction".to_string(), Value::collection(vec![
                    Value::object(HashMap::from([
                        ("coding".to_string(), Value::collection(vec![
                                Value::object(HashMap::from([
                                    ("system".to_string(), Value::String("urn:iso:std:iso:3166".to_string())),
                                    ("code".to_string(), Value::String("US".to_string())),
                                ])),
                            ])),
                    ])),
                ])),
            ("purpose".to_string(), Value::String("To help implementers map from HL7 v3/CDA to FHIR".to_string())),
            ("copyright".to_string(), Value::String("Creative Commons 0".to_string())),
            ("sourceUri".to_string(), Value::String("http://hl7.org/fhir/ValueSet/address-use".to_string())),
            ("targetUri".to_string(), Value::String("http://terminology.hl7.org/ValueSet/v3-AddressUse".to_string())),
            ("group".to_string(), Value::collection(vec![
                    Value::object(HashMap::from([
                        ("source".to_string(), Value::String("http://hl7.org/fhir/address-use".to_string())),
                        ("target".to_string(), Value::String("http://terminology.hl7.org/CodeSystem/v3-AddressUse".to_string())),
                        ("element".to_string(), Value::collection(vec![
                                Value::object(HashMap::from([
                                    ("code".to_string(), Value::String("home".to_string())),
                                    ("display".to_string(), Value::String("home".to_string())),
                                    ("target".to_string(), Value::collection(vec![
                                            Value::object(HashMap::from([
                                                ("code".to_string(), Value::String("H".to_string())),
                                                ("display".to_string(), Value::String("home".to_string())),
                                                ("equivalence".to_string(), Value::String("equivalent".to_string())),
                                            ])),
                                        ])),
                                ])),
                                Value::object(HashMap::from([
                                    ("code".to_string(), Value::String("work".to_string())),
                                    ("display".to_string(), Value::String("work".to_string())),
                                    ("target".to_string(), Value::collection(vec![
                                            Value::object(HashMap::from([
                                                ("code".to_string(), Value::String("WP".to_string())),
                                                ("display".to_string(), Value::String("work place".to_string())),
                                                ("equivalence".to_string(), Value::String("equivalent".to_string())),
                                            ])),
                                        ])),
                                ])),
                                Value::object(HashMap::from([
                                    ("code".to_string(), Value::String("temp".to_string())),
                                    ("display".to_string(), Value::String("temp".to_string())),
                                    ("target".to_string(), Value::collection(vec![
                                            Value::object(HashMap::from([
                                                ("code".to_string(), Value::String("TMP".to_string())),
                                                ("display".to_string(), Value::String("temporary address".to_string())),
                                                ("equivalence".to_string(), Value::String("equivalent".to_string())),
                                            ])),
                                        ])),
                                ])),
                                Value::object(HashMap::from([
                                    ("code".to_string(), Value::String("old".to_string())),
                                    ("display".to_string(), Value::String("old".to_string())),
                                    ("target".to_string(), Value::collection(vec![
                                            Value::object(HashMap::from([
                                                ("code".to_string(), Value::String("BAD".to_string())),
                                                ("display".to_string(), Value::String("bad address".to_string())),
                                                ("equivalence".to_string(), Value::String("disjoint".to_string())),
                                                ("comment".to_string(), Value::String("In the HL7 v3 AD, old is handled by the usablePeriod element, but you have to provide a time, there's no simple equivalent of flagging an address as old".to_string())),
                                            ])),
                                        ])),
                                ])),
                            ])),
                        ("unmapped".to_string(), Value::object(HashMap::from([
                                ("mode".to_string(), Value::String("fixed".to_string())),
                                ("code".to_string(), Value::String("temp".to_string())),
                                ("display".to_string(), Value::String("temp".to_string())),
                            ]))),
                    ])),
                ])),
        ]));

    pub static EXPLANATIONOFBENEFIT_EXAMPLE: Value =
        Value::object(HashMap::from([
            ("resourceType".to_string(), Value::String("ExplanationOfBenefit".to_string())),
            ("id".to_string(), Value::String("example".to_string())),
            ("supportingInfo".to_string(), Value::collection(vec![
                    Value::object(HashMap::from([
                        ("sequence".to_string(), Value::Number(1.0)),
                        ("category".to_string(), Value::object(HashMap::from([
                                ("coding".to_string(), Value::collection(vec![
                                        Value::object(HashMap::from([
                                            ("system".to_string(), Value::String("http://hl7.org/fhir/us/carin-bb/CodeSystem/C4BBSupportingInfoType".to_string())),
                                            ("code".to_string(), Value::String("additionalbodysite".to_string())),
                                        ])),
                                    ])),
                            ]))),
                    ])),
                    Value::object(HashMap::from([
                        ("sequence".to_string(), Value::Number(2.0)),
                        ("category".to_string(), Value::object(HashMap::from([
                                ("coding".to_string(), Value::collection(vec![
                                        Value::object(HashMap::from([
                                            ("system".to_string(), Value::String("http://hl7.org/fhir/us/carin-bb/CodeSystem/C4BBSupportingInfoType".to_string())),
                                            ("code".to_string(), Value::String("additionalbodysite".to_string())),
                                        ])),
                                    ])),
                            ]))),
                    ])),
                ])),
            ("item".to_string(), Value::collection(vec![
                    Value::object(HashMap::from([
                        ("sequence".to_string(), Value::Number(1.0)),
                        ("informationSequence".to_string(), Value::collection(vec![
                                Value::Number(2.0),
                            ])),
                    ])),
                    Value::object(HashMap::from([
                        ("sequence".to_string(), Value::Number(2.0)),
                        ("informationSequence".to_string(), Value::collection(vec![
                                Value::Number(1.0),
                            ])),
                    ])),
                ])),
        ]));

    pub static OBSERVATION_EXAMPLE: Value =
        Value::object(HashMap::from([
            ("resourceType".to_string(), Value::String("Observation".to_string())),
            ("id".to_string(), Value::String("example".to_string())),
            ("text".to_string(), Value::object(HashMap::from([
                    ("status".to_string(), Value::String("generated".to_string())),
                ]))),
            ("extension".to_string(), Value::collection(vec![
                    Value::object(HashMap::from([
                        ("url".to_string(), Value::String("http://example.com/fhir/StructureDefinition/patient-age".to_string())),
                        ("valueAge".to_string(), Value::object(HashMap::from([
                                ("value".to_string(), Value::Number(41.0)),
                                ("system".to_string(), Value::String("http://unitsofmeasure.org".to_string())),
                                ("code".to_string(), Value::String("a".to_string())),
                            ]))),
                    ])),
                ])),
            ("status".to_string(), Value::String("final".to_string())),
            ("category".to_string(), Value::collection(vec![
                    Value::object(HashMap::from([
                        ("coding".to_string(), Value::collection(vec![
                                Value::object(HashMap::from([
                                    ("system".to_string(), Value::String("http://terminology.hl7.org/CodeSystem/observation-category".to_string())),
                                    ("code".to_string(), Value::String("vital-signs".to_string())),
                                    ("display".to_string(), Value::String("Vital Signs".to_string())),
                                ])),
                            ])),
                    ])),
                ])),
            ("code".to_string(), Value::object(HashMap::from([
                    ("coding".to_string(), Value::collection(vec![
                            Value::object(HashMap::from([
                                ("system".to_string(), Value::String("http://loinc.org".to_string())),
                                ("code".to_string(), Value::String("29463-7".to_string())),
                                ("display".to_string(), Value::String("Body Weight".to_string())),
                            ])),
                            Value::object(HashMap::from([
                                ("system".to_string(), Value::String("http://loinc.org".to_string())),
                                ("code".to_string(), Value::String("3141-9".to_string())),
                                ("display".to_string(), Value::String("Body weight Measured".to_string())),
                            ])),
                            Value::object(HashMap::from([
                                ("system".to_string(), Value::String("http://snomed.info/sct".to_string())),
                                ("code".to_string(), Value::String("27113001".to_string())),
                                ("display".to_string(), Value::String("Body weight".to_string())),
                            ])),
                            Value::object(HashMap::from([
                                ("system".to_string(), Value::String("http://acme.org/devices/clinical-codes".to_string())),
                                ("code".to_string(), Value::String("body-weight".to_string())),
                                ("display".to_string(), Value::String("Body Weight".to_string())),
                            ])),
                            Value::object(HashMap::from([
                                ("system".to_string(), Value::String("http://i18n.example/rtl-code/".to_string())),
                                ("code".to_string(), Value::String("كتلة-الجسم".to_string())),
                                ("display".to_string(), Value::String("كتلة الجسم".to_string())),
                            ])),
                        ])),
                ]))),
            ("subject".to_string(), Value::object(HashMap::from([
                    ("reference".to_string(), Value::String("Patient/example".to_string())),
                ]))),
            ("encounter".to_string(), Value::object(HashMap::from([
                    ("reference".to_string(), Value::String("Encounter/example".to_string())),
                ]))),
            ("effectiveDateTime".to_string(), Value::from_date_str("2016-03-28").expect("date: 2016-03-28")),
            ("valueQuantity".to_string(), Value::object(HashMap::from([
                    ("value".to_string(), Value::Number(185.0)),
                    ("unit".to_string(), Value::String("lbs".to_string())),
                    ("system".to_string(), Value::String("http://unitsofmeasure.org".to_string())),
                    ("code".to_string(), Value::String("[lb_av]".to_string())),
                ]))),
        ]));

    pub static PARAMETERS_EXAMPLE_TYPES: Value =
        Value::object(HashMap::from([
            ("resourceType".to_string(), Value::String("Parameters".to_string())),
            ("id".to_string(), Value::String("example-types".to_string())),
            ("parameter".to_string(), Value::collection(vec![
                    Value::object(HashMap::from([
                        ("name".to_string(), Value::String("string".to_string())),
                        ("valueString".to_string(), Value::String("string".to_string())),
                    ])),
                    Value::object(HashMap::from([
                        ("name".to_string(), Value::String("integer".to_string())),
                        ("valueInteger".to_string(), Value::Number(1.0)),
                    ])),
                    Value::object(HashMap::from([
                        ("name".to_string(), Value::String("uuid".to_string())),
                        ("valueUuid".to_string(), Value::String("urn:uuid:79a14950-442c-11ed-b878-0242ac120002".to_string())),
                    ])),
                    Value::object(HashMap::from([
                        ("name".to_string(), Value::String("decimal".to_string())),
                        ("valueDecimal".to_string(), Value::Number(1.0)),
                    ])),
                ])),
        ]));

    pub static PATIENT_CONTAINER_EXAMPLE: Value =
        Value::object(HashMap::from([
            ("resourceType".to_string(), Value::String("Patient".to_string())),
            ("id".to_string(), Value::String("example-container".to_string())),
            ("contained".to_string(), Value::collection(vec![
                    Value::object(HashMap::from([
                        ("resourceType".to_string(), Value::String("Organization".to_string())),
                        ("id".to_string(), Value::String("1".to_string())),
                    ])),
                ])),
            ("name".to_string(), Value::collection(vec![
                    Value::object(HashMap::from([
                        ("text".to_string(), Value::String("some-name".to_string())),
                    ])),
                ])),
            ("managingOrganization".to_string(), Value::object(HashMap::from([
                    ("reference".to_string(), Value::String("1".to_string())),
                    ("display".to_string(), Value::String("Gastroenterology".to_string())),
                ]))),
        ]));

    pub static PATIENT_EXAMPLE_NAME: Value =
        Value::object(HashMap::from([
            ("resourceType".to_string(), Value::String("Patient".to_string())),
            ("id".to_string(), Value::String("example".to_string())),
            ("text".to_string(), Value::object(HashMap::from([
                    ("status".to_string(), Value::String("generated".to_string())),
                ]))),
            ("identifier".to_string(), Value::collection(vec![
                    Value::object(HashMap::from([
                        ("use".to_string(), Value::String("usual".to_string())),
                        ("type".to_string(), Value::object(HashMap::from([
                                ("coding".to_string(), Value::collection(vec![
                                        Value::object(HashMap::from([
                                            ("system".to_string(), Value::String("http://terminology.hl7.org/CodeSystem/v2-0203".to_string())),
                                            ("code".to_string(), Value::String("MR".to_string())),
                                        ])),
                                    ])),
                            ]))),
                        ("system".to_string(), Value::String("urn:oid:1.2.36.146.595.217.0.1".to_string())),
                        ("value".to_string(), Value::String("12345".to_string())),
                        ("period".to_string(), Value::object(HashMap::from([
                                ("start".to_string(), Value::from_date_str("2001-05-06").expect("date: 2001-05-06")),
                            ]))),
                        ("assigner".to_string(), Value::object(HashMap::from([
                                ("display".to_string(), Value::String("Acme Healthcare".to_string())),
                            ]))),
                    ])),
                ])),
            ("active".to_string(), Value::Boolean(true)),
            ("name".to_string(), Value::collection(vec![
                    Value::object(HashMap::from([
                        ("use".to_string(), Value::String("official".to_string())),
                        ("text".to_string(), Value::String("Pater J Chalmers".to_string())),
                        ("family".to_string(), Value::String("Chalmers".to_string())),
                        ("given".to_string(), Value::collection(vec![
                                Value::String("Peter".to_string()),
                                Value::String("James".to_string()),
                            ])),
                    ])),
                    Value::object(HashMap::from([
                        ("use".to_string(), Value::String("usual".to_string())),
                        ("given".to_string(), Value::collection(vec![
                                Value::String("Jim".to_string()),
                            ])),
                    ])),
                    Value::object(HashMap::from([
                        ("use".to_string(), Value::String("maiden".to_string())),
                        ("family".to_string(), Value::String("Windsor".to_string())),
                        ("given".to_string(), Value::collection(vec![
                                Value::String("Peter".to_string()),
                                Value::String("James".to_string()),
                            ])),
                        ("period".to_string(), Value::object(HashMap::from([
                                ("end".to_string(), Value::from_date_str("2002").expect("date: 2002")),
                            ]))),
                    ])),
                ])),
            ("telecom".to_string(), Value::collection(vec![
                    Value::object(HashMap::from([
                        ("use".to_string(), Value::String("home".to_string())),
                    ])),
                    Value::object(HashMap::from([
                        ("system".to_string(), Value::String("phone".to_string())),
                        ("value".to_string(), Value::String("(03) 5555 6473".to_string())),
                        ("use".to_string(), Value::String("work".to_string())),
                        ("rank".to_string(), Value::Number(1.0)),
                    ])),
                    Value::object(HashMap::from([
                        ("system".to_string(), Value::String("phone".to_string())),
                        ("value".to_string(), Value::String("(03) 3410 5613".to_string())),
                        ("use".to_string(), Value::String("mobile".to_string())),
                        ("rank".to_string(), Value::Number(2.0)),
                    ])),
                    Value::object(HashMap::from([
                        ("system".to_string(), Value::String("phone".to_string())),
                        ("value".to_string(), Value::String("(03) 5555 8834".to_string())),
                        ("use".to_string(), Value::String("old".to_string())),
                        ("period".to_string(), Value::object(HashMap::from([
                                ("end".to_string(), Value::from_date_str("2014").expect("date: 2014")),
                            ]))),
                    ])),
                ])),
            ("gender".to_string(), Value::String("male".to_string())),
            ("birthDate".to_string(), Value::from_date_str("1974-12-25").expect("date: 1974-12-25")),
            ("_birthDate".to_string(), Value::object(HashMap::from([
                    ("extension".to_string(), Value::collection(vec![
                            Value::object(HashMap::from([
                                ("url".to_string(), Value::String("http://hl7.org/fhir/StructureDefinition/patient-birthTime".to_string())),
                                ("valueDateTime".to_string(), Value::from_datetime_str("1974-12-25T14:35:45-05:00").expect("datetime: 1974-12-25T14:35:45-05:00")),
                            ])),
                        ])),
                ]))),
            ("deceasedBoolean".to_string(), Value::Boolean(false)),
            ("address".to_string(), Value::collection(vec![
                    Value::object(HashMap::from([
                        ("use".to_string(), Value::String("home".to_string())),
                        ("type".to_string(), Value::String("both".to_string())),
                        ("text".to_string(), Value::String("534 Erewhon St PeasantVille, Rainbow, Vic  3999".to_string())),
                        ("line".to_string(), Value::collection(vec![
                                Value::String("534 Erewhon St".to_string()),
                            ])),
                        ("city".to_string(), Value::String("PleasantVille".to_string())),
                        ("district".to_string(), Value::String("Rainbow".to_string())),
                        ("state".to_string(), Value::String("Vic".to_string())),
                        ("postalCode".to_string(), Value::String("3999".to_string())),
                        ("period".to_string(), Value::object(HashMap::from([
                                ("start".to_string(), Value::from_date_str("1974-12-25").expect("date: 1974-12-25")),
                            ]))),
                    ])),
                ])),
            ("contact".to_string(), Value::collection(vec![
                    Value::object(HashMap::from([
                        ("relationship".to_string(), Value::collection(vec![
                                Value::object(HashMap::from([
                                    ("coding".to_string(), Value::collection(vec![
                                            Value::object(HashMap::from([
                                                ("system".to_string(), Value::String("http://terminology.hl7.org/CodeSystem/v2-0131".to_string())),
                                                ("code".to_string(), Value::String("N".to_string())),
                                            ])),
                                        ])),
                                ])),
                            ])),
                        ("name".to_string(), Value::object(HashMap::from([
                                ("family".to_string(), Value::String("du Marché".to_string())),
                                ("_family".to_string(), Value::object(HashMap::from([
                                        ("extension".to_string(), Value::collection(vec![
                                                Value::object(HashMap::from([
                                                    ("url".to_string(), Value::String("http://hl7.org/fhir/StructureDefinition/humanname-own-prefix".to_string())),
                                                    ("valueString".to_string(), Value::String("VV".to_string())),
                                                ])),
                                            ])),
                                    ]))),
                                ("given".to_string(), Value::collection(vec![
                                        Value::String("Bénédicte".to_string()),
                                    ])),
                            ]))),
                        ("telecom".to_string(), Value::collection(vec![
                                Value::object(HashMap::from([
                                    ("system".to_string(), Value::String("phone".to_string())),
                                    ("value".to_string(), Value::String("+33 (237) 998327".to_string())),
                                ])),
                            ])),
                        ("address".to_string(), Value::object(HashMap::from([
                                ("use".to_string(), Value::String("home".to_string())),
                                ("type".to_string(), Value::String("both".to_string())),
                                ("line".to_string(), Value::collection(vec![
                                        Value::String("534 Erewhon St".to_string()),
                                    ])),
                                ("city".to_string(), Value::String("PleasantVille".to_string())),
                                ("district".to_string(), Value::String("Rainbow".to_string())),
                                ("state".to_string(), Value::String("Vic".to_string())),
                                ("postalCode".to_string(), Value::String("3999".to_string())),
                                ("period".to_string(), Value::object(HashMap::from([
                                        ("start".to_string(), Value::from_date_str("1974-12-25").expect("date: 1974-12-25")),
                                    ]))),
                            ]))),
                        ("gender".to_string(), Value::String("female".to_string())),
                        ("period".to_string(), Value::object(HashMap::from([
                                ("start".to_string(), Value::from_date_str("2012").expect("date: 2012")),
                            ]))),
                    ])),
                ])),
            ("managingOrganization".to_string(), Value::object(HashMap::from([
                    ("reference".to_string(), Value::String("Organization/1".to_string())),
                ]))),
        ]));

    pub static PATIENT_EXAMPLE_PERIOD: Value =
        Value::object(HashMap::from([
            ("resourceType".to_string(), Value::String("Patient".to_string())),
            ("id".to_string(), Value::String("example".to_string())),
            ("text".to_string(), Value::object(HashMap::from([
                    ("status".to_string(), Value::String("generated".to_string())),
                ]))),
            ("identifier".to_string(), Value::collection(vec![
                    Value::object(HashMap::from([
                        ("use".to_string(), Value::String("usual".to_string())),
                        ("type".to_string(), Value::object(HashMap::from([
                                ("coding".to_string(), Value::collection(vec![
                                        Value::object(HashMap::from([
                                            ("system".to_string(), Value::String("http://terminology.hl7.org/CodeSystem/v2-0203".to_string())),
                                            ("code".to_string(), Value::String("MR".to_string())),
                                        ])),
                                    ])),
                            ]))),
                        ("system".to_string(), Value::String("urn:oid:1.2.36.146.595.217.0.1".to_string())),
                        ("value".to_string(), Value::String("12345".to_string())),
                        ("period".to_string(), Value::object(HashMap::from([
                                ("start".to_string(), Value::from_date_str("2001-05-06").expect("date: 2001-05-06")),
                                ("end".to_string(), Value::from_datetime_str("2001-05-06T10:10:10Z").expect("datetime: 2001-05-06T10:10:10Z")),
                            ]))),
                        ("assigner".to_string(), Value::object(HashMap::from([
                                ("display".to_string(), Value::String("Acme Healthcare".to_string())),
                            ]))),
                    ])),
                ])),
            ("active".to_string(), Value::Boolean(true)),
            ("name".to_string(), Value::collection(vec![
                    Value::object(HashMap::from([
                        ("use".to_string(), Value::String("official".to_string())),
                        ("family".to_string(), Value::String("Chalmers".to_string())),
                        ("given".to_string(), Value::collection(vec![
                                Value::String("Peter".to_string()),
                                Value::String("James".to_string()),
                            ])),
                    ])),
                    Value::object(HashMap::from([
                        ("use".to_string(), Value::String("usual".to_string())),
                        ("given".to_string(), Value::collection(vec![
                                Value::String("Jim".to_string()),
                            ])),
                    ])),
                    Value::object(HashMap::from([
                        ("use".to_string(), Value::String("maiden".to_string())),
                        ("family".to_string(), Value::String("Windsor".to_string())),
                        ("given".to_string(), Value::collection(vec![
                                Value::String("Peter".to_string()),
                                Value::String("James".to_string()),
                            ])),
                        ("period".to_string(), Value::object(HashMap::from([
                                ("end".to_string(), Value::from_date_str("2002").expect("date: 2002")),
                            ]))),
                    ])),
                ])),
            ("telecom".to_string(), Value::collection(vec![
                    Value::object(HashMap::from([
                        ("use".to_string(), Value::String("home".to_string())),
                    ])),
                    Value::object(HashMap::from([
                        ("system".to_string(), Value::String("phone".to_string())),
                        ("value".to_string(), Value::String("(03) 5555 6473".to_string())),
                        ("use".to_string(), Value::String("work".to_string())),
                        ("rank".to_string(), Value::Number(1.0)),
                    ])),
                    Value::object(HashMap::from([
                        ("system".to_string(), Value::String("phone".to_string())),
                        ("value".to_string(), Value::String("(03) 3410 5613".to_string())),
                        ("use".to_string(), Value::String("mobile".to_string())),
                        ("rank".to_string(), Value::Number(2.0)),
                    ])),
                    Value::object(HashMap::from([
                        ("system".to_string(), Value::String("phone".to_string())),
                        ("value".to_string(), Value::String("(03) 5555 8834".to_string())),
                        ("use".to_string(), Value::String("old".to_string())),
                        ("period".to_string(), Value::object(HashMap::from([
                                ("end".to_string(), Value::from_date_str("2014").expect("date: 2014")),
                            ]))),
                    ])),
                ])),
            ("gender".to_string(), Value::String("male".to_string())),
            ("birthDate".to_string(), Value::from_date_str("1974-12-25").expect("date: 1974-12-25")),
            ("_birthDate".to_string(), Value::object(HashMap::from([
                    ("extension".to_string(), Value::collection(vec![
                            Value::object(HashMap::from([
                                ("url".to_string(), Value::String("http://hl7.org/fhir/StructureDefinition/patient-birthTime".to_string())),
                                ("valueDateTime".to_string(), Value::from_datetime_str("1974-12-25T14:35:45-05:00").expect("datetime: 1974-12-25T14:35:45-05:00")),
                            ])),
                        ])),
                ]))),
            ("deceasedBoolean".to_string(), Value::Boolean(false)),
            ("address".to_string(), Value::collection(vec![
                    Value::object(HashMap::from([
                        ("use".to_string(), Value::String("home".to_string())),
                        ("type".to_string(), Value::String("both".to_string())),
                        ("text".to_string(), Value::String("534 Erewhon St PeasantVille, Rainbow, Vic  3999".to_string())),
                        ("line".to_string(), Value::collection(vec![
                                Value::String("534 Erewhon St".to_string()),
                            ])),
                        ("city".to_string(), Value::String("PleasantVille".to_string())),
                        ("district".to_string(), Value::String("Rainbow".to_string())),
                        ("state".to_string(), Value::String("Vic".to_string())),
                        ("postalCode".to_string(), Value::String("3999".to_string())),
                        ("period".to_string(), Value::object(HashMap::from([
                                ("start".to_string(), Value::from_date_str("1974-12-25").expect("date: 1974-12-25")),
                            ]))),
                    ])),
                ])),
            ("contact".to_string(), Value::collection(vec![
                    Value::object(HashMap::from([
                        ("relationship".to_string(), Value::collection(vec![
                                Value::object(HashMap::from([
                                    ("coding".to_string(), Value::collection(vec![
                                            Value::object(HashMap::from([
                                                ("system".to_string(), Value::String("http://terminology.hl7.org/CodeSystem/v2-0131".to_string())),
                                                ("code".to_string(), Value::String("N".to_string())),
                                            ])),
                                        ])),
                                ])),
                            ])),
                        ("name".to_string(), Value::object(HashMap::from([
                                ("family".to_string(), Value::String("du Marché".to_string())),
                                ("_family".to_string(), Value::object(HashMap::from([
                                        ("extension".to_string(), Value::collection(vec![
                                                Value::object(HashMap::from([
                                                    ("url".to_string(), Value::String("http://hl7.org/fhir/StructureDefinition/humanname-own-prefix".to_string())),
                                                    ("valueString".to_string(), Value::String("VV".to_string())),
                                                ])),
                                            ])),
                                    ]))),
                                ("given".to_string(), Value::collection(vec![
                                        Value::String("Bénédicte".to_string()),
                                    ])),
                            ]))),
                        ("telecom".to_string(), Value::collection(vec![
                                Value::object(HashMap::from([
                                    ("system".to_string(), Value::String("phone".to_string())),
                                    ("value".to_string(), Value::String("+33 (237) 998327".to_string())),
                                ])),
                            ])),
                        ("address".to_string(), Value::object(HashMap::from([
                                ("use".to_string(), Value::String("home".to_string())),
                                ("type".to_string(), Value::String("both".to_string())),
                                ("line".to_string(), Value::collection(vec![
                                        Value::String("534 Erewhon St".to_string()),
                                    ])),
                                ("city".to_string(), Value::String("PleasantVille".to_string())),
                                ("district".to_string(), Value::String("Rainbow".to_string())),
                                ("state".to_string(), Value::String("Vic".to_string())),
                                ("postalCode".to_string(), Value::String("3999".to_string())),
                                ("period".to_string(), Value::object(HashMap::from([
                                        ("start".to_string(), Value::from_date_str("1974-12-25").expect("date: 1974-12-25")),
                                    ]))),
                            ]))),
                        ("gender".to_string(), Value::String("female".to_string())),
                        ("period".to_string(), Value::object(HashMap::from([
                                ("start".to_string(), Value::from_date_str("2012").expect("date: 2012")),
                            ]))),
                    ])),
                ])),
            ("managingOrganization".to_string(), Value::object(HashMap::from([
                    ("reference".to_string(), Value::String("Organization/1".to_string())),
                ]))),
        ]));

    pub static PATIENT_EXAMPLE: Value =
        Value::object(HashMap::from([
            ("resourceType".to_string(), Value::String("Patient".to_string())),
            ("id".to_string(), Value::String("example".to_string())),
            ("text".to_string(), Value::object(HashMap::from([
                    ("status".to_string(), Value::String("generated".to_string())),
                ]))),
            ("identifier".to_string(), Value::collection(vec![
                    Value::object(HashMap::from([
                        ("use".to_string(), Value::String("usual".to_string())),
                        ("type".to_string(), Value::object(HashMap::from([
                                ("coding".to_string(), Value::collection(vec![
                                        Value::object(HashMap::from([
                                            ("system".to_string(), Value::String("http://terminology.hl7.org/CodeSystem/v2-0203".to_string())),
                                            ("code".to_string(), Value::String("MR".to_string())),
                                        ])),
                                    ])),
                            ]))),
                        ("system".to_string(), Value::String("urn:oid:1.2.36.146.595.217.0.1".to_string())),
                        ("value".to_string(), Value::String("12345".to_string())),
                        ("period".to_string(), Value::object(HashMap::from([
                                ("start".to_string(), Value::from_date_str("2001-05-06").expect("date: 2001-05-06")),
                            ]))),
                        ("assigner".to_string(), Value::object(HashMap::from([
                                ("display".to_string(), Value::String("Acme Healthcare".to_string())),
                            ]))),
                    ])),
                ])),
            ("active".to_string(), Value::Boolean(true)),
            ("name".to_string(), Value::collection(vec![
                    Value::object(HashMap::from([
                        ("use".to_string(), Value::String("official".to_string())),
                        ("family".to_string(), Value::String("Chalmers".to_string())),
                        ("given".to_string(), Value::collection(vec![
                                Value::String("Peter".to_string()),
                                Value::String("James".to_string()),
                            ])),
                    ])),
                    Value::object(HashMap::from([
                        ("use".to_string(), Value::String("usual".to_string())),
                        ("given".to_string(), Value::collection(vec![
                                Value::String("Jim".to_string()),
                            ])),
                    ])),
                    Value::object(HashMap::from([
                        ("use".to_string(), Value::String("maiden".to_string())),
                        ("family".to_string(), Value::String("Windsor".to_string())),
                        ("given".to_string(), Value::collection(vec![
                                Value::String("Peter".to_string()),
                                Value::String("James".to_string()),
                            ])),
                        ("period".to_string(), Value::object(HashMap::from([
                                ("end".to_string(), Value::from_date_str("2002").expect("date: 2002")),
                            ]))),
                    ])),
                ])),
            ("telecom".to_string(), Value::collection(vec![
                    Value::object(HashMap::from([
                        ("use".to_string(), Value::String("home".to_string())),
                    ])),
                    Value::object(HashMap::from([
                        ("system".to_string(), Value::String("phone".to_string())),
                        ("value".to_string(), Value::String("(03) 5555 6473".to_string())),
                        ("use".to_string(), Value::String("work".to_string())),
                        ("rank".to_string(), Value::Number(1.0)),
                    ])),
                    Value::object(HashMap::from([
                        ("system".to_string(), Value::String("phone".to_string())),
                        ("value".to_string(), Value::String("(03) 3410 5613".to_string())),
                        ("use".to_string(), Value::String("mobile".to_string())),
                        ("rank".to_string(), Value::Number(2.0)),
                    ])),
                    Value::object(HashMap::from([
                        ("system".to_string(), Value::String("phone".to_string())),
                        ("value".to_string(), Value::String("(03) 5555 8834".to_string())),
                        ("use".to_string(), Value::String("old".to_string())),
                        ("period".to_string(), Value::object(HashMap::from([
                                ("end".to_string(), Value::from_date_str("2014").expect("date: 2014")),
                            ]))),
                    ])),
                ])),
            ("gender".to_string(), Value::String("male".to_string())),
            ("birthDate".to_string(), Value::from_date_str("1974-12-25").expect("date: 1974-12-25")),
            ("_birthDate".to_string(), Value::object(HashMap::from([
                    ("extension".to_string(), Value::collection(vec![
                            Value::object(HashMap::from([
                                ("url".to_string(), Value::String("http://hl7.org/fhir/StructureDefinition/patient-birthTime".to_string())),
                                ("valueDateTime".to_string(), Value::from_datetime_str("1974-12-25T14:35:45-05:00").expect("datetime: 1974-12-25T14:35:45-05:00")),
                            ])),
                        ])),
                ]))),
            ("deceasedBoolean".to_string(), Value::Boolean(false)),
            ("address".to_string(), Value::collection(vec![
                    Value::object(HashMap::from([
                        ("use".to_string(), Value::String("home".to_string())),
                        ("type".to_string(), Value::String("both".to_string())),
                        ("text".to_string(), Value::String("534 Erewhon St PeasantVille, Rainbow, Vic  3999".to_string())),
                        ("line".to_string(), Value::collection(vec![
                                Value::String("534 Erewhon St".to_string()),
                            ])),
                        ("city".to_string(), Value::String("PleasantVille".to_string())),
                        ("district".to_string(), Value::String("Rainbow".to_string())),
                        ("state".to_string(), Value::String("Vic".to_string())),
                        ("postalCode".to_string(), Value::String("3999".to_string())),
                        ("period".to_string(), Value::object(HashMap::from([
                                ("start".to_string(), Value::from_date_str("1974-12-25").expect("date: 1974-12-25")),
                            ]))),
                    ])),
                ])),
            ("contact".to_string(), Value::collection(vec![
                    Value::object(HashMap::from([
                        ("relationship".to_string(), Value::collection(vec![
                                Value::object(HashMap::from([
                                    ("coding".to_string(), Value::collection(vec![
                                            Value::object(HashMap::from([
                                                ("system".to_string(), Value::String("http://terminology.hl7.org/CodeSystem/v2-0131".to_string())),
                                                ("code".to_string(), Value::String("N".to_string())),
                                            ])),
                                        ])),
                                ])),
                            ])),
                        ("name".to_string(), Value::object(HashMap::from([
                                ("family".to_string(), Value::String("du Marché".to_string())),
                                ("_family".to_string(), Value::object(HashMap::from([
                                        ("extension".to_string(), Value::collection(vec![
                                                Value::object(HashMap::from([
                                                    ("url".to_string(), Value::String("http://hl7.org/fhir/StructureDefinition/humanname-own-prefix".to_string())),
                                                    ("valueString".to_string(), Value::String("VV".to_string())),
                                                ])),
                                            ])),
                                    ]))),
                                ("given".to_string(), Value::collection(vec![
                                        Value::String("Bénédicte".to_string()),
                                    ])),
                            ]))),
                        ("telecom".to_string(), Value::collection(vec![
                                Value::object(HashMap::from([
                                    ("system".to_string(), Value::String("phone".to_string())),
                                    ("value".to_string(), Value::String("+33 (237) 998327".to_string())),
                                ])),
                            ])),
                        ("address".to_string(), Value::object(HashMap::from([
                                ("use".to_string(), Value::String("home".to_string())),
                                ("type".to_string(), Value::String("both".to_string())),
                                ("line".to_string(), Value::collection(vec![
                                        Value::String("534 Erewhon St".to_string()),
                                    ])),
                                ("city".to_string(), Value::String("PleasantVille".to_string())),
                                ("district".to_string(), Value::String("Rainbow".to_string())),
                                ("state".to_string(), Value::String("Vic".to_string())),
                                ("postalCode".to_string(), Value::String("3999".to_string())),
                                ("period".to_string(), Value::object(HashMap::from([
                                        ("start".to_string(), Value::from_date_str("1974-12-25").expect("date: 1974-12-25")),
                                    ]))),
                            ]))),
                        ("gender".to_string(), Value::String("female".to_string())),
                        ("period".to_string(), Value::object(HashMap::from([
                                ("start".to_string(), Value::from_date_str("2012").expect("date: 2012")),
                            ]))),
                    ])),
                ])),
            ("managingOrganization".to_string(), Value::object(HashMap::from([
                    ("reference".to_string(), Value::String("Organization/1".to_string())),
                ]))),
        ]));

    pub static QUESTIONNAIRE_EXAMPLE: Value =
        Value::object(HashMap::from([
            ("resourceType".to_string(), Value::String("Questionnaire".to_string())),
            ("id".to_string(), Value::String("3141".to_string())),
            ("text".to_string(), Value::object(HashMap::from([
                    ("status".to_string(), Value::String("generated".to_string())),
                ]))),
            ("url".to_string(), Value::String("http://hl7.org/fhir/Questionnaire/3141".to_string())),
            ("title".to_string(), Value::String("Cancer Quality Forum Questionnaire 2012".to_string())),
            ("status".to_string(), Value::String("draft".to_string())),
            ("subjectType".to_string(), Value::collection(vec![
                    Value::String("Patient".to_string()),
                ])),
            ("date".to_string(), Value::from_date_str("2012-01").expect("date: 2012-01")),
            ("item".to_string(), Value::collection(vec![
                    Value::object(HashMap::from([
                        ("linkId".to_string(), Value::String("1".to_string())),
                        ("code".to_string(), Value::collection(vec![
                                Value::object(HashMap::from([
                                    ("system".to_string(), Value::String("http://example.org/system/code/sections".to_string())),
                                    ("code".to_string(), Value::String("COMORBIDITY".to_string())),
                                ])),
                            ])),
                        ("type".to_string(), Value::String("group".to_string())),
                        ("item".to_string(), Value::collection(vec![
                                Value::object(HashMap::from([
                                    ("linkId".to_string(), Value::String("1.1".to_string())),
                                    ("code".to_string(), Value::collection(vec![
                                            Value::object(HashMap::from([
                                                ("system".to_string(), Value::String("http://example.org/system/code/questions".to_string())),
                                                ("code".to_string(), Value::String("COMORB".to_string())),
                                            ])),
                                        ])),
                                    ("prefix".to_string(), Value::String("1".to_string())),
                                    ("type".to_string(), Value::String("choice".to_string())),
                                    ("answerValueSet".to_string(), Value::String("http://hl7.org/fhir/ValueSet/yesnodontknow".to_string())),
                                    ("item".to_string(), Value::collection(vec![
                                            Value::object(HashMap::from([
                                                ("linkId".to_string(), Value::String("1.1.1".to_string())),
                                                ("code".to_string(), Value::collection(vec![
                                                        Value::object(HashMap::from([
                                                            ("system".to_string(), Value::String("http://example.org/system/code/sections".to_string())),
                                                            ("code".to_string(), Value::String("CARDIAL".to_string())),
                                                        ])),
                                                    ])),
                                                ("type".to_string(), Value::String("group".to_string())),
                                                ("enableWhen".to_string(), Value::collection(vec![
                                                        Value::object(HashMap::from([
                                                            ("question".to_string(), Value::String("1.1".to_string())),
                                                            ("operator".to_string(), Value::String("=".to_string())),
                                                            ("answerCoding".to_string(), Value::object(HashMap::from([
                                                                    ("system".to_string(), Value::String("http://terminology.hl7.org/CodeSystem/v2-0136".to_string())),
                                                                    ("code".to_string(), Value::String("Y".to_string())),
                                                                ]))),
                                                        ])),
                                                    ])),
                                                ("item".to_string(), Value::collection(vec![
                                                        Value::object(HashMap::from([
                                                            ("linkId".to_string(), Value::String("1.1.1.1".to_string())),
                                                            ("code".to_string(), Value::collection(vec![
                                                                    Value::object(HashMap::from([
                                                                        ("system".to_string(), Value::String("http://example.org/system/code/questions".to_string())),
                                                                        ("code".to_string(), Value::String("COMORBCAR".to_string())),
                                                                    ])),
                                                                ])),
                                                            ("prefix".to_string(), Value::String("1.1".to_string())),
                                                            ("type".to_string(), Value::String("choice".to_string())),
                                                            ("answerValueSet".to_string(), Value::String("http://hl7.org/fhir/ValueSet/yesnodontknow".to_string())),
                                                            ("item".to_string(), Value::collection(vec![
                                                                    Value::object(HashMap::from([
                                                                        ("linkId".to_string(), Value::String("1.1.1.1.1".to_string())),
                                                                        ("code".to_string(), Value::collection(vec![
                                                                                Value::object(HashMap::from([
                                                                                    ("system".to_string(), Value::String("http://example.org/system/code/questions".to_string())),
                                                                                    ("code".to_string(), Value::String("COMCAR00".to_string())),
                                                                                    ("display".to_string(), Value::String("Angina Pectoris".to_string())),
                                                                                ])),
                                                                                Value::object(HashMap::from([
                                                                                    ("system".to_string(), Value::String("http://snomed.info/sct".to_string())),
                                                                                    ("code".to_string(), Value::String("194828000".to_string())),
                                                                                    ("display".to_string(), Value::String("Angina (disorder)".to_string())),
                                                                                ])),
                                                                            ])),
                                                                        ("prefix".to_string(), Value::String("1.1.1".to_string())),
                                                                        ("type".to_string(), Value::String("choice".to_string())),
                                                                        ("answerValueSet".to_string(), Value::String("http://hl7.org/fhir/ValueSet/yesnodontknow".to_string())),
                                                                    ])),
                                                                    Value::object(HashMap::from([
                                                                        ("linkId".to_string(), Value::String("1.1.1.1.2".to_string())),
                                                                        ("code".to_string(), Value::collection(vec![
                                                                                Value::object(HashMap::from([
                                                                                    ("system".to_string(), Value::String("http://snomed.info/sct".to_string())),
                                                                                    ("code".to_string(), Value::String("22298006".to_string())),
                                                                                    ("display".to_string(), Value::String("Myocardial infarction (disorder)".to_string())),
                                                                                ])),
                                                                            ])),
                                                                        ("prefix".to_string(), Value::String("1.1.2".to_string())),
                                                                        ("type".to_string(), Value::String("choice".to_string())),
                                                                        ("answerValueSet".to_string(), Value::String("http://hl7.org/fhir/ValueSet/yesnodontknow".to_string())),
                                                                    ])),
                                                                ])),
                                                        ])),
                                                        Value::object(HashMap::from([
                                                            ("linkId".to_string(), Value::String("1.1.1.2".to_string())),
                                                            ("code".to_string(), Value::collection(vec![
                                                                    Value::object(HashMap::from([
                                                                        ("system".to_string(), Value::String("http://example.org/system/code/questions".to_string())),
                                                                        ("code".to_string(), Value::String("COMORBVAS".to_string())),
                                                                    ])),
                                                                ])),
                                                            ("prefix".to_string(), Value::String("1.2".to_string())),
                                                            ("type".to_string(), Value::String("choice".to_string())),
                                                            ("answerValueSet".to_string(), Value::String("http://hl7.org/fhir/ValueSet/yesnodontknow".to_string())),
                                                        ])),
                                                    ])),
                                            ])),
                                        ])),
                                ])),
                            ])),
                    ])),
                    Value::object(HashMap::from([
                        ("linkId".to_string(), Value::String("2".to_string())),
                        ("code".to_string(), Value::collection(vec![
                                Value::object(HashMap::from([
                                    ("system".to_string(), Value::String("http://example.org/system/code/sections".to_string())),
                                    ("code".to_string(), Value::String("HISTOPATHOLOGY".to_string())),
                                ])),
                            ])),
                        ("type".to_string(), Value::String("group".to_string())),
                        ("item".to_string(), Value::collection(vec![
                                Value::object(HashMap::from([
                                    ("linkId".to_string(), Value::String("2.1".to_string())),
                                    ("code".to_string(), Value::collection(vec![
                                            Value::object(HashMap::from([
                                                ("system".to_string(), Value::String("http://example.org/system/code/sections".to_string())),
                                                ("code".to_string(), Value::String("ABDOMINAL".to_string())),
                                            ])),
                                        ])),
                                    ("type".to_string(), Value::String("group".to_string())),
                                    ("item".to_string(), Value::collection(vec![
                                            Value::object(HashMap::from([
                                                ("linkId".to_string(), Value::String("2.1.2".to_string())),
                                                ("code".to_string(), Value::collection(vec![
                                                        Value::object(HashMap::from([
                                                            ("system".to_string(), Value::String("http://example.org/system/code/questions".to_string())),
                                                            ("code".to_string(), Value::String("STADPT".to_string())),
                                                            ("display".to_string(), Value::String("pT category".to_string())),
                                                        ])),
                                                    ])),
                                                ("type".to_string(), Value::String("choice".to_string())),
                                            ])),
                                        ])),
                                ])),
                            ])),
                    ])),
                ])),
        ]));

    pub static VALUESET_EXAMPLE_EXPANSION: Value =
        Value::object(HashMap::from([
            ("resourceType".to_string(), Value::String("ValueSet".to_string())),
            ("id".to_string(), Value::String("example-expansion".to_string())),
            ("meta".to_string(), Value::object(HashMap::from([
                    ("profile".to_string(), Value::collection(vec![
                            Value::String("http://hl7.org/fhir/StructureDefinition/shareablevalueset".to_string()),
                        ])),
                ]))),
            ("text".to_string(), Value::object(HashMap::from([
                    ("status".to_string(), Value::String("generated".to_string())),
                ]))),
            ("url".to_string(), Value::String("http://hl7.org/fhir/ValueSet/example-expansion".to_string())),
            ("version".to_string(), Value::String("20150622".to_string())),
            ("name".to_string(), Value::String("LOINC Codes for Cholesterol in Serum/Plasma".to_string())),
            ("status".to_string(), Value::String("draft".to_string())),
            ("experimental".to_string(), Value::Boolean(true)),
            ("date".to_string(), Value::from_date_str("2015-06-22").expect("date: 2015-06-22")),
            ("publisher".to_string(), Value::String("FHIR Project team".to_string())),
            ("contact".to_string(), Value::collection(vec![
                    Value::object(HashMap::from([
                        ("telecom".to_string(), Value::collection(vec![
                                Value::object(HashMap::from([
                                    ("system".to_string(), Value::String("url".to_string())),
                                    ("value".to_string(), Value::String("http://hl7.org/fhir".to_string())),
                                ])),
                            ])),
                    ])),
                ])),
            ("description".to_string(), Value::String("This is an example value set that includes all the LOINC codes for serum/plasma cholesterol from v2.36.".to_string())),
            ("copyright".to_string(), Value::String("This content from LOINC® is copyright © 1995 Regenstrief Institute, Inc. and the LOINC Committee, and available at no cost under the license at http://loinc.org/terms-of-use.".to_string())),
            ("compose".to_string(), Value::object(HashMap::from([
                    ("include".to_string(), Value::collection(vec![
                            Value::object(HashMap::from([
                                ("system".to_string(), Value::String("http://loinc.org".to_string())),
                                ("filter".to_string(), Value::collection(vec![
                                        Value::object(HashMap::from([
                                            ("property".to_string(), Value::String("parent".to_string())),
                                            ("op".to_string(), Value::String("=".to_string())),
                                            ("value".to_string(), Value::String("LP43571-6".to_string())),
                                        ])),
                                    ])),
                            ])),
                        ])),
                ]))),
            ("expansion".to_string(), Value::object(HashMap::from([
                    ("extension".to_string(), Value::collection(vec![
                            Value::object(HashMap::from([
                                ("url".to_string(), Value::String("http://hl7.org/fhir/StructureDefinition/valueset-expansionSource".to_string())),
                                ("valueUri".to_string(), Value::String("http://hl7.org/fhir/ValueSet/example-extensional".to_string())),
                            ])),
                        ])),
                    ("identifier".to_string(), Value::String("urn:uuid:42316ff8-2714-4680-9980-f37a6d1a71bc".to_string())),
                    ("timestamp".to_string(), Value::from_datetime_str("2015-06-22T13:56:07Z").expect("datetime: 2015-06-22T13:56:07Z")),
                    ("total".to_string(), Value::Number(8.0)),
                    ("offset".to_string(), Value::Number(0.0)),
                    ("parameter".to_string(), Value::collection(vec![
                            Value::object(HashMap::from([
                                ("name".to_string(), Value::String("version".to_string())),
                                ("valueString".to_string(), Value::String("2.50".to_string())),
                            ])),
                        ])),
                    ("contains".to_string(), Value::collection(vec![
                            Value::object(HashMap::from([
                                ("system".to_string(), Value::String("http://loinc.org".to_string())),
                                ("version".to_string(), Value::String("2.50".to_string())),
                                ("code".to_string(), Value::String("14647-2".to_string())),
                                ("display".to_string(), Value::String("Cholesterol [Moles/volume] in Serum or Plasma".to_string())),
                            ])),
                            Value::object(HashMap::from([
                                ("abstract".to_string(), Value::Boolean(true)),
                                ("display".to_string(), Value::String("Cholesterol codes".to_string())),
                                ("contains".to_string(), Value::collection(vec![
                                        Value::object(HashMap::from([
                                            ("system".to_string(), Value::String("http://loinc.org".to_string())),
                                            ("version".to_string(), Value::String("2.50".to_string())),
                                            ("code".to_string(), Value::String("2093-3".to_string())),
                                            ("display".to_string(), Value::String("Cholesterol [Mass/volume] in Serum or Plasma".to_string())),
                                        ])),
                                        Value::object(HashMap::from([
                                            ("system".to_string(), Value::String("http://loinc.org".to_string())),
                                            ("version".to_string(), Value::String("2.50".to_string())),
                                            ("code".to_string(), Value::String("48620-9".to_string())),
                                            ("display".to_string(), Value::String("Cholesterol [Mass/volume] in Serum or Plasma ultracentrifugate".to_string())),
                                        ])),
                                        Value::object(HashMap::from([
                                            ("system".to_string(), Value::String("http://loinc.org".to_string())),
                                            ("version".to_string(), Value::String("2.50".to_string())),
                                            ("code".to_string(), Value::String("9342-7".to_string())),
                                            ("display".to_string(), Value::String("Cholesterol [Percentile]".to_string())),
                                        ])),
                                    ])),
                            ])),
                            Value::object(HashMap::from([
                                ("abstract".to_string(), Value::Boolean(true)),
                                ("display".to_string(), Value::String("Cholesterol Ratios".to_string())),
                                ("contains".to_string(), Value::collection(vec![
                                        Value::object(HashMap::from([
                                            ("system".to_string(), Value::String("http://loinc.org".to_string())),
                                            ("version".to_string(), Value::String("2.50".to_string())),
                                            ("code".to_string(), Value::String("2096-6".to_string())),
                                            ("display".to_string(), Value::String("Cholesterol/Triglyceride [Mass Ratio] in Serum or Plasma".to_string())),
                                        ])),
                                        Value::object(HashMap::from([
                                            ("system".to_string(), Value::String("http://loinc.org".to_string())),
                                            ("version".to_string(), Value::String("2.50".to_string())),
                                            ("code".to_string(), Value::String("35200-5".to_string())),
                                            ("display".to_string(), Value::String("Cholesterol/Triglyceride [Mass Ratio] in Serum or Plasma".to_string())),
                                        ])),
                                        Value::object(HashMap::from([
                                            ("system".to_string(), Value::String("http://loinc.org".to_string())),
                                            ("version".to_string(), Value::String("2.50".to_string())),
                                            ("code".to_string(), Value::String("48089-7".to_string())),
                                            ("display".to_string(), Value::String("Cholesterol/Apolipoprotein B [Molar ratio] in Serum or Plasma".to_string())),
                                        ])),
                                        Value::object(HashMap::from([
                                            ("system".to_string(), Value::String("http://loinc.org".to_string())),
                                            ("version".to_string(), Value::String("2.50".to_string())),
                                            ("code".to_string(), Value::String("55838-7".to_string())),
                                            ("display".to_string(), Value::String("Cholesterol/Phospholipid [Molar ratio] in Serum or Plasma".to_string())),
                                        ])),
                                    ])),
                            ])),
                        ])),
                ]))),
        ]));

}
