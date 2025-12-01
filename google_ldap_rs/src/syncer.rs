// use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{Read, Write};
// use std::hash::Hash;
use chrono::{DateTime, TimeZone, Utc};
use config::{Config,  FileFormat};
// use futures::SinkExt;
// use futures::future::err;
use ldap3::{ SearchEntry};
// use reqwest::get;
// use rocket::time::macros::time;
// use rocket::figment::util::bool_from_str_or_int;
//use std::{error::Error, io, process  };
//use ldap_rs::LdapClient;
use crate::datasource::{ldap, people_api,contact};
use crate::datasource::contact::{GooglePerson, LdapPerson, Person, PersonField};
use crate::datasource::ldap::send_ldap_updates;
use crate::datasource::people_api::{send_google_contacts, Operation};
// use crate::datasource::people_api::Operation::Update;
use crate::datasource::sync_config::GoogleConfig;

#[derive(Clone)]
struct UpdatePerson{
    ldap_person:LdapPerson,
    // key is username, value is the Google contact
    google_persons:HashMap<String,GooglePerson>
}
impl UpdatePerson{
    fn new(ldap_person:LdapPerson)->Self{
        UpdatePerson{
            ldap_person:ldap_person,
            google_persons:HashMap::new()
        }
    }
}
struct DataSnapshot{
    // this stores in memory all the contents of LDAP
    update_person_lookup:HashMap<String,UpdatePerson>,
    // this stores Google contacts for each user
    google_contacts_by_user:HashMap<String,Vec<GooglePerson>>,
    // this stores the set of DNs for each user
    ldap_dn_by_user:HashMap<String,HashSet<String>>
}

impl DataSnapshot{
    fn init()->DataSnapshot{
        DataSnapshot{
            update_person_lookup:HashMap::new(),
            google_contacts_by_user:HashMap::new(),
            ldap_dn_by_user:HashMap::new()
        }
    }
}

fn save_timestamp(filename: &str) ->Result<(),Box<dyn std::error::Error>>{
    let now = Utc::now();
    let timestamp = now.to_rfc3339();
    let mut file = File::create(filename)?;
    file.write_all(timestamp.as_bytes())?;
    Ok(())
}

fn load_timestamp(filename: &str) ->Result<DateTime<Utc>,Box<dyn std::error::Error>>{
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let timestamp = DateTime::parse_from_rfc3339(&contents).
    map_err(|_|format!("Invalid timestamp: {}", contents))?.with_timezone(&Utc);
    Ok(timestamp)
}

async fn get_ldap_person_vec(update_person_lookup:&mut HashMap<String,UpdatePerson>)->Result<(),Box<dyn std::error::Error>> {
    let _results = match ldap::fetch_ldap_entries(&"").await {
        Ok(results) => {
            for result in results.iter(){
                let user_entry = SearchEntry::construct(result.clone());
                if let Ok(ldap_person) = LdapPerson::from_ldap(user_entry){
                    if let Ok(res) = ldap_person.get_field(PersonField::FullName) &&
                        let Some(fullname) = res {
                        let update_person = UpdatePerson::new(ldap_person);
                        update_person_lookup.insert(fullname,update_person);
                        // if let Some(subscriptions) = ldap_person.get_subscriptions() {
                            // println!("{} subscriptions: {:?}", fullname, subscriptions);

                        // }
                    }
                }
            }
        },
        Err(e) => return Err(e),
    };
    Ok(())

}

async fn get_google_contact_hashmap(google_config:&GoogleConfig, google_contacts_by_user:&mut HashMap<String,Vec<GooglePerson>>,update_person_lookup:&mut HashMap<String,UpdatePerson>)->Result<(),Box<dyn std::error::Error>> {
    let user_vec = &google_config.users;
    // println!("users: {:?}", user_vec);
    for user in user_vec {
        let username = user.username.as_str();
        // let user_filter = &user.filter;
        // let user_filter = "(|(ou=frank)(ou=family))";
        println!("Reloading Google contacts for user {}",username);
        let _contacts = match people_api::fetch_google_contacts(username).await{
            Ok(contacts) => {
                google_contacts_by_user.insert(username.into(),contacts.clone());
                for contact in contacts.iter(){
                    if let Ok(res) = contact.get_field(PersonField::FullName) &&
                        let Some(fullname) = res  &&
                        let Some(update_person) = update_person_lookup.get(&fullname){
                        // println!("Found update_person for {}",&fullname);
                        // update_person.google_persons.insert(username.to_string(),contact.clone());
                        let mut update_person_copy = update_person.clone();
                        update_person_copy.google_persons.insert(username.to_string(),contact.clone());
                        update_person_lookup.insert(fullname,update_person_copy);

                    }
                }
            },
            Err(e) => return Err(e),
        };
    }
    Ok(())
}

// fn check_overlap(vec1:&Vec<String>,vec2:&Vec<String>)->bool{
//     for i1 in vec1.iter(){
//         for i2 in vec2.iter(){
//             if i1 == i2 {
//                 return true;
//             }
//         }
//     }
//     false
// }
async fn get_ldap_dn_by_user(update_person_lookup:&HashMap<String,UpdatePerson>,
                             google_config:&GoogleConfig,
                             ldap_dn_by_user:&mut HashMap<String,HashSet<String>>)
    ->Result<(),Box<dyn std::error::Error>> {
    let check_overlap = |vec1:&Vec<String>,vec2:&Vec<String>|->bool{
        for i1 in vec1.iter(){
            for i2 in vec2.iter(){
                if i1 == i2 {
                    return true;
                }
            }
        }
        false
    };
    let user_vec = &google_config.users;
    // println!("users: {:?}", user_vec);
    for user in user_vec {
        let username = user.username.as_str();
        // let user_filter = &user.filter;
        ldap_dn_by_user.insert(username.into(), HashSet::new());
    }
    for (fullname,update_person) in update_person_lookup.iter() {
        // if let Ok(res) = ldap_person.get_field(PersonField::FullName) &&
        //     let Some(fullname) = res {
        let ldap_person = &update_person.ldap_person;
        if let Some(ldap_subscriptions) = ldap_person.get_subscriptions() {
            // println!("{} LDAP subscriptions: {:?}", fullname, ldap_subscriptions);
            for user in user_vec {
                let username = user.username.as_str();
                let user_subscriptions = &user.filter;
                // println!("{}'s subscriptions: {:?}", username, user_subscriptions);
                if check_overlap(&ldap_subscriptions,user_subscriptions) {
                    // println!("Overlap found");
                    if ldap_dn_by_user.get(username).is_none(){
                        println!("Creating new ldap fullnames for {}",username);
                        ldap_dn_by_user.insert(username.into(),HashSet::new());
                    }
                    if let Some(fullnames) = ldap_dn_by_user.get_mut(username) {
                        // println!("Adding {} for {}", fullname,username);
                        fullnames.insert(fullname.to_string());
                    };

                }
            }
        }
        // }
    }
    Ok(())
}

async fn purge_duplicates(google_config:&GoogleConfig,data_snapshot: &DataSnapshot
)->Result<(),Box<dyn std::error::Error>> {
    let user_vec = &google_config.users;
    for user in user_vec {
        println!("Purging duplicates for {}",&user.username);
        if let Some(contacts_vec) = data_snapshot.google_contacts_by_user.get(&user.username) {
            let mut resource_name_map:HashMap<String,Vec<(String,DateTime<Utc>)>> = HashMap::new();
            for contact in contacts_vec.iter() {
                if let Some(fullname) = contact.get_field(PersonField::FullName).unwrap() {
                    if let Some(resource_name) = &contact.resource_name &&
                    let Ok(res) = &contact.get_modify_timestamp() &&
                    let Some(update_time) = res{
                        if !resource_name_map.contains_key(&fullname) {
                            resource_name_map.insert(fullname.clone(),Vec::new());
                        }
                        let tuple_vec = resource_name_map.get_mut(&fullname).unwrap();
                        tuple_vec.push((resource_name.clone(),update_time.clone()));
                    }
                }
            }
            let mut resources_for_delete:Vec<String> = Vec::new();
            for (fullname,tuples_vec) in resource_name_map.iter(){
                if tuples_vec.len() > 1 {
                    println!("Found {} copies for {}",tuples_vec.len(),fullname);
                    let earliest:DateTime<Utc> = Utc.timestamp_nanos(0);
                    let mut max_modify_time = earliest;
                    for (_,modify_time) in tuples_vec.iter(){
                        if *modify_time>max_modify_time{
                            max_modify_time = modify_time.clone();
                        }
                    }
                    for (resource_name,modify_time) in tuples_vec.iter(){
                        if *modify_time<max_modify_time{
                            resources_for_delete.push(resource_name.clone());
                        }
                    }
                }
            }
            if resources_for_delete.len() > 0 {
                println!("Resources for delete: {:?}",resources_for_delete);
                send_google_contacts(&user.username,None,Some(&resources_for_delete),Operation::Delete).await?;
            }
        }
    }
    Ok(())
}

async fn do_ldap_additions(google_config:&GoogleConfig,data_snapshot: &DataSnapshot
                            )->Result<(),Box<dyn std::error::Error>> {
    let mut ldap_additions:Vec<LdapPerson> = Vec::new();
    let mut google_updates:HashMap<String,Vec<GooglePerson>> = HashMap::new();
    let user_vec = &google_config.users;


    for user in user_vec {
        println!("Adding to LDAP for user {}",&user.username);
        let username = user.username.as_str();
        let user_subscription = &user.subscription;
        // let user_filter = "(|(ou=frank)(ou=family))";

        if let Some(contacts) = &data_snapshot.google_contacts_by_user.get(&user.username){
            for contact in contacts.iter(){
                let fullname=contact
                    .names.as_ref()
                    .and_then(|namevec:&Vec<contact::Name>| namevec.first())
                    //.display_name.clone())
                    .and_then(|name:&contact::Name| name.display_name.clone())
                    .map(|display_name|display_name.to_string());
                match fullname{
                    Some(name)=>{
                        let sync_status =  contact.is_synced();
                        if !sync_status{
                            println!("contact {} sync is {} so adding to LDAP",name, sync_status);
                            let mut ldap_person = LdapPerson::from_google(contact)?;
                            ldap_person.set_subscriptions(&vec![user_subscription.clone()]);
                            ldap_additions.push(ldap_person);
                            let mut contact_update = contact.clone();
                            contact_update.set_synced();
                            // google_updates.push(contact_update);
                            if google_updates.get(username).is_none() {
                                google_updates.insert(username.to_string(),Vec::new());
                            };
                            if let Some(updates) = google_updates.get_mut(username){
                                updates.push(contact_update);
                            }

                        }
                        // if name=="Example Person"{
                        //     println!("contacts test OK\n {:?} ",contact);
                        //     break;
                        // }
                    },
                    None=>{},
                };
            }
        };
    }
    println!("There are {} google updates and {} ldap additions.",google_updates.len(),ldap_additions.len());
    for (username,google_contacts) in google_updates.iter() {
        if google_contacts.len()>0{
            let _res = send_google_contacts(username.as_str(),Some(google_contacts),None,Operation::Update).await;
        }

    }
    let _res = send_ldap_updates(&ldap_additions,false).await;
    Ok(())
}

async fn purge_orphans(google_config:&GoogleConfig,data_snapshot: &DataSnapshot
)->Result<(),Box<dyn std::error::Error>> {
    let user_vec = &google_config.users;

    #[derive(Debug)]
    enum Existing{
        Ldap,
        Google
    }
    //deletions by username


    // println!("users: {:?}", user_vec);
    // loop through the Google contacts
    for user in user_vec {
        println!("Purging orphans for user {}",&user.username);
        let mut google_deletions:Vec<String> = Vec::new();
        let mut google_additions:Vec<String> = Vec::new();
        let mut resource_name_map:HashMap<String,String> = HashMap::new();
        let mut existing_map:HashMap<String,Vec<Existing>> = HashMap::new();

        if let Some(contacts_vec) = data_snapshot.google_contacts_by_user.get(&user.username) &&
        let Some(dn_vec) = data_snapshot.ldap_dn_by_user.get(&user.username){
            for contact in contacts_vec.iter(){

                if let Some(fullname) = contact.get_field(PersonField::FullName).unwrap(){
                    if let Some(resource_name) = &contact.resource_name{
                        resource_name_map.insert(fullname.clone(),resource_name.clone());
                    }
                    if existing_map.get(&fullname).is_none(){
                        existing_map.insert(fullname.to_string(),Vec::new());
                    }
                    if let Some(existing_vec) = existing_map.get_mut(&fullname.to_string()) {
                        existing_vec.push(Existing::Google);
                    }
                }
            }
            for dn in dn_vec.iter(){
                if existing_map.get(dn).is_none(){
                    existing_map.insert(dn.to_string(),Vec::new());
                }
                if let Some(existing_vec) = existing_map.get_mut(dn) {
                    existing_vec.push(Existing::Ldap);
                }
            }
        }
        for (name,existing_vec) in existing_map.iter(){
            // if let Some(existing_vec) = existing_map.get(name) {
            let exist_count = existing_vec.len();

            if exist_count==1{


                if let Some(exist) = existing_vec.first(){
                    println!("{} exists for category {:?}",name,exist);
                    match exist{
                        Existing::Ldap=>{
                            google_additions.push(name.to_string());
                            // if google_deletions.get(&user.username).is_none(){
                            //     google_deletions.insert(user.username.clone(),Vec::new());
                            // }
                            // if let Some(vec) = google_additions.get_mut(&user.username) {
                            //     vec.push(name.to_string());
                            // }
                        },
                        Existing::Google=>{
                            google_deletions.push(name.to_string());
                            // if google_additions.get(&user.username).is_none(){
                            //     google_additions.insert(user.username.clone(), Vec::new());
                            // }
                            // if let Some(vec) = google_deletions.get_mut(&user.username) {
                            //     vec.push(name.to_string());
                            // }
                        }
                    }
                }
            }
        }
        let mut add_vec:Vec<GooglePerson> = Vec::new();
        for fullname in google_additions.iter(){
            if let Some(update_person) = data_snapshot.update_person_lookup.get(&fullname.to_string()){
                let ldap_person = &update_person.ldap_person;
                println!("Copying LDAP to Google for {}",fullname);
                if let Ok(google_person)  = GooglePerson::new(None,&ldap_person){
                    add_vec.push(google_person);
                }
            }
        }
        if add_vec.len()>0{
            println!("Adding {} Google contacts",add_vec.len());
            let _res = send_google_contacts(&user.username,Some(&add_vec),None,Operation::Add).await;
        }
        let mut del_vec:Vec<String> = Vec::new();
        for fullname in google_deletions.iter() {
            if let Some(resource_name) = resource_name_map.get(&fullname.to_string()) {
                println!("Removing from Google {}",fullname);
                del_vec.push(resource_name.clone());

            }
        }
        if del_vec.len()>0{
            println!("Deleting {} Google contacts",del_vec.len());
            let _res = send_google_contacts(&user.username,None,Some(&del_vec),Operation::Delete).await;
        }
    }
    Ok(())
}

async fn run_updates(google_config:&GoogleConfig,data_snapshot: &DataSnapshot) ->Result<(),Box<dyn std::error::Error>> {
    let user_vec = &google_config.users;
    let filename="timestamp.txt";
    let current_time = load_timestamp(filename).unwrap();

    let get_most_recent = |update_person:&UpdatePerson|->
        Result<(Option<DateTime<Utc>>,Option<LdapPerson>,Option<(String,GooglePerson)>,Option<Vec<String>>),Box<dyn std::error::Error>> {
        let mut max_modify_time = current_time;

        if let Ok(res) = update_person.ldap_person.get_modify_timestamp()
        && let Some(modify_time) = res{
            if modify_time >= max_modify_time{
                println!("LDAP is newest");
                max_modify_time = modify_time;
            }
        }
        for (_username,google_person) in update_person.google_persons.iter(){
            if let Ok(res) = google_person.get_modify_timestamp()
            && let Some(modify_time) = res{
                if modify_time >= max_modify_time{
                    println!("Google contact is newest");
                    max_modify_time = modify_time;
                }
            }
        }
        if let Ok(res) = update_person.ldap_person.get_modify_timestamp()
        && let Some(modify_time) = res
        && modify_time==max_modify_time {
            println!("Declaring LDAP as the newest update");
            return Ok((Some(modify_time),Some(update_person.ldap_person.clone()),None,update_person.ldap_person.get_subscriptions()));
        }else{
            for (username,google_person) in update_person.google_persons.iter(){
                if let Ok(res) = google_person.get_modify_timestamp()
                && let Some(modify_time) = res
                && modify_time==max_modify_time{
                    println!("Declaring Google as the newest update");
                    return Ok((Some(modify_time),None,Some((username.to_string(), google_person.clone()) ),update_person.ldap_person.get_subscriptions()));
                }
            }
        }
        Ok((None, None, None,None))
    };
    let get_subscribed_users = |fullname:&str|->Vec<String>{
        let mut subscribed:Vec<String> = Vec::new();
        for user in user_vec {
            if let Some(dn_set) = data_snapshot.ldap_dn_by_user.get(&user.username){
                if dn_set.contains(&fullname.to_string()){
                    subscribed.push(user.username.to_string());
                }
            }
        }
        subscribed.clone()
    };


    let mut ldap_updates:Vec<LdapPerson> = Vec::new();
    let mut google_updates:HashMap<String,Vec<GooglePerson>> = HashMap::new();
    for (fullname,update_person) in data_snapshot.update_person_lookup.iter(){
        if let Ok(res_tuple) = get_most_recent(update_person){
            let (opt_ts,opt_ldap,opt_google,subscriptions_opt) = res_tuple;
            if let Some(timestamp) = opt_ts{
                println!("Found an edit for {} with time {}",fullname,timestamp);
                if let Some(ldap_person) = opt_ldap{
                    // propagate LDAP to all Google contacts
                    println!("Propagating from LDAP");
                    for username in get_subscribed_users(fullname).iter(){
                        if let Some(orig_google_person) = update_person.google_persons.get(username){
                            println!("Propagating to {}",username);
                            let google_person = GooglePerson::new(Some(&orig_google_person),&ldap_person).unwrap();
                            if google_updates.get(username).is_none(){
                                google_updates.insert(username.to_string(),Vec::new());
                            }
                            if let Some(person_vec) = google_updates.get_mut(username){
                                person_vec.push(google_person);
                            }
                            // google_updates.insert(username.to_string(),google_person);
                        }
                    }
                }else if let Some((username,google_person)) = opt_google{
                    // propagate Google to LDAP and all Google contacts
                    println!("Propagating from Google");
                    println!("Propagating to LDAP");
                    let mut ldap_person = LdapPerson::from_google(&google_person).unwrap();
                    if let Some(subscriptions) = subscriptions_opt{
                        ldap_person.set_subscriptions(&subscriptions);
                    }

                    ldap_updates.push(ldap_person.clone());
                    for other_username in get_subscribed_users(fullname).iter(){
                        if *other_username != username {
                            if let Some(orig_google_person) = update_person.google_persons.get(other_username){
                                println!("Propagating to {}",other_username);
                                let google_person = GooglePerson::new(Some(&orig_google_person),&ldap_person).unwrap();
                                if google_updates.get(&username).is_none() {
                                    google_updates.insert(username.clone(), Vec::new());
                                }
                                if let Some(person_vec) = google_updates.get_mut(other_username){
                                    person_vec.push(google_person);
                                }

                                // google_updates.insert(other_username.to_string(),google_person);
                            }
                        }
                    }

                }
            }
        }
    }
    if ldap_updates.len()>0{
        println!("LDAP updates {:?}",ldap_updates);
        let _res = send_ldap_updates(&ldap_updates,true).await;
    }
    for (username,person_vec) in google_updates.iter(){
        if person_vec.len()>0{
            println!("Google update for user {} with contact: {:?}",username,person_vec);
            let _ = send_google_contacts(username,Some(person_vec),None,Operation::Update).await;
        }
    }
    let _ = save_timestamp(filename);
    Ok(())
}



async fn refresh_data_snapshot(google_config:&GoogleConfig,data_snapshot:&mut DataSnapshot)->Result<(),Box<dyn std::error::Error>> {
    let _results = get_ldap_person_vec(&mut data_snapshot.update_person_lookup).await?;
    // this stores Google contacts for each user
    let _results = get_google_contact_hashmap(&google_config,&mut data_snapshot.google_contacts_by_user,&mut data_snapshot.update_person_lookup).await?;
    // this stores the set of DNs for each user
    let _results = get_ldap_dn_by_user(&mut data_snapshot.update_person_lookup, &google_config, &mut data_snapshot.ldap_dn_by_user).await?;
    Ok(())
}

fn get_config()->Result<GoogleConfig,Box<dyn std::error::Error>> {
    let google_settings = Config::builder().
        add_source(config::File::new("google_config.toml",FileFormat::Toml)).
        build()?;
    println!("Loaded google_config.toml");
    let google_config: GoogleConfig = google_settings.try_deserialize()?;
    println!("Deserialized google_config.toml");
    Ok(google_config)
}

pub async fn sync_contacts()->Result<(),Box<dyn std::error::Error>> {
    let google_config = get_config()?;
    let mut data_snapshot:DataSnapshot = DataSnapshot::init();

    let _ = refresh_data_snapshot(&google_config,&mut data_snapshot).await?;
    let _results = purge_duplicates(&google_config,&data_snapshot).await?;

    // work flow step of checking for sync status and updating sync to true and adding to LDAP
    let _ = refresh_data_snapshot(&google_config,&mut data_snapshot).await?;
    let _results = do_ldap_additions(&google_config,&data_snapshot).await?;

    let _ = refresh_data_snapshot(&google_config,&mut data_snapshot).await?;
    let _results = purge_orphans(&google_config,&data_snapshot).await?;

    let _ = refresh_data_snapshot(&google_config,&mut data_snapshot).await?;
    let _results = run_updates(&google_config,&data_snapshot).await?;

    println!("End sync_contacts");
    Ok(())
}
#[cfg(test)]
mod tests {
    // use chrono::Days;
    use super::*;
    // use crate::contact::*;
    // use ldap_rs as ldap;
    // use ldap_rs::bytes::Bytes;
    // use rocket::time::macros::time;
    // use rocket::yansi::Paint;
    // use crate::datasource::contact;

    #[tokio::test]
    async fn run_sync_contacts() {
        println!("Running sync contacts");
        let _res = sync_contacts().await;
    }

    // #[test]
    // fn save_timestamp_test(){
    //     println!("Saving timestamp");
    //     let filename="timestamp.txt";
    //     let _ = save_timestamp(filename);
    //
    // }
    // #[test]
    // fn load_timestamp_test(){
    //     let filename = "timestamp.txt";
    //     let timestamp = load_timestamp(filename).unwrap();
    //     println!("The time stamp is {}",timestamp);
    // }


}
