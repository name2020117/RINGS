use bson::{doc, document::Document};
use regex::Regex;
use serde::{Deserialize, Serialize};

fn validate_imsi(imsi: &str) -> bool {
    let re = Regex::new("^20893[0-9]{10}$").unwrap();
    re.is_match(imsi)
}

#[cfg(test)]
mod tests {
    use super::validate_imsi;
    #[test]
    fn test_imsi_default() {
        assert_eq!(validate_imsi("208930000000003"), true, "default imsi");
        assert_eq!(validate_imsi("208930000000003"), true, "default imsi");
    }

    #[test]
    fn test_imsi_wrong_mcc() {
        assert_eq!(
            validate_imsi("000000000000003"),
            false,
            "not 20893 mobile country code"
        );
    }

    #[test]
    fn test_imsi_too_long() {
        assert_eq!(
            validate_imsi("0000000000000030"),
            false,
            "not 20893 mobile country code"
        );
    }

    #[test]
    fn test_imsi_too_short() {
        assert_eq!(
            validate_imsi("00000000000003"),
            false,
            "not 20893 mobile country code"
        );
    }

    #[test]
    fn test_imsi_from_u64() {
        assert_eq!(
            validate_imsi(&208930000000005u64.to_string()),
            true,
            "not 20893 mobile country code"
        );
    }
}

/// am data
/// Default entry:
/// { "_id" : ObjectId("62c42f172159f019f54ee5f9"), "subscCats" : [ "free5gc" ], "ueId" : "imsi-208930000000003" }
#[allow(non_snake_case, dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct PolicyUeSm {
    subscCats: Vec<String>,
    #[allow(non_snake_case)]
    ueId: String,
}

#[allow(dead_code)]
impl PolicyUeSm {
    pub fn new(imsi: String) -> Self {
        if validate_imsi(&imsi) {
            return Self {
                subscCats: vec![String::from("free5gc")],
                ueId: format!("imsi-{}", imsi),
            };
        } else {
            panic!("imsi number is not valid")
        }
    }
}

pub fn policy_am_doc(imsi: &str) -> Document {
    assert_eq!(true, validate_imsi(imsi));
    doc! { "subscCats" : [ "free5gc" ], "ueId" : format!("imsi-{}", imsi) }
}

pub fn policy_sm_doc(imsi: &str) -> Document {
    assert_eq!(true, validate_imsi(imsi));
    doc! { "smPolicySnssaiData" : { "01010203" : { "snssai" : { "sst" : 1, "sd" : "010203" }, "smPolicyDnnData" : { "internet" : { "dnn" : "internet" }, "internet2" : { "dnn" : "internet2" } } }, "01112233" : { "snssai" : { "sst" : 1, "sd" : "112233" }, "smPolicyDnnData" : { "internet" : { "dnn" : "internet" }, "internet2" : { "dnn" : "internet2" } } } }, "ueId" : format!("imsi-{}", imsi) }
}

pub fn subscription_auth_doc(imsi: &str) -> Document {
    assert_eq!(true, validate_imsi(imsi));
    doc! { "authenticationMethod" : "5G_AKA", "permanentKey" : { "permanentKeyValue" : "8baf473f2f8fd09487cccbd7097c6862", "encryptionKey" : 0, "encryptionAlgorithm" : 0 }, "sequenceNumber" : "16f3b3f70fc2", "authenticationManagementField" : "8000", "milenage" : { "op" : { "opValue" : "", "encryptionKey" : 0, "encryptionAlgorithm" : 0 } }, "opc" : { "opcValue" : "8e27b6af0e692e750f32667a3b14605d", "encryptionKey" : 0, "encryptionAlgorithm" : 0 }, "ueId" : format!("imsi-{}", imsi)
    }
}

pub fn subscription_provision_am_doc(imsi: &str) -> Document {
    assert_eq!(true, validate_imsi(imsi));
    doc! { "gpsis" : [ "msisdn-0900000000" ], "subscribedUeAmbr" : { "uplink" : "1 Gbps", "downlink" : "2 Gbps" }, "nssai" : { "defaultSingleNssais" : [ { "sst" : 1, "sd" : "010203" }, { "sst" : 1, "sd" : "112233" } ] }, "ueId" : format!("imsi-{}", imsi), "servingPlmnId" : "20893"
    }
}

pub fn subscription_provision_sm_doc_1(imsi: &str) -> Document {
    assert_eq!(true, validate_imsi(imsi));
    doc! {
    "dnnConfigurations" : { "internet" : { "pduSessionTypes" : { "defaultSessionType" : "IPV4", "allowedSessionTypes" : [ "IPV4" ] }, "sscModes" : { "defaultSscMode" : "SSC_MODE_1", "allowedSscModes" : [ "SSC_MODE_2", "SSC_MODE_3" ] }, "5gQosProfile" : { "priorityLevel" : 8, "5qi" : 9, "arp" : { "priorityLevel" : 8, "preemptCap" : "", "preemptVuln" : "" } }, "sessionAmbr" : { "downlink" : "100 Mbps", "uplink" : "200 Mbps" } }, "internet2" : { "pduSessionTypes" : { "defaultSessionType" : "IPV4", "allowedSessionTypes" : [ "IPV4" ] }, "sscModes" : { "defaultSscMode" : "SSC_MODE_1", "allowedSscModes" : [ "SSC_MODE_2", "SSC_MODE_3" ] }, "5gQosProfile" : { "5qi" : 9, "arp" : { "priorityLevel" : 8, "preemptCap" : "", "preemptVuln" : "" }, "priorityLevel" : 8 }, "sessionAmbr" : { "uplink" : "200 Mbps", "downlink" : "100 Mbps" } } }, "ueId" : format!("imsi-{}",imsi), "servingPlmnId" : "20893", "singleNssai" : { "sst" : 1, "sd" : "010203" }
       }
}

pub fn subscription_provision_sm_doc_2(imsi: &str) -> Document {
    assert_eq!(true, validate_imsi(imsi));
    doc! { "singleNssai" : { "sst" : 1, "sd" : "112233" }, "dnnConfigurations" : { "internet" : { "pduSessionTypes" : { "defaultSessionType" : "IPV4", "allowedSessionTypes" : [ "IPV4" ] }, "sscModes" : { "defaultSscMode" : "SSC_MODE_1", "allowedSscModes" : [ "SSC_MODE_2", "SSC_MODE_3" ] }, "5gQosProfile" : { "5qi" : 9, "arp" : { "priorityLevel" : 8, "preemptCap" : "", "preemptVuln" : "" }, "priorityLevel" : 8 }, "sessionAmbr" : { "uplink" : "200 Mbps", "downlink" : "100 Mbps" } }, "internet2" : { "sessionAmbr" : { "uplink" : "200 Mbps", "downlink" : "100 Mbps" }, "pduSessionTypes" : { "defaultSessionType" : "IPV4", "allowedSessionTypes" : [ "IPV4" ] }, "sscModes" : { "defaultSscMode" : "SSC_MODE_1", "allowedSscModes" : [ "SSC_MODE_2", "SSC_MODE_3" ] }, "5gQosProfile" : { "priorityLevel" : 8, "5qi" : 9, "arp" : { "preemptCap" : "", "preemptVuln" : "", "priorityLevel" : 8 } } } }, "ueId" : format!("imsi-{}", imsi), "servingPlmnId" : "20893"
    }
}

pub fn subscription_provision_smf_doc(imsi: &str) -> Document {
    assert_eq!(true, validate_imsi(imsi));
    doc! {
         "subscribedSnssaiInfos" : { "01010203" : { "dnnInfos" : [ { "dnn" : "internet" }, { "dnn" : "internet2" } ] }, "01112233" : { "dnnInfos" : [ { "dnn" : "internet" }, { "dnn" : "internet2" } ] } }, "ueId" : format!("imsi-{}", imsi), "servingPlmnId" : "20893"

    }
}

// pub fn policy_data_sm_doc(imsi: &str) -> Document {
//    doc! {
//        "smPolicySnssaiData" : { \"01010203\" : {{ \"snssai\" : {{ \"sst\" : 1, \"sd\" : \"010203\" }}, \"smPolicyDnnData\" : {{ \"internet\" : {{ \"dnn\" : \"internet\" }}, \"internet\" : {{ \"dnn\" : \"internet2\" }} }} }}, \"01112233\" : {{ \"snssai\" : {{ \"sst\" : 1, \"sd\" : \"112233\" }}, \"smPolicyDnnData\" : {{ \"internet\" : {{ \"dnn\" : \"internet\" }}, \"internet2\" : {{ \"dnn\" : \"internet2\" }} }} }} }}, \"ueId\" : \"imsi-{}\" ", imsi)
//    }
// }

// pub fn policy_data_sm_string(imsi: &str) -> String {
//     if validate_imsi(&imsi) {
//         format!("\"smPolicySnssaiData\" : {{ \"01010203\" : {{ \"snssai\" : {{ \"sst\" : 1, \"sd\" : \"010203\" }}, \"smPolicyDnnData\" : {{ \"internet\" : {{ \"dnn\" : \"internet\" }}, \"internet\" : {{ \"dnn\" : \"internet2\" }} }} }}, \"01112233\" : {{ \"snssai\" : {{ \"sst\" : 1, \"sd\" : \"112233\" }}, \"smPolicyDnnData\" : {{ \"internet\" : {{ \"dnn\" : \"internet\" }}, \"internet2\" : {{ \"dnn\" : \"internet2\" }} }} }} }}, \"ueId\" : \"imsi-{}\" ", imsi)
//     } else {
//         panic!("imsi number is not valid")
//     }
// }

// sm data
// Default entry:
// { "_id" : ObjectId("62c42f182159f019f54ee5fa"), "smPolicySnssaiData" : { "01010203" : { "snssai" : { "sst" : 1, "sd" : "010203" }, "smPolicyDnnData" : { "internet" : { "dnn" : "internet" }, "internet2" : { "dnn" : "internet2" } } }, "01112233" : { "snssai" : { "sst" : 1, "sd" : "112233" }, "smPolicyDnnData" : { "internet" : { "dnn" : "internet" }, "internet2" : { "dnn" : "internet2" } } } }, "ueId" : "imsi-208930000000003" }
// #[allow(non_snake_case, dead_code)]
// #[derive(Debug, Serialize, Deserialize)]
// #[allow(dead_code)]
// struct UesSmData {}
