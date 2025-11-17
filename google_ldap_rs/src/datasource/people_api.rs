use reqwest::{Client};
use crate::datasource::contact;
use crate::datasource::contact::GooglePerson;

pub async fn fetch_contacts(username: &str) ->Result<Vec<GooglePerson>, Box<dyn std::error::Error>> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contact::*;
    

    #[tokio::test]
    async fn extract_google_fullname(){
        if let Ok(mut results) = fetch_contacts("garyc@caseyandgary.com").await{
            for person in results.iter_mut() {
                println!("{:?} {:?}",person.get_field(contact::PersonField::FullName).unwrap(),person.get_modify_timestamp().unwrap());

                let _ = person.set_field(contact::PersonField::FullName,"new person");
                println!("{:?} {:?}",person.get_field(contact::PersonField::FullName).unwrap(),person.get_modify_timestamp().unwrap());

                break;

            }
        }


    }
}

