use std::collections::HashSet;
// use serde::Deserialize;
// use futures::TryStreamExt;
//use std::{error::Error, io, process  };
//use rocket::tokio;
//use futures::TryStreamExt;
// use ldap_rs::{LdapClient, SearchEntries, SearchEntry,SearchRequest, SearchRequestDerefAliases, SearchRequestScope};

// use ldap3::result::Result as LdapResult;
use ldap3::{LdapConnAsync,Scope,SearchEntry,ResultEntry};
use config::{Config,File,FileFormat};
use crate::datasource::contact::{LdapPerson, Person, PersonField};
use super::sync_config::{LdapConfig};

// use ldap_rs::error;


pub async fn fetch_ldap_entries(user_filter: &str) -> Result<Vec<ResultEntry>, Box<dyn std::error::Error>> {
    let ldap_settings = Config::builder().
        add_source(File::new("ldap_config.toml",FileFormat::Toml)).
        build()?;
    let ldap_config: LdapConfig = ldap_settings.try_deserialize()?;
    let mut ldap_url = String::from("ldap://");
    ldap_url.push_str(&ldap_config.host);
    ldap_url.push_str(":389");
    let filter = String::from("(&(objectClass=mozillaAbPersonAlpha)")+user_filter+")";
    let (conn,mut ldap) = LdapConnAsync::new(&ldap_url).await?;
    ldap3::drive!(conn);
    let bind_result = ldap.simple_bind(&ldap_config.admin_dn,&ldap_config.password).await;
    match bind_result {
        Ok(_bind_result) => {
            let(rs,_res) = ldap.
                search(
                    "ou=Public,dc=pioneerind,dc=com",
                    Scope::Subtree,
                    filter.as_str(),
                    vec!["cn","givenName","sn","mail",
                         "mozillaSecondEmail","homePhone", "facsimileTelephoneNumber",
                         "mobile", "pager","street", "l","st", "postalCode","mozillaHomeStreet",
                         "mozillaHomeLocalityName","mozillaHomeState",
                         "mozillaHomePostalCode","telephoneNumber",
                         "telephoneNumber","o", "title","description", "ou",
                         "modifyTimestamp"]).await?.success()?;
            ldap.unbind().await?;
            Ok(rs)
        },
        Err(e) => {
            println!("Bind error: {:?}", e);
            return Err(Box::new(e));

        }
    }
}
pub async fn send_ldap_updates(ldap_person_vec: &Vec<LdapPerson>,delete:bool) -> Result<(), Box<dyn std::error::Error>> {
    let ldap_settings = Config::builder().
        add_source(File::new("ldap_config.toml",FileFormat::Toml)).
        build()?;
    let ldap_config: LdapConfig = ldap_settings.try_deserialize()?;
    let mut ldap_url = String::from("ldap://");
    ldap_url.push_str(&ldap_config.host);
    ldap_url.push_str(":389");

    let (conn,mut ldap) = LdapConnAsync::new(&ldap_url).await?;
    ldap3::drive!(conn);
    let bind_result = ldap.simple_bind(&ldap_config.admin_dn,&ldap_config.password).await;
    match bind_result {
        Ok(_bind_result) => {
            for ldap_person in ldap_person_vec.iter(){
                let res = ldap_person.get_field(PersonField::FullName).unwrap();
                let fullname = res.unwrap();
                let dn = String::from("cn=")+fullname.as_str()+",ou=Public,dc=pioneerind,dc=com";
                if delete{
                    println!("Deleting dn {}",dn);
                    let res = ldap.delete(&dn.clone() ).await;
                    match res {
                        Ok(_result) => {
                            let res2 = _result.success();
                            match res2 {
                                Ok(_result) => {
                                    println!("Successfully deleted dn {}", dn);
                                },
                                Err(e) => {
                                    println!("Failed to delete {} because: {:?}",dn, e);
                                    // return Err(Box::new(e));
                                }
                            }
                        },
                        Err(e) => {
                            println!("LDAP delete error 1: {:?}", e);
                            // return Err(Box::new(e));
                        }
                    }
                }                
                println!("Adding dn {}",dn);
                let attrs_map = &ldap_person.search_entry.as_ref().unwrap().attrs;
                let mut attrs_vec = Vec::new();
                for (key, vals) in attrs_map.iter() {
                    // println!("Adding key {}",key);
                    let mut val_set = HashSet::new();
                    for val in vals.iter() {
                        val_set.insert(val.clone());
                    }
                    attrs_vec.push((key.clone(), val_set));
                }
                let vals = vec!["top","person","organizationalPerson","inetOrgPerson","mozillaAbPersonAlpha"];
                let mut val_set = HashSet::new();
                for val in vals.iter() {
                    val_set.insert(val.to_string());
                }
                attrs_vec.push(("objectClass".into(),val_set));
                let res = ldap.add(&dn.clone(), attrs_vec).await;
                match res {
                    Ok(_result) => {
                        let res2 = _result.success();
                        match res2 {
                            Ok(_result) => {
                                println!("Successfully added dn {}", dn);
                            },
                            Err(e) => {
                                println!("Failed to add {} because: {:?}",dn, e);
                                // return Err(Box::new(e));
                            }
                        }
                    },
                    Err(e) => {
                        println!("LDAP add error 1: {:?}", e);
                        // return Err(Box::new(e));
                    }
                }

               // }
            }
            ldap.unbind().await?;

        },
        Err(e) => {
            println!("Bind error: {:?}", e);
            return Err(Box::new(e));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use chrono::Days;
    use super::*;
    use crate::contact::*;
    // use ldap_rs as ldap;
    // use ldap_rs::bytes::Bytes;
    // use rocket::time::macros::time;
    // use rocket::yansi::Paint;
    use crate::datasource::contact;

    #[tokio::test]
    async fn extract_ldap_cn(){
        let user_filter = "(|(ou=frank)(ou=family))";
        let test_results = fetch_ldap_entries(user_filter).await;
        if let Ok(mut results) = test_results{
            for result in results.iter_mut() {
                let user_entry:&SearchEntry = &SearchEntry::construct(result.clone());
                let dn = &user_entry.dn;
                //println!("Ldap test OK\n {:?} ", user_entry);
                //if dn=="cn=Example Person,ou=Public,dc=pioneerind,dc=com" {
                if dn=="cn=Gary Chen,ou=Public,dc=pioneerind,dc=com" {
                    let mut ldap_person = LdapPerson{
                        search_entry: Some(user_entry.clone())
                    };
                    let mut last_name = ldap_person.get_field( contact::PersonField::LastName).unwrap().unwrap();

                    println!("Last name {:?}",last_name);
                    let _res = ldap_person.set_field(PersonField::LastName,"newlastname");
                    last_name = ldap_person.get_field( contact::PersonField::LastName).unwrap().unwrap();
                    println!("Last name {:?}",last_name);

                    let mut timestamp = ldap_person.get_modify_timestamp().unwrap().unwrap();
                    println!("Timestamp {:?} ", timestamp);
                    if let Some(futuretime) = timestamp.checked_add_days(Days::new(1)) {
                        println!("Timestamp {:?} ", futuretime);
                        let _res = ldap_person.set_modify_timestamp(futuretime);
                        timestamp = ldap_person.get_modify_timestamp().unwrap().unwrap();
                        println!("Timestamp {:?} ", timestamp);
                    }
                    let _res = match GooglePerson::new(None,&ldap_person){
                        Ok(google_person) => {
                            let google_str = serde_json::to_string(&google_person).unwrap();
                            println!("{:}", google_str);
                        },
                        Err(e) => {
                            println!("Error is {:?}",e);
                        }
                    };






                    break;
                }
                //println!("{:?}", result);
            }
        }else{

            println!("{:?}", test_results.err().unwrap());
        }

    }
}

//#[tokio::main]
// pub async fn ldap_test2() -> Result<(Vec<SearchEntry>), Box<dyn std::error::Error>> {
//     //pretty_env_logger::init_timed();
//
//     // // let options = TlsOptions::tls();
//     //
//     let attributes:Vec<&str> = ["cn","givenName","sn","mail",
//         "mozillaSecondEmail","homePhone", "facsimileTelephoneNumber",
//         "mobile", "pager","street", "l","st", "postalCode","mozillaHomeStreet",
//         "mozillaHomeLocalityName","mozillaHomeState",
//         "mozillaHomePostalCode","telephoneNumber",
//         "telephoneNumber","o", "title","description", "ou",
//         "modifyTimestamp"].to_vec();
//     let mut client = LdapClient::builder("lupine")
//         //     // .tls_options(options)
//         .connect()
//         .await?;
//     client
//         .simple_bind("cn=admin,dc=pioneerind,dc=com", "cas98ey")
//         .await?;
//     println!("client created");
//
//     let req = SearchRequest::builder()
//         .base_dn("dc=pioneerind,dc=com")
//         .scope(SearchRequestScope::WholeSubtree)
//         .deref_aliases(SearchRequestDerefAliases::NeverDerefAliases)
//         .filter("(&(objectClass=mozillaAbPersonAlpha)(|(ou=garyc)(ou=family)))")
//         .attributes(attributes)
//         .build()?;
//     println!("search request created");
//
//     let result:SearchEntries= client.search(req).await?;
//     let items:Vec<SearchEntry> = result.try_collect().await?;
//
//     // println!("Items: {items:#?}");
//     Ok(items)
// }
