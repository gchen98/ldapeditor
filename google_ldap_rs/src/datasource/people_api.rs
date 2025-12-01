use reqwest::{Client};
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
pub async fn send_google_contacts(username: &str,contact_list:Option<&Vec<GooglePerson>>,
                                  resource_name_list:Option<&Vec<String>>,operation:Operation)
    ->Result<(),Box<dyn std::error::Error>>{
    let client  = Client::new();
    // let mut json_str =String::new();
    let request_builder = match operation{
        Operation::Add => {
            let url = String::from("http://127.0.0.1:8340/AddContacts/")+username;
            let request_builder = client.post(url);
            let json_list = contact_list.unwrap();
            // let json_str = serde_json::to_string(&json_list).unwrap().clone();
            // println!("Sending JSON {:?}",&json_str);
            // println!("Sending JSON2 {:?}",json_list);
            request_builder.body(serde_json::to_string(&json_list).unwrap())
            // request_builder.json(&json_list)
        },
        Operation::Update =>{
            let url = String::from("http://127.0.0.1:8340/UpdateContacts/")+username;
            let request_builder = client.post(url);
            let json_list = contact_list.unwrap();
            // let  json_str = serde_json::to_string(&json_list).unwrap().clone();
            // println!("Sending JSON {:?}",&json_str);
            // println!("Sending JSON2 {:?}",serde_json::to_string(&json_list).unwrap());
            request_builder.body(serde_json::to_string(&json_list).unwrap())
            // request_builder.json(&json_list)
        },
        Operation::Delete => {
            let url = String::from("http://127.0.0.1:8340/DeleteContacts/")+username;
            let request_builder = client.post(url);
            let json_list = resource_name_list.unwrap();
            // println!("Sending JSON {:?}",json_list);
            request_builder.json(&json_list)
        }
    };
    // let request_builder = client.post(url);
    // println!("Sending JSON {:?}",contact_list);
    let response = request_builder.send().await;
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

