use std::cmp::min;
// use futures::SinkExt;
use reqwest::{Client};
use serde::Serialize;
use crate::datasource::contact;
use crate::datasource::contact::*;

pub async fn fetch_google_contacts(username: &str) ->Result<Vec<GooglePerson>, Box<dyn std::error::Error>> {
    let client  = Client::new();
    let url = String::from("http://127.0.0.1:8340/ListContacts/")+username;
    let request_builder = client.get(url);
    // println!("Built request builder");
    let response = request_builder.send().await?;
    // println!("Built response");
    // let body = response.text().await?;
    // println!("body = {:?}",body);
    let person_list =  response.json::<Vec<contact::GooglePerson>>().await?;
    // println!("person_list: \n\n {:?}",person_list);
    Ok(person_list)
}
pub enum Operation{
    Add,
    Update,
    Delete
}

pub async fn send_google_contacts<T: Serialize>(operation:Operation,username: &str,full_list:&Vec<T>,list_limit:usize)
    ->Result<(),Box<dyn std::error::Error>>{
    let client  = Client::new();
    let vec_size = full_list.len();
    let mut start = 0;
    let mut end = min(start + list_limit,vec_size);
    let uri = match operation {
        Operation::Add => "AddContacts",
        Operation::Update => "UpdateContacts",
        Operation::Delete => "DeleteContacts"
    };
    while start<end {
        println!("Submitting to Google sublist from index {} to {}",start,end);
        let sub_list = &full_list[start..end];
        start = min(end, full_list.len());
        end = min(start + list_limit, full_list.len());
        let request_builder = client.post(String::from("http://127.0.0.1:8340/".to_owned() +uri+"/"+username));
        let response = request_builder.body(serde_json::to_string(&sub_list).unwrap()).send().await;
        match response{
            Ok(response) => {
                let body = response.text().await;
                match body{
                    Ok(body) => {
                        println!("Server response: {}",body);
                    },
                    Err(e) => {
                        println!("Body error {}",e);
                    }
                }

            },
            Err(error) => {
                println!("Response error {}",error);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    //use ldap3::Ldap;
    use super::*;
    // use crate::contact::GooglePerson;

    

    #[tokio::test]
    async fn extract_google_fullname(){
        if let Ok(mut results) = fetch_google_contacts("garyc@caseyandgary.com").await{
            for person in results.iter_mut() {
                if let Ok(res) = person.get_field(contact::PersonField::FullName){
                    if let Some(name) = res{
                        if name == "Gary Chen"{
                        // if name == "Example Person"{
                            println!("Found: {:?}",name);
                            let ldap_person = LdapPerson::from_google(person);
                            println!("lDAP entry: {:?}",ldap_person);
                            break;
                        }
                    }
                }
                // Some(fullname) =
                // println!("{:?} {:?}",person.get_field(contact::PersonField::FullName).unwrap(),person.get_modify_timestamp().unwrap());
                //
                // let _ = person.set_field(contact::PersonField::FullName,"new person");
                // println!("{:?} {:?}",person.get_field(contact::PersonField::FullName).unwrap(),person.get_modify_timestamp().unwrap());
                //
                // break;

            }
        }


    }
}

