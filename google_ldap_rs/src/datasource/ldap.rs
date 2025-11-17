use serde::Deserialize;
// use futures::TryStreamExt;
//use std::{error::Error, io, process  };
//use rocket::tokio;
//use futures::TryStreamExt;
// use ldap_rs::{LdapClient, SearchEntries, SearchEntry,SearchRequest, SearchRequestDerefAliases, SearchRequestScope};

// use ldap3::result::Result as LdapResult;
use ldap3::{LdapConnAsync,Scope,SearchEntry,ResultEntry};
use config::{Config,File,FileFormat};
// use ldap_rs::error;

#[derive(Deserialize, Debug)]
struct LdapConfig {
    host: String,
    admin_dn: String,
    password: String,
}

pub async fn ldap_test() -> Result<Vec<ResultEntry>, Box<dyn std::error::Error>> {
    let settings = Config::builder().
        add_source(File::new("config.toml",FileFormat::Toml)).
        build()?;
    let ldap_config:LdapConfig = settings.try_deserialize()?;
    let mut ldap_url = String::from("ldap://");
    ldap_url.push_str(&ldap_config.host);
    ldap_url.push_str(":389");
    let (conn,mut ldap) = LdapConnAsync::new(&ldap_url).await?;
    ldap3::drive!(conn);
    let bind_result = ldap.simple_bind(&ldap_config.admin_dn,&ldap_config.password).await;
    match bind_result {
        Ok(_bind_result) => {
            let(rs,_res) = ldap.
                search(
                    "ou=Public,dc=pioneerind,dc=com",
                    Scope::Subtree,
                    "(&(objectClass=mozillaAbPersonAlpha)(|(ou=frank)(ou=family)))",
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
        let test_results = ldap_test().await;
        if let Ok(mut results) = test_results{
            for result in results.iter_mut() {
                let user_entry:&SearchEntry = &SearchEntry::construct(result.clone());
                let dn = &user_entry.dn;
                //println!("Ldap test OK\n {:?} ", user_entry);
                if dn=="cn=Example Person,ou=Public,dc=pioneerind,dc=com" {
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