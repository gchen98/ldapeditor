// use futures::future::err;
use ldap3::SearchEntry;
//use std::{error::Error, io, process  };
//use ldap_rs::LdapClient;
use crate::datasource::{ldap, people_api,contact};
pub async fn sync_contacts()->Result<(),Box<dyn std::error::Error>> {
    let _results = match ldap::ldap_test().await {
        Ok(results) => {
            for result in results.iter(){
                let user_entry = SearchEntry::construct(result.clone());
                let dn = user_entry.dn;
                if dn=="cn=Example Person,ou=Public,dc=pioneerind,dc=com"{
                    println!("Ldap test OK\n {:?} ",result);
                    break;
                }else{
                    //println!("Ldap test searching\n {:?} ",dn);
                }

            }

        },
        Err(e) => return Err(e),
    };
    let _contacts = match people_api::fetch_contacts("garyc%40caseyandgary.com").await{
        Ok(contacts) => {
            for contact in contacts.iter(){
                let fullname=contact
                    .names.as_ref()
                    .and_then(|namevec:&Vec<contact::Name>| namevec.first())
                        //.display_name.clone())
                    .and_then(|name:&contact::Name| name.display_name.clone())
                    .map(|display_name|display_name.to_string());
                match fullname{
                    Some(name)=>{
                        if name=="Example Person"{
                            println!("contacts test OK\n {:?} ",contact);
                            break;
                        }
                    },
                    None=>{},
                };
            }
        },
        Err(e) => return Err(e),
    };
    println!("begin sync_contacts");
    Ok(())
}