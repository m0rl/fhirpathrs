//! Auto-generated from FHIRPath spec tests-fhir-r5.xml
//! Source: https://github.com/FHIR/fhir-test-cases/blob/master/r5/fhirpath/tests-fhir-r5.xml
//! Regenerate: python scripts/generate_spec_tests.py

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::approx_constant,
    clippy::needless_raw_string_hashes
)]

use interpreter::{InterpreterContext, Value, interpret};
use parser::parse;
use std::collections::HashMap;

mod cdatests;
mod comments;
mod comparable;
mod definevariable;
mod fixtures;
mod from_zulip;
mod highboundary;
mod index_part;
mod lowboundary;
mod miscenginetests;
mod period;
mod polymorphics;
mod precision;
mod terminologytests;
mod testabs;
mod testaggregate;
mod testall;
mod testbasics;
mod testbooleanimplies;
mod testbooleanlogicand;
mod testbooleanlogicor;
mod testbooleanlogicxor;
mod testcase;
mod testceiling;
mod testcollectionboolean;
mod testcombine__;
mod testconcatenate;
mod testconformsto;
mod testcontainscollection;
mod testcontainsstring;
mod testcount;
mod testdistinct;
mod testdiv;
mod testdivide;
mod testdollar;
mod testencodedecode;
mod testendswith;
mod testequality;
mod testequivalent;
mod testescapeunescape;
mod testexclude;
mod testexists;
mod testexp;
mod testextension;
mod testfirstlast;
mod testfloor;
mod testgreaterthan;
mod testgreatororequal;
mod testiif;
mod testin;
mod testindexer;
mod testindexof;
mod testinheritance;
mod testintersect;
mod testjoin;
mod testlength;
mod testlessorequal;
mod testlessthan;
mod testliterals;
mod testln;
mod testlog;
mod testmatches;
mod testminus;
mod testmiscellaneousaccessortests;
mod testmod;
mod testmultiply;
mod testnequality;
mod testnotequivalent;
mod testnow;
mod testobservations;
mod testplus;
mod testpower;
mod testprecedence;
mod testquantity;
mod testrepeat;
mod testreplace;
mod testreplacematches;
mod testround;
mod testselect;
mod testsingle;
mod testskip;
mod testsort;
mod testsplit;
mod testsqrt;
mod teststartswith;
mod testsubsetof;
mod testsubstring;
mod testsupersetof;
mod testtail;
mod testtake;
mod testtochars;
mod testtoday;
mod testtodecimal;
mod testtointeger;
mod testtostring;
mod testtrace;
mod testtrim;
mod testtruncate;
mod testtype;
mod testtypes;
mod testunion;
mod testvariables;
mod testwhere;
