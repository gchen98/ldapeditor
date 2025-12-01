use std::string::String;
// use std::arch::global_asm;
// use std::arch::x86_64::_mm256_mask_unpacklo_pd;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
// use futures::future::ok;
use ldap3::SearchEntry;
// use rocket::time::Date;
// use rocket::yansi::Paint;
use serde_with::skip_serializing_none;
// use serde_json::Value::String;
// use crate::datasource::contact;

//#[allow(dead_code)]

#[allow(dead_code)]
pub enum ContactError{
    Message(String)
}
pub enum PersonField{
    FullName,
    FirstName,
    LastName,
    Company,
    Title,
    EmailWork,
    EmailHome,
    PhoneMobile1,
    PhoneMobile2,
    PhoneWork1,
    PhoneWork2,
    PhoneHome1,
    PhoneHome2,
    AddressStreetWork,
    AddressCityWork,
    AddressRegionWork,
    AddressPostalCodeWork,
    AddressStreetHome,
    AddressCityHome,
    AddressRegionHome,
    AddressPostalCodeHome,
    Notes,
    UserDefined,

}

#[derive(Debug)]
#[derive(Clone)]
#[skip_serializing_none]
#[derive(Serialize,Deserialize)]
#[serde(rename = "profileMetadata")]
pub struct ProfileMetadata {
    #[serde(rename = "objectType")]
    pub object_type: Option<String>,
    #[serde(rename = "userTypes")]
    pub user_types:Option<Vec<String>>
}
impl ProfileMetadata {

}
#[derive(Debug)]
#[derive(Clone)]
#[skip_serializing_none]
#[derive(Serialize,Deserialize)]
#[serde(rename = "source")]
pub struct Source {
    pub id : Option<String>,
    pub etag: Option<String>,
    #[serde(rename = "profileMetadata")]
    pub profile_metadata: Option<ProfileMetadata>,
    #[serde(rename = "type")]
    pub source_type: Option<String>,
    #[serde(rename = "updateTime")]
    pub update_time: Option<DateTime<Utc>>
}
impl Source {
    //#[allow(dead_code)]
    pub fn new() -> Source {
        Source{
            id:None,
            etag:None,
            profile_metadata:None,
            source_type:Some("CONTACT".into()),
            update_time:None
        }
    }
}
#[derive(Debug)]
#[derive(Clone)]
#[skip_serializing_none]
#[derive(Serialize,Deserialize)]
#[serde(rename = "metadata")]
pub struct Metadata {
    pub primary: Option<bool>,
    pub source: Option<Source>
    //#[serde(rename = "sourcePrimary")]
}
impl Metadata {
    #[allow(dead_code)]
    pub fn new() -> Metadata {
        Metadata{
            primary:None,
            source:Some(Source{
                id:None,
                source_type:Some(String::from("CONTACT")),
                etag:None,
                profile_metadata:None,
                update_time:None
            })

        }
    }
}
#[derive(Debug)]
#[derive(Clone)]
#[skip_serializing_none]
#[derive(Serialize,Deserialize)]
#[serde(rename = "metadata")]
pub struct PersonMetadata {
    #[serde(rename = "objectType")]
    pub object_type: Option<String>,
    pub sources:Option<Vec<Source>>
}
impl PersonMetadata {
    //#[allow(dead_code)]
    pub fn new(object_type: Option<String>, sources: Option<Vec<Source>>) -> PersonMetadata {
        PersonMetadata{
            object_type: object_type,
            sources: sources
        }
    }
}
#[derive(Debug)]
#[derive(Clone)]
#[skip_serializing_none]
#[derive(Serialize,Deserialize)]
#[serde(rename = "address")]
pub struct Address{
    pub city: Option<String>,
    #[serde(rename = "formattedType")]
    pub formatted_type: Option<String>,
    #[serde(rename = "formattedValue")]
    pub formatted_value: Option<String>,
    pub metadata: Option<Metadata>,
    #[serde(rename = "postalCode")]
    pub postal_code: Option<String>,
    pub region: Option<String>,
    #[serde(rename = "streetAddress")]
    pub street_address: Option<String>,
    #[serde(rename = "type")]
    pub address_type: Option<String>
}
// impl Address{
//     //#[allow(dead_code)]
//     pub fn new(city: Option<String>, formatted_type: Option<String> ,
//                formatted_value: Option<String>, metadata: Option<Metadata>,
//                postal_code: Option<String>, region: Option<String>,
//                street_address: Option<String>, address_type:Option<String>) -> Address {
//         Address {
//             city: city,
//             formatted_type: formatted_type,
//             formatted_value: formatted_value,
//             metadata: metadata,
//             postal_code: postal_code,
//             region: region,
//             street_address: street_address,
//             address_type: address_type
//         }
//     }
// }
#[derive(Debug)]
#[derive(Clone)]
#[skip_serializing_none]
#[derive(Serialize,Deserialize)]
#[serde(rename = "biography")]
pub struct Biography{
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
    pub metadata: Option<Metadata>,
    pub value: Option<String>
}
// impl Biography{
//     //#[allow(dead_code)]
//     pub fn new(content_type: Option<String>, metadata: Option<Metadata>,
//                value: Option<String>) -> Biography{
//         Biography{
//             content_type:content_type,
//             metadata:metadata,
//             value: value
//         }
//     }
// }
#[derive(Debug)]
#[derive(Clone)]
#[skip_serializing_none]
#[derive(Serialize,Deserialize)]
#[serde(rename = "emailAddress")]
pub struct EmailAddress{
    #[serde(rename = "formattedType")]
    pub formatted_type: Option<String>,
    pub metadata: Option<Metadata>,
    #[serde(rename = "type")]
    pub email_address_type: Option<String>,
    pub value: Option<String>
}
impl EmailAddress{
    //#[allow(dead_code)]
    pub fn new(formatted_type: Option<String>,metadata: Option<Metadata>,
               email_address_type: Option<String>, value: Option<String>) -> EmailAddress{
        EmailAddress{
            formatted_type:formatted_type,
            metadata:metadata,
            email_address_type:email_address_type,
            value: value
        }
    }
}
#[derive(Debug)]
#[derive(Clone)]
#[skip_serializing_none]
#[derive(Serialize,Deserialize)]
#[serde(rename = "name")]
pub struct Name{
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    #[serde(rename = "displayNameLastFirst")]
    pub display_name_last_first: Option<String>,
    #[serde(rename = "familyName")]
    pub family_name: Option<String>,
    #[serde(rename = "givenName")]
    pub given_name: Option<String>,
    pub metadata: Option<Metadata>,
    #[serde(rename = "unstructuredName")]
    pub unstructured_name:Option<String>
}
impl Name{
    //#[allow(dead_code)]
    pub fn new()->Name {
        Name{
            display_name:None,
            display_name_last_first:None,
            family_name:None,
            given_name:None,
            metadata:None,
            unstructured_name:None
        }
    }
}
#[derive(Debug)]
#[derive(Clone)]
#[skip_serializing_none]
#[derive(Serialize,Deserialize)]
#[serde(rename = "organization")]
pub struct Organization {
    #[serde(rename = "formattedType")]
    pub formatted_type: Option<String>,
    pub metadata: Option<Metadata>,
    pub name: Option<String>,
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub organization_type: Option<String>
}
// impl Organization {
//     //#[allow(dead_code)]
//     pub fn new(formatted_type: Option<String>, metadata: Option<Metadata>,
//     name: Option<String>, title: Option<String>, organization_type:Option<String>)->Organization {
//         Organization{
//             formatted_type:formatted_type,
//             metadata:metadata,
//             name:name,
//             title:title,
//             organization_type:organization_type
//         }
//     }
// }
#[derive(Debug)]
#[derive(Clone)]
#[skip_serializing_none]
#[derive(Serialize,Deserialize)]
#[serde(rename = "phoneNumber")]
pub struct PhoneNumber {
    #[serde(rename = "formattedType")]
    pub formatted_type: Option<String>,
    pub metadata: Option<Metadata>,
    #[serde(rename = "type")]
    pub phone_number_type: Option<String>,
    pub value: Option<String>
}
impl PhoneNumber{
    //#[allow(dead_code)]
    pub fn new(formatted_type: Option<String>, metadata: Option<Metadata>,
    phone_number_type: Option<String>, value:Option<String>)->PhoneNumber{
        PhoneNumber{
            formatted_type:formatted_type,
            metadata:metadata,
            phone_number_type:phone_number_type,
            value:value
        }
    }
}
#[derive(Debug)]
#[derive(Clone)]
#[skip_serializing_none]
#[derive(Serialize,Deserialize)]
#[serde(rename = "userDefined")]
pub struct UserDefined {
    pub key: Option<String>,
    pub metadata: Option<Metadata>,
    pub value: Option<String>
}
impl UserDefined{
     //#[allow(dead_code)]
     pub fn new(key:Option<String>,metadata:Option<Metadata>,value:Option<String>)->UserDefined{
         UserDefined{
             key:key,
             metadata:metadata,
             value:value
         }
     }
}
#[derive(Debug)]
#[derive(Clone)]
#[skip_serializing_none]
#[derive(Serialize,Deserialize)]
#[serde(rename = "person")]
pub struct GooglePerson {
    pub addresses:Option<Vec<Address>>,
    pub biographies:Option<Vec<Biography>>,
    #[serde(rename = "emailAddresses")]
    pub email_addresses:Option<Vec<EmailAddress>>,
    pub etag:Option<String>,
    pub metadata:Option<PersonMetadata>,
    pub names:Option<Vec<Name>>,
    pub organizations:Option<Vec<Organization>>,
    #[serde(rename = "phoneNumbers")]
    pub phone_numbers:Option<Vec<PhoneNumber>>,
    #[serde(rename = "resourceName")]
    pub resource_name:Option<String>,
    #[serde(rename = "userDefined")]
    pub user_defined:Option<Vec<UserDefined>>
}

#[derive(Debug)]
#[derive(Clone)]
pub struct LdapPerson{
    pub search_entry: Option<SearchEntry>
}


pub trait Person{
    fn get_field(&self,person_field:PersonField)->Result<Option<String>,Box<dyn std::error::Error>>;
    fn get_modify_timestamp(&self)->Result<Option<chrono::DateTime<Utc>>,Box<dyn std::error::Error>>;
    fn set_field(&mut self,person_field:PersonField,value:&str)->Result<(),Box<dyn std::error::Error>>;
    fn set_modify_timestamp(&mut self, newval: chrono::DateTime<Utc>)->Result<(),Box<dyn std::error::Error>>;
}

impl Person for LdapPerson {

    fn get_field(&self,person_field:PersonField)->Result<Option<String>,Box<dyn std::error::Error>>{
        let getter = |attribute_name:&str,index:usize|->Result<Option<String>,Box<dyn std::error::Error>>{
            if let Some(search_entry) = self.search_entry.as_ref()  &&
                let Some(attribute_vec) = search_entry.attrs.get(attribute_name)&&
                let Some(attr_val) = attribute_vec.get(index){
                    return Ok(Some(attr_val.into()));
            }
            Ok(None)
        };
        let res = match person_field {
            PersonField::FullName=>getter("cn",0),
            PersonField::FirstName=>getter("givenName",0),
            PersonField::LastName=>getter("sn",0),
            PersonField::Company=>getter("o",0),
            PersonField::Title=>getter("title",0),
            PersonField::Notes=>getter("description",0),
            PersonField::UserDefined=>getter("ou",0),
            PersonField::EmailWork=>getter("mail",0),
            PersonField::EmailHome=>getter("secondaryemail",0),
            PersonField::PhoneMobile1=>getter("mobile",0),
            PersonField::PhoneMobile2=>getter("mobile",1),
            PersonField::PhoneHome1=>getter("homePhone",0),
            PersonField::PhoneHome2=>getter("homePhone",1),
            PersonField::PhoneWork1=>getter("telephoneNumber",0),
            PersonField::PhoneWork2=>getter("telephoneNumber",1),
            PersonField::AddressStreetWork=>getter("street",0),
            PersonField::AddressCityWork=>getter("l",0),
            PersonField::AddressRegionWork=>getter("st",0),
            PersonField::AddressPostalCodeWork=>getter("postalCode",0),
            PersonField::AddressStreetHome=>getter("mozillaHomeStreet",0),
            PersonField::AddressCityHome=>getter("mozillaHomeLocalityName",0),
            PersonField::AddressRegionHome=>getter("mozillaHomeState",0),
            PersonField::AddressPostalCodeHome=>getter("mozillaHomePostalCode",0),
            // _=> return Ok(None),
        };
        res
    }

    fn get_modify_timestamp(&self) -> Result<Option<DateTime<Utc>>,Box<dyn std::error::Error>> {
        let ts_str = self.search_entry.as_ref().ok_or("no search entry")?.attrs.
            get("modifyTimestamp").ok_or("no modify timestamp")?.get(0).ok_or("no first index")?;
        // hack for making this a proper UTC
        let edited = String::from(&ts_str[0..14])+".000+0000";
        let fmt = "%Y%m%d%H%M%S%.f%z";
        let parse_result  = chrono::DateTime::parse_from_str(&edited,fmt);
        if let Ok(date_time) = parse_result{
            // println!("parse ok {:?}",date_time);
            //let datetime2 = DateTime::<Utc>::from_naive_utc_and_offset(date_time,L)
            return Ok(Some(date_time.into()));
            // return None;
        }else if let Err(e) = parse_result {
            println!("Could not parse {:?} because of {:?}",edited,e);
        }
        Ok(None)
    }
    fn set_field(&mut self,person_field:PersonField,value:&str)->Result<(),Box<dyn std::error::Error>>{
        let mut setter = |attribute_name:&str|->Result<(),Box<dyn std::error::Error>>{
            if self.search_entry.is_none(){
                let mut dn = String::from("cn=");
                dn.push_str(value);
                dn.push_str(",ou=Public,dc=pioneerind,dc=com");
                self.search_entry = Some(SearchEntry {
                    dn: dn,
                    attrs: HashMap::new(),
                    bin_attrs: HashMap::new()
                });
            }else{
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other,"Must add a CN first")))
            }
            if let Some(attr_vec) =self.search_entry.as_ref().
                ok_or("no search entry")?.attrs.get(attribute_name){
                let mut new_vec = attr_vec.clone().to_vec();
                new_vec.push(value.to_string());
                self.search_entry.as_mut().ok_or("no search entry")?.attrs.insert("cn".into(),new_vec);
            }else{
                self.search_entry.as_mut().ok_or("no search entry")?.attrs.insert("cn".into(),vec![value.to_string()]);
            }
            Ok(())
        };
        let res = match person_field{
            PersonField::FullName=>setter("cn"),
            PersonField::FirstName=>setter("givenName"),
            PersonField::LastName=>setter("sn"),
            PersonField::Company=>setter("o"),
            PersonField::Title=>setter("title"),
            PersonField::Notes=>setter("description"),
            PersonField::UserDefined=>setter("ou"),
            PersonField::EmailWork  =>setter("mail"),
            PersonField::EmailHome=>setter("mozillaSecondEmail"),
            PersonField::PhoneMobile1=>setter("mobile"),
            PersonField::PhoneMobile2=>setter("mobile"),
            PersonField::PhoneWork1=>setter("telephoneNumber"),
            PersonField::PhoneWork2=>setter("telephoneNumber"),
            PersonField::PhoneHome1=>setter("homePhone"),
            PersonField::PhoneHome2=>setter("homePhone"),
            PersonField::AddressStreetWork=>setter("street"),
            PersonField::AddressCityWork=>setter("l"),
            PersonField::AddressRegionWork=>setter("st"),
            PersonField::AddressPostalCodeWork=>setter("postalCode"),
            PersonField::AddressStreetHome=>setter("mozillaHomeStreet"),
            PersonField::AddressCityHome=>setter("mozillaHomeLocalityName"),
            PersonField::AddressRegionHome=>setter("mozillaHomeState"),
            PersonField::AddressPostalCodeHome=>setter("mozillaHomePostalCode"),
            // _=>Ok(())
        };
        res
        // match pers
    }

    fn set_modify_timestamp(&mut self, newval: DateTime<Utc>)->Result<(),Box<dyn std::error::Error>> {
        let fmt = "%Y%m%d%H%M%SZ";
        let formatted = format!("{}", newval.format(fmt));
        println!("{}", formatted);
        if self.search_entry.is_none(){
            self.search_entry = Some(SearchEntry{
                // "cn=Example Person,ou=Public,dc=pioneerind,dc=com" is proper DN.
                // Initialize the with suffix
                dn:"ou=Public,dc=pioneerind,dc=com".to_string(),
                attrs:HashMap::new(),
                bin_attrs:HashMap::new()
            });
        }
        self.search_entry.as_mut().ok_or("no search entry")?.attrs.insert("modifyTimestamp".into(),vec![formatted]);
        Ok(())
    }

}

impl Person for GooglePerson {
    fn get_field(&self,person_field:PersonField)->Result<Option<String>,Box<dyn std::error::Error>> {
        let get_street_address = |selected_address_type:&str|->Result<Option<String>,Box<dyn std::error::Error>>{
            if let Some(addresses) = self.addresses.as_ref(){
                for address in addresses{
                    if let Some(address_type) = address.address_type.as_ref() && address_type==selected_address_type{
                        return Ok(address.street_address.clone());
                    }
                }
            }
            Ok(None)
        };
        let get_city = |selected_address_type:&str|->Result<Option<String>,Box<dyn std::error::Error>>{
            if let Some(addresses) = self.addresses.as_ref() {
                for address in addresses{
                    if let Some(address_type) = address.address_type.as_ref()  &&  address_type==selected_address_type {
                        return Ok(address.city.clone());
                    }
                }
            }
            Ok(None)
        };
        let get_region = |selected_address_type:&str|->Result<Option<String>,Box<dyn std::error::Error>>{
            if let Some(addresses) = self.addresses.as_ref(){
                for address in addresses{
                    if let Some(address_type) = address.address_type.as_ref()  &&  address_type==selected_address_type {
                        return Ok(address.region.clone());
                    }
                }
            }
            Ok(None)
        };
        let get_postal_code = |selected_address_type:&str|->Result<Option<String>,Box<dyn std::error::Error>>{
            if let Some(addresses) = self.addresses.as_ref(){
                for address in addresses{
                    if let Some(address_type) = address.address_type.as_ref()  &&  address_type==selected_address_type {
                        return Ok(address.postal_code.clone());
                    }
                }
            }
            Ok(None)
        };
        let get_email  = |email_type:&str|->Result<Option<String>,Box<dyn std::error::Error>>{
            if let Some(email_addresses) = self.email_addresses.as_ref() {
                for email_address in email_addresses {
                    if let Some(email_address_type) = email_address.email_address_type.as_ref()&&  email_address_type==email_type{
                        return Ok(email_address.value.clone());
                    }
                }
            }
            Ok(None)
        };
        let get_phone_number = |selected_phone_number_type:&str,selected_index:usize|->Result<Option<String>,Box<dyn std::error::Error>>{
            if let Some(phone_numbers) = self.phone_numbers.as_ref() {
                let mut index = 0;
                for phone_number in phone_numbers {
                    if let Some(phone_number_type) = phone_number.phone_number_type.as_ref() &&
                        phone_number_type == selected_phone_number_type &&
                        selected_index == index {
                        return Ok(phone_number.value.clone());
                    }
                    index += 1;
                }
            }
            Ok(None)
        };
        let get_name = ||->Result<Option<Name>,Box<dyn std::error::Error>>{
            if let Some(names) =self.names.as_ref() && let Some(first) = names.first(){
                return Ok(Some(first.clone()));
            }
            Ok(None)
        };
        let get_org = ||->Result<Option<Organization>,Box<dyn std::error::Error>>{
            if let Some(organizations) = self.organizations.as_ref(){
                for organization in organizations {
                    if let Some(org_type) = organization.organization_type.as_ref() &&
                        org_type=="work"{
                            return Ok(Some(organization.clone()));
                    }
                }
            }
            Ok(None)
        };
        match person_field {
            PersonField::FullName=>{
                if let Ok(res) = get_name() && let Some(name) = res{
                    return Ok(name.display_name.clone());
                }
            },
            PersonField::FirstName=>{
                if let Ok(res) = get_name() && let Some(name) = res{
                    return Ok(name.given_name.clone());
                }
            },
            PersonField::LastName=>{
                if let Ok(res) = get_name() && let Some(name) = res{
                    return Ok(name.family_name.clone());
                }
            },
            PersonField::Company=>{
                if let Ok(res) = get_org() && let Some(organization) = res{
                    return Ok(organization.name.clone());
                }
            }
            PersonField::Title=>{
                if let Ok(res) = get_org() && let Some(organization) = res{
                    return Ok(organization.title.clone());
                }
            }
            PersonField::Notes=>{
                if let Some(biographies) = self.biographies.as_ref() && let Some(biography) = biographies.first(){
                    return Ok(biography.value.clone());
                }
            }
            PersonField::UserDefined=>{
                if let Some(vec_user_defined) = self.user_defined.as_ref() &&  let Some(user_defined) = vec_user_defined.first(){
                    return Ok(user_defined.value.clone());
                }
            }
            PersonField::EmailWork=>return get_email("work"),
            PersonField::EmailHome=>return get_email("home"),
            PersonField::PhoneWork1=>return get_phone_number("work",0),
            PersonField::PhoneWork2=>return get_phone_number("work",1),
            PersonField::PhoneHome1=>return get_phone_number("home",0),
            PersonField::PhoneHome2=>return get_phone_number("home",1),
            PersonField::PhoneMobile1=>return get_phone_number("mobile",0),
            PersonField::PhoneMobile2=>return get_phone_number("mobile",1),
            PersonField::AddressStreetWork=>return get_street_address("work"),
            PersonField::AddressStreetHome=>return get_street_address("home"),
            PersonField::AddressCityWork=>return get_city("work"),
            PersonField::AddressCityHome=>return get_city("home"),
            PersonField::AddressRegionWork=>return get_region("work"),
            PersonField::AddressRegionHome=>return get_region("home"),
            PersonField::AddressPostalCodeWork=>return get_postal_code("work"),
            PersonField::AddressPostalCodeHome=>return get_postal_code("home"),
        }
        Ok(None)
    }
    fn get_modify_timestamp(&self) -> Result<Option<DateTime<Utc>>,Box<dyn std::error::Error>> {
        let source = self.metadata.as_ref().ok_or("no metadata")?.sources.as_ref().
            ok_or("no sources")?.first().ok_or("no first element")?;
        let source_type = source.source_type.as_ref().ok_or("")?;
        if source_type == "CONTACT" {
            return Ok(source.update_time);
        }
        Ok(None)
    }
    fn set_field(&mut self, person_field: PersonField, value: &str) ->Result<(),Box<dyn std::error::Error>> {
        let mut set_email = |email_type: &str |->Result<(),Box<dyn std::error::Error>>{
            if self.email_addresses.is_none(){
                let mut vec:Vec<EmailAddress> = Vec::new();
                vec.push(EmailAddress::new(None,None,None,None));
                self.email_addresses = Some(vec);
            }
            let orig_vec = self.email_addresses.as_mut().unwrap();
            orig_vec[0].email_address_type = Some(String::from(email_type));
            orig_vec[0].value = Some(String::from(value));
            Ok(())
        };
        let mut set_phone = |selected_type:&str|->Result<(),Box<dyn std::error::Error>> {
            let phone_number = PhoneNumber::new(None,None,Some(selected_type.into()),Some(value.into()));
            if self.phone_numbers.is_none() {
                self.phone_numbers = Some(Vec::new());
                // let mut vec: Vec<PhoneNumber> = Vec::new();
            }
            self.phone_numbers.as_mut().unwrap().push(phone_number);
            Ok(())
        };
        let mut set_sync = ||->Result<(),Box<dyn std::error::Error>> {
            let user_defined = UserDefined::new(Some("ldap".into()),None,Some("sync".into()));
            if self.user_defined.is_none() {
                self.user_defined = Some(Vec::new());
            }
            self.user_defined.as_mut().unwrap().push(user_defined);
            Ok(())
        };
        match person_field{
            PersonField::FullName=>{
                if self.names.is_none(){
                    let mut name_vec = Vec::new();
                    name_vec.push(Name::new());
                    self.names = Some(name_vec);
                }
                let orig_vec = self.names.as_mut().unwrap();
                orig_vec[0].display_name = Some(String::from(value));
                return Ok(())
            },
            PersonField::FirstName=>{
                if self.names.is_none(){
                    let mut name_vec = Vec::new();
                    name_vec.push(Name::new());
                    self.names = Some(name_vec);
                }
                let orig_vec = self.names.as_mut().unwrap();
                orig_vec[0].given_name = Some(String::from(value));
                return Ok(())
            },
            PersonField::LastName=>{
                if self.names.is_none(){
                    let mut name_vec = Vec::new();
                    name_vec.push(Name::new());
                    self.names = Some(name_vec);
                }
                let orig_vec = self.names.as_mut().unwrap();
                orig_vec[0].family_name = Some(String::from(value));
                return Ok(())
            },
            PersonField::EmailWork=>set_email("work")?,
            PersonField::EmailHome=>set_email("home")?,
            PersonField::PhoneMobile1=>set_phone("mobile")?,
            PersonField::UserDefined=>set_sync()?,
            _=>()
        }
        Ok(())
    }

    fn set_modify_timestamp(& mut self, newval: DateTime<Utc>) ->Result<(),Box<dyn std::error::Error>> {
        if self.metadata.is_none() {
            //2025-11-11T19:54:21.353790Z
            // let fmt = "%Y-%m-%dT%H:%M:%S%.f%Z";
            // let formatted = format!("{}", newval.format(fmt));
            // println!("{}", formatted);

            let mut source = Source::new();
            source.source_type = Some(String::from("CONTACT"));
            source.update_time = Some(newval);
            let sources_vec:Vec<Source> = vec![source];
            self.metadata = Some(PersonMetadata::new(Some("PERSON".into()),Some(sources_vec)));
        }
        // let source = self.metadata.as_ref().ok_or("no metadata")?.sources.as_ref().
        //     ok_or("no sources")?.first().ok_or("no first element")?;
        // let source_type = source.source_type.as_ref().ok_or("")?;
        // if source_type == "CONTACT" {
        //     return Ok(source.update_time);
        // }
        Ok(())
    }


}

impl LdapPerson{
    // fn make_dn(google_person:&GooglePerson)->Result<String,Box<dyn std::error::Error>>{
    //     let dn = String::from("cn=")+&google_person.get_field(PersonField::FullName)?.
    //         ok_or("cannot get full name")?+",ou=Public,dc=pioneerind,dc=com";
    //     Ok(dn)
    // }

    pub fn get_subscriptions(&self)->Option<Vec<String>>{
        if let Some(search_entry)= self.search_entry.as_ref() &&
            let Some(attr_vec) = search_entry.attrs.get("ou"){
            return Some(attr_vec.clone());

          }
        None
    }
    pub fn set_subscriptions(&mut self, subscriptions: &Vec<String>)->(){
        if let Some(search_entry)= self.search_entry.as_mut() {
            search_entry.attrs.insert("ou".to_string(),subscriptions.clone());

        }
        ()
    }

    pub fn from_ldap(search_entry: SearchEntry)->Result<LdapPerson,Box<dyn std::error::Error>>{
        let ldap_person = LdapPerson{
            search_entry:Some(search_entry)
        };
        Ok(ldap_person)
    }

    //#[allow(dead_code)]
    pub fn from_google(google_person: &GooglePerson)->Result<LdapPerson,Box<dyn std::error::Error>> {
        let create_search_entry = ||->Result<SearchEntry,Box<dyn std::error::Error>> {
            let make_dn = ||->Result<String,Box<dyn std::error::Error>> {
                let dn = String::from("cn=")+&google_person.get_field(PersonField::FullName)?.
                    ok_or("cannot get full name")?+",ou=Public,dc=pioneerind,dc=com";
                Ok(dn)
            };

            let make_attrs =||->Result<HashMap<String,Vec<String>>,Box<dyn std::error::Error>>{
                let mut attrs = HashMap::new();
                
                let mut make_double_attr = |attr_name:&str, person_field1: PersonField,person_field2_option:Option<PersonField>|->Result<(),Box<dyn std::error::Error>>{
                    let mut attr_vec = Vec::new();
                    if let Ok(res) = google_person.get_field(person_field1) &&
                        let Some(field_value) = res {
                        attr_vec.push(field_value);
                    }
                    if let Some(person_field2) = person_field2_option &&
                        let Ok(res) = google_person.get_field(person_field2) &&                        
                        let Some(field_value) = res {
                        attr_vec.push(field_value);
                    }
                    if attr_vec.len() > 0 {
                        attrs.insert(String::from(attr_name),attr_vec);
                    }
                    Ok(())
                };
                make_double_attr("cn",PersonField::FullName,None)?;
                make_double_attr("givenName",PersonField::FirstName,None)?;
                make_double_attr("sn",PersonField::LastName,None)?;
                make_double_attr("mail",PersonField::EmailWork,None)?;
                make_double_attr("o",PersonField::Company,None)?;
                make_double_attr("title",PersonField::Title,None)?;
                make_double_attr("description",PersonField::Notes,None)?;
                make_double_attr("mozillaSecondEmail",PersonField::EmailHome,None)?;
                make_double_attr("telephoneNumber",PersonField::PhoneWork1,Some(PersonField::PhoneWork2))?;
                make_double_attr("homePhone",PersonField::PhoneHome1,Some(PersonField::PhoneHome2))?;
                make_double_attr("mobile",PersonField::PhoneMobile1,Some(PersonField::PhoneMobile2))?;
                make_double_attr("street",PersonField::AddressStreetWork,None)?;
                make_double_attr("l",PersonField::AddressCityWork,None)?;
                make_double_attr("st",PersonField::AddressRegionWork,None)?;
                make_double_attr("postalCode",PersonField::AddressPostalCodeWork,None)?;
                make_double_attr("mozillaHomeStreet",PersonField::AddressStreetHome,None)?;
                make_double_attr("mozillaHomeLocalityName",PersonField::AddressCityHome,None)?;
                make_double_attr("mozillaHomeState",PersonField::AddressRegionHome,None)?;
                make_double_attr("mozillaHomePostalCode",PersonField::AddressPostalCodeHome,None)?;
                Ok(attrs)
            };
            let make_bin_attrs = ||->Result<HashMap<String,Vec<Vec<u8>>>,Box<dyn std::error::Error>>{
                let bin_attrs = HashMap::new();
                Ok(bin_attrs)
            };
            Ok(
                SearchEntry{
                    dn:make_dn()?,
                    attrs:make_attrs()?,
                    bin_attrs:make_bin_attrs()?
                }
            )
        };
        let ldap_person = LdapPerson{
            search_entry:Some(create_search_entry()?)
        };
        // let timestamp = google_person.get_modify_timestamp()?.ok_or("Can't get timestamp")?;
        // ldap_person.set_modify_timestamp(timestamp)?;
        // ldap_person.set_field(PersonField::UserDefined,contact_group)?;
        Ok(ldap_person)
    }

}

impl GooglePerson{
    //#[allow(dead_code)]
    pub fn is_synced(&self)->bool{
        if let Ok(res) = self.get_field(PersonField::UserDefined) {
            // println!("got user defined");
            if let Some(sync_field) = res {
                // println!("got sync field of value {:?}", sync_field);
                if sync_field=="sync" {
                    // println!("returning true");
                    return true;
                }
            }

        }
        false
    }



    pub fn set_synced(&mut self)->() {
        let _res = self.set_field(PersonField::UserDefined,"sync");

        ()
    }

    pub fn new(orig_google_person:Option<&GooglePerson>,ldap_person: &LdapPerson)->Result<GooglePerson,Box<dyn std::error::Error>> {
        let get_addresses = ||->Result<Vec<Address>,Box<dyn std::error::Error>> {
            Ok(vec![
                Address{
                    city:ldap_person.get_field(PersonField::AddressCityWork)?,
                    formatted_type:Some(String::from("Work")),
                    formatted_value:None,
                    metadata:Some(Metadata::new()),
                    postal_code:ldap_person.get_field(PersonField::AddressPostalCodeWork)?,
                    region:ldap_person.get_field(PersonField::AddressRegionWork)?,
                    street_address:ldap_person.get_field(PersonField::AddressStreetWork)?,
                    address_type:Some(String::from("work"))
                },
                Address{
                    city:ldap_person.get_field(PersonField::AddressCityWork)?,
                    formatted_type:Some(String::from("Home")),
                    formatted_value:None,
                    metadata:Some(Metadata::new()),
                    postal_code:ldap_person.get_field(PersonField::AddressPostalCodeWork)?,
                    region:ldap_person.get_field(PersonField::AddressRegionWork)?,
                    street_address:ldap_person.get_field(PersonField::AddressStreetWork)?,
                    address_type:Some(String::from("home"))
                }
            ])
        };
        let get_emails = ||->Result<Vec<EmailAddress>,Box<dyn std::error::Error>> {
            Ok(vec![
                EmailAddress{
                    formatted_type:Some(String::from("Work")),
                    value:ldap_person.get_field(PersonField::EmailWork)?,
                    metadata:Some(Metadata::new()),
                    email_address_type:Some(String::from("work"))
                },
                EmailAddress{
                    formatted_type:Some(String::from("Home")),
                    value:ldap_person.get_field(PersonField::EmailHome)?,
                    metadata:Some(Metadata::new()),
                    email_address_type:Some(String::from("home"))
                }
            ])
        };
        let get_names = ||->Result<Vec<Name>,Box<dyn std::error::Error>> {
            Ok(vec![
                Name
                {
                    display_name:ldap_person.get_field(PersonField::FullName)?,
                    family_name:ldap_person.get_field(PersonField::LastName)?,
                    given_name:ldap_person.get_field(PersonField::FirstName)?,
                    display_name_last_first:None,
                    unstructured_name:None,
                    metadata:Some(Metadata::new())
                }
            ])
        };
        let get_phones = ||->Result<Vec<PhoneNumber>,Box<dyn std::error::Error>> {
            Ok(vec![
                PhoneNumber
                {
                    formatted_type:Some("Mobile".into()),
                    metadata:Some(Metadata::new()),
                    phone_number_type:Some("mobile".into()),
                    value:ldap_person.get_field(PersonField::PhoneMobile1)?
                },
                PhoneNumber
                {
                    formatted_type:Some("Mobile".into()),
                    metadata:Some(Metadata::new()),
                    phone_number_type:Some("mobile".into()),
                    value:ldap_person.get_field(PersonField::PhoneMobile2)?
                },
                PhoneNumber
                {
                    formatted_type:Some("Work".into()),
                    metadata:Some(Metadata::new()),
                    phone_number_type:Some("work".into()),
                    value:ldap_person.get_field(PersonField::PhoneWork1)?
                },
                PhoneNumber
                {
                    formatted_type:Some("Work".into()),
                    metadata:Some(Metadata::new()),
                    phone_number_type:Some("work".into()),
                    value:ldap_person.get_field(PersonField::PhoneWork2)?
                },
                PhoneNumber
                {
                    formatted_type:Some("Home".into()),
                    metadata:Some(Metadata::new()),
                    phone_number_type:Some("home".into()),
                    value:ldap_person.get_field(PersonField::PhoneHome1)?
                },
                PhoneNumber
                {
                    formatted_type:Some("Home".into()),
                    metadata:Some(Metadata::new()),
                    phone_number_type:Some("home".into()),
                    value:ldap_person.get_field(PersonField::PhoneHome2)?
                },
            ])
        };
        let get_organizations = ||->Result<Vec<Organization>,Box<dyn std::error::Error>> {
            Ok(vec![
                Organization
                {
                    formatted_type:None,
                    organization_type:None,
                    name:ldap_person.get_field(PersonField::Company)?,
                    title:ldap_person.get_field(PersonField::Title)?,
                    metadata:Some(Metadata::new())
                }
            ])
        };
        let get_biographies = ||->Result<Vec<Biography>,Box<dyn std::error::Error>> {
            Ok(vec![
                Biography{
                    content_type:Some(String::from("TEXT_PLAIN")),
                    value:ldap_person.get_field(PersonField::Notes)?,
                    metadata:Some(Metadata::new())
                }
            ])
        };
        let get_user_defined = ||->Result<Vec<UserDefined>,Box<dyn std::error::Error>> {
            Ok(vec![
                UserDefined{
                    key:Some("ldap".into()),
                    metadata:Some(Metadata::new()),
                    value:Some("sync".into())
                }
            ])
        };
        let mut resource_name:Option<String> = None;
        let mut etag:Option<String> = None;
        let mut person_metadata:Option<PersonMetadata> = Some(PersonMetadata{
            object_type:Some(String::from("PERSON")),
            sources:Some(vec![
                Source::new()
            ])
        });
        if let Some(google_person) = orig_google_person{
            resource_name = google_person.resource_name.clone();
            etag = google_person.etag.clone();
            person_metadata = google_person.metadata.clone();
        }
        let google_person = GooglePerson{
            addresses: Some(get_addresses()?),
            biographies: Some(get_biographies()?),
            email_addresses: Some(get_emails()?),
            etag: etag,
            metadata: person_metadata,
            names: Some(get_names()?),
            organizations: Some(get_organizations()?),
            phone_numbers:Some(get_phones()?),
            resource_name: resource_name,
            user_defined: Some(get_user_defined()?)
        };
        Ok(google_person)
    }

    // pub fn new(addresses: Option<Vec<Address>>, biographies: Option<Vec<Biography>>,
    //            email_addresses: Option<Vec<EmailAddress>>, etag:Option<String>,
    //            metadata:Option<PersonMetadata>, names:Option<Vec<Name>>,
    //            organizations:Option<Vec<Organization>>,phone_numbers:Option<Vec<PhoneNumber>>,
    //            resource_name:Option<String>,user_defined:Option<Vec<UserDefined>>)->GooglePerson{
    //     GooglePerson {
    //         addresses: addresses,
    //         biographies: biographies,
    //         email_addresses: email_addresses,
    //         etag: etag,
    //         metadata: metadata,
    //         names: names,
    //         organizations: organizations,
    //         phone_numbers:phone_numbers,
    //         resource_name: resource_name,
    //         user_defined: user_defined
    //     }
    // }
}