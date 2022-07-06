use anyhow::Result;
use bson::Document;
use clap::Parser;
use fiveg_cli::cli::Args;
use fiveg_cli::schema::{policy_am_doc, policy_sm_doc, subscription_auth_status_doc, subscription_auth_subscription_doc, subscription_provision_am_doc, subscription_provision_sm_doc_1, subscription_provision_sm_doc_2, subscription_provision_smf_doc};
use mongodb::sync::{Client, Collection};

const POLICY_UE_AM: &str = "policyData.ues.amData";
const POLICY_UE_SM: &str = "policyData.ues.smData";
const SUB_AUTH_SUBSCRIPTION: &str = "subscriptionData.authenticationData.authenticationSubscription";
const SUB_AUTH_STATUS: &str = "subscriptionData.authenticationData.authenticationStatus";
const SUB_AM: &str = "subscriptionData.provisionedData.amData";
const SUB_SM: &str = "subscriptionData.provisionedData.smData";
const SUB_SMF: &str = "subscriptionData.provisionedData.smfSelectionSubscriptionData";

fn main() -> Result<()> {
    let args = Args::parse();
    let client = Client::with_uri_str(format!("mongodb://{}:{}", args.db_ip, args.port))?;
    println!("Printing databases on {}:{}", args.db_ip, args.port);
    for db_name in client.list_database_names(None, None)? {
        println!("db found = {}", db_name);
    }
    let db = client.database("free5gc");
    for i in db.list_collection_names(None).unwrap().iter() {
        println!("Collection = {:?}", i,);
    }
    // HACK: This is code is incredibly ugly and not clean, but it was faster to c&p then make it
    // clean at the time of writing
    let policy_ue_am_collection: Collection<Document> = db.collection(POLICY_UE_AM);
    let policy_ue_sm_collection: Collection<Document> = db.collection(POLICY_UE_SM);
    let subscription_auth_subscription_collection: Collection<Document> = db.collection(SUB_AUTH_SUBSCRIPTION);
    let subscription_auth_status_collection: Collection<Document> = db.collection(SUB_AUTH_STATUS);
    let subscription_provision_am_collection: Collection<Document> = db.collection(SUB_AM);
    let subscription_provision_sm_collection: Collection<Document> = db.collection(SUB_SM);
    let subscription_provision_smf_collection: Collection<Document> = db.collection(SUB_SMF);
    let mut policy_ue_am_vec: Vec<Document> = Vec::new();
    let mut policy_ue_sm_vec: Vec<Document> = Vec::new();
    let mut subscription_auth_subscription_vec: Vec<Document> = Vec::new();
    let mut subscription_auth_status_vec: Vec<Document> = Vec::new();
    let mut subscription_provision_am_vec: Vec<Document> = Vec::new();
    let mut subscription_provision_sm_vec_1: Vec<Document> = Vec::new();
    let mut subscription_provision_sm_vec_2: Vec<Document> = Vec::new();
    let mut subscription_provision_smf_vec: Vec<Document> = Vec::new();
    for i in args.imsi..(args.imsi + u64::from(args.n_subscribers) + 1) {
        policy_ue_am_vec.push(policy_am_doc(&i.to_string()));
        policy_ue_sm_vec.push(policy_sm_doc(&i.to_string()));
        subscription_auth_subscription_vec.push(subscription_auth_subscription_doc(&i.to_string()));
        subscription_auth_status_vec.push(subscription_auth_status_doc(&i.to_string()));
        subscription_provision_am_vec.push(subscription_provision_am_doc(&i.to_string()));
        subscription_provision_sm_vec_1.push(subscription_provision_sm_doc_1(&i.to_string()));
        subscription_provision_sm_vec_2.push(subscription_provision_sm_doc_2(&i.to_string()));
        subscription_provision_smf_vec.push(subscription_provision_smf_doc(&i.to_string()));
    }
    policy_ue_am_collection.insert_many(policy_ue_am_vec, None)?;
    policy_ue_sm_collection.insert_many(policy_ue_sm_vec, None)?;
    subscription_auth_subscription_collection.insert_many(subscription_auth_subscription_vec, None)?;
    subscription_auth_status_collection.insert_many(subscription_auth_status_vec, None)?;
    subscription_provision_am_collection.insert_many(subscription_provision_am_vec, None)?;
    subscription_provision_sm_collection.insert_many(subscription_provision_sm_vec_1, None)?;
    subscription_provision_sm_collection.insert_many(subscription_provision_sm_vec_2, None)?;
    subscription_provision_smf_collection.insert_many(subscription_provision_smf_vec, None)?;
    Ok(())
}
