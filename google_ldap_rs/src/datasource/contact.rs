use std::string::String;
// use std::arch::global_asm;
// use std::arch::x86_64::_mm256_mask_unpacklo_pd;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use ldap3::SearchEntry;
// use rocket::time::Date;
// use rocket::yansi::Paint;

// use serde_json::Value::String;
// use crate::datasource::contact;

#[allow(dead_code)]

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
#[derive(Serialize,Deserialize)]
#[serde(rename = "profileMetadata")]
pub struct ProfileMetadata {
    #[serde(rename = "objectType")]
    pub object_type: Option<String>,
    #[serde(rename = "userTypes")]
    pub user_types:Option<Vec<String>>
}
impl ProfileMetadata {
    #[allow(dead_code)]
    pub fn new(object_type: Option<String>, user_types: Option<Vec<String>>) -> ProfileMetadata {
        ProfileMetadata{
            object_type: object_type,
            user_types: user_types
        }
    }
}
#[derive(Debug)]
#[derive(Clone)]
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
    #[allow(dead_code)]
    pub fn new(id: Option<String>, etag: Option<String>, profile_metadata: Option<ProfileMetadata>,
    source_type:Option<String>,update_time:Option<DateTime<Utc>>) -> Source {
        Source{
            id,
            etag,
            profile_metadata,
            source_type,
            update_time
        }
    }
}
#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize,Deserialize)]
#[serde(rename = "metadata")]
pub struct Metadata {
    pub primary: Option<bool>,
    pub source: Option<Source>
}
impl Metadata {
    #[allow(dead_code)]
    pub fn new(primary: Option<bool>, source: Option<Source>) -> Metadata {
        Metadata{
            primary:primary,
            source: source
        }
    }
}
#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize,Deserialize)]
#[serde(rename = "metadata")]
pub struct PersonMetadata {
    #[serde(rename = "objectType")]
    pub object_type: Option<String>,
    pub sources:Option<Vec<Source>>
}
impl PersonMetadata {
    #[allow(dead_code)]
    pub fn new(object_type: Option<String>, sources: Option<Vec<Source>>) -> PersonMetadata {
        PersonMetadata{
            object_type: object_type,
            sources: sources
        }
    }
}
#[derive(Debug)]
#[derive(Clone)]
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
impl Address{
    #[allow(dead_code)]
    pub fn new(city: Option<String>, formatted_type: Option<String> ,
               formatted_value: Option<String>, metadata: Option<Metadata>,
               postal_code: Option<String>, region: Option<String>,
               street_address: Option<String>, address_type:Option<String>) -> Address {
        Address {
            city: city,
            formatted_type: formatted_type,
            formatted_value: formatted_value,
            metadata: metadata,
            postal_code: postal_code,
            region: region,
            street_address: street_address,
            address_type: address_type
        }
    }
}
#[derive(Debug)]
#[derive(Clone)]
#[derive(Serialize,Deserialize)]
#[serde(rename = "biography")]
pub struct Biography{
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
    pub metadata: Option<Metadata>,
    pub value: Option<String>
}
impl Biography{
    #[allow(dead_code)]
    pub fn new(content_type: Option<String>, metadata: Option<Metadata>,
               value: Option<String>) -> Biography{
        Biography{
            content_type:content_type,
            metadata:metadata,
            value: value
        }
    }
}
#[derive(Debug)]
#[derive(Clone)]
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
    #[allow(dead_code)]
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
    #[serde(rename = "sourcePrimary")]
    pub source_primary: Option<bool>,
    #[serde(rename = "unstructuredName")]
    pub unstructured_name:Option<String>
}
impl Name{
    #[allow(dead_code)]
    pub fn new(display_name: Option<String>,display_name_last_first: Option<String>,
               family_name: Option<String>, given_name:Option<String>, metadata: Option<Metadata>,
               source_primary: Option<bool>, unstructured_name:Option<String>)->Name {
        Name{
            display_name:display_name,
            display_name_last_first:display_name_last_first,
            family_name:family_name,
            given_name:given_name,
            metadata:metadata,
            source_primary:source_primary,
            unstructured_name:unstructured_name
        }
    }
}
#[derive(Debug)]
#[derive(Clone)]
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
impl Organization {
    #[allow(dead_code)]
    pub fn new(formatted_type: Option<String>, metadata: Option<Metadata>,
    name: Option<String>, title: Option<String>, organization_type:Option<String>)->Organization {
        Organization{
            formatted_type:formatted_type,
            metadata:metadata,
            name:name,
            title:title,
            organization_type:organization_type
        }
    }
}
#[derive(Debug)]
#[derive(Clone)]
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
    #[allow(dead_code)]
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
#[derive(Serialize,Deserialize)]
#[serde(rename = "userDefined")]
pub struct UserDefined {
    pub key: String,
    pub metadata: Metadata,
    pub value: String
}
impl UserDefined{
    #[allow(dead_code)]
    pub fn new(key:String,metadata:Metadata,value:String)->UserDefined{
        UserDefined{
            key:key,
            metadata:metadata,
            value:value
        }
    }
}
#[derive(Debug)]
#[derive(Clone)]
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
impl GooglePerson{
    #[allow(dead_code)]
    pub fn new(addresses: Option<Vec<Address>>, biographies: Option<Vec<Biography>>,
               email_addresses: Option<Vec<EmailAddress>>, etag:Option<String>,
               metadata:Option<PersonMetadata>, names:Option<Vec<Name>>,
               organizations:Option<Vec<Organization>>,phone_numbers:Option<Vec<PhoneNumber>>,
               resource_name:Option<String>,user_defined:Option<Vec<UserDefined>>)->GooglePerson{
        GooglePerson {
            addresses: addresses,
            biographies: biographies,
            email_addresses: email_addresses,
            etag: etag,
            metadata: metadata,
            names: names,
            organizations: organizations,
            phone_numbers:phone_numbers,
            resource_name: resource_name,
            user_defined: user_defined
        }
    }
}
pub struct LdapPerson{
    pub search_entry: Option<SearchEntry>
}

impl LdapPerson{
    #[allow(dead_code)]
    pub fn new(search_entry:Option<SearchEntry>)->LdapPerson{
        LdapPerson{
            search_entry:search_entry
        }
    }
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
            return Ok(Some(self.search_entry.as_ref().ok_or("no search entry")?.
                attrs.get(attribute_name).ok_or("no SN")?.get(index).ok_or("no first index")?.
                to_string()));
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
            println!("parse ok {:?}",date_time);
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
                if attribute_name=="cn" {
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
        let get_email  = |email_type:&str|->Result<Option<String>,Box<dyn std::error::Error>>{
            let email_addresses = self.email_addresses.as_ref().ok_or("no names vec")?;
            for email_address in email_addresses {
                let email_address_type = email_address.email_address_type.as_ref().ok_or("no email address type")?;
                if email_address_type==email_type{
                    return Ok(email_address.value.clone());
                }
            }
            Ok(None)
        };
        let get_phone_number = |selected_phone_number_type:&str,selected_index:usize|->Result<Option<String>,Box<dyn std::error::Error>>{
            let phone_numbers = self.phone_numbers.as_ref().ok_or("no names vec")?;
            let mut index = 0;
            for phone_number in phone_numbers {
                let phone_number_type = phone_number.phone_number_type.as_ref().ok_or("no phone number type")?;
                if phone_number_type==selected_phone_number_type {
                    if selected_index==index {
                        return Ok(phone_number.value.clone());
                    }
                    index+=1;
                }
            }
            Ok(None)
        };
        match person_field {
            PersonField::FullName=>{
                return Ok(Some(self.names.as_ref().ok_or("no names vec")?.
                    first().ok_or("no first element")?.display_name.as_ref().
                    ok_or("no display name")?.to_string()));
            },
            PersonField::FirstName=>{
                return Ok(Some(self.names.as_ref().ok_or("no names vec")?.
                    first().ok_or("no first element")?.given_name.as_ref().
                    ok_or("no display name")?.to_string()));
            },
            PersonField::LastName=>{
                return Ok(Some(self.names.as_ref().ok_or("no names vec")?.
                    first().ok_or("no first element")?.family_name.as_ref().
                    ok_or("no display name")?.to_string()));
            },
            PersonField::Company=>{
                let organizations = self.organizations.as_ref().ok_or("no names vec")?;
                for organization in organizations {
                    let org_type = organization.organization_type.as_ref().ok_or("no organization type")?;
                    if org_type=="work"{
                        return Ok(organization.name.clone());
                    }
                }
            }
            PersonField::Title=>{
                let organizations = self.organizations.as_ref().ok_or("no names vec")?;
                for organization in organizations {
                    let org_type = organization.organization_type.as_ref().ok_or("no organization type")?;
                    if org_type=="work"{
                        return Ok(organization.title.clone());
                    }
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
            _ => ()
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
        match person_field{
            PersonField::FullName=>{
                if self.names.is_none(){
                    let mut name_vec = Vec::new();
                    name_vec.push(Name::new(None,None,None,None,None,None,None));
                    self.names = Some(name_vec);
                }
                let orig_vec = self.names.as_mut().unwrap();
                orig_vec[0].display_name = Some(String::from(value));
            }
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

            let source = Source::new(None,None,None,Some("CONTACT".into()),Some(newval));
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


    // fn get_email_work(&self) -> Option<String> {
    //     if let Some(emails) = &self.email_addresses{
    //         for email_address in emails {
    //             if let Some(email_address_type)=&email_address.email_address_type{
    //                 if email_address_type=="work"{
    //                     if let Some(email_value) = &email_address.value{
    //                         return Some(email_value.to_string());
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     None
    // }
    //
    // fn get_email_home(&self) -> Option<String> {
    //     if let Some(emails) = &self.email_addresses{
    //         for email_address in emails {
    //             if let Some(email_address_type)=&email_address.email_address_type{
    //                 if let Some(email_value) = &email_address.value{
    //                     return Some(email_value.to_string());
    //                 }
    //             }
    //         }
    //     }
    //     None
    // }
    //
    // fn get_company(&self) -> Option<String> {
    //     if let Some(organizations) = &self.organizations{
    //         for organization in organizations {
    //             if let Some(organization_type)=&organization.organization_type{
    //                 if organization_type=="work"{
    //                     if let Some(org_name) = &organization.name{
    //                         return Some(org_name.to_string());
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     None
    // }
    //
    // fn get_job_title(&self) -> Option<String> {
    //     if let Some(organizations) = &self.organizations{
    //         for organization in organizations {
    //             if let Some(organization_type)=&organization.organization_type{
    //                 if organization_type=="work"{
    //                     if let Some(org_title) = &organization.title{
    //                         return Some(org_title.to_string());
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     None
    // }
    //
    // fn get_phone_number_work1(&self) -> Option<String> {
    //     return phone_number_helper(&self,"work",0);
    //
    // }
    //
    // fn get_phone_number_work2(&self) -> Option<String> {
    //     return phone_number_helper(&self,"work",1);
    // }
    //
    // fn get_phone_number_home1(&self) -> Option<String> {
    //     return phone_number_helper(&self,"home",0);
    // }
    //
    // fn get_phone_number_home2(&self) -> Option<String> {
    //     return phone_number_helper(&self,"home",1);
    // }
    //
    // fn get_phone_number_mobile1(&self) -> Option<String> {
    //     return phone_number_helper(&self,"mobile",0);
    // }
    //
    // fn get_phone_number_mobile2(&self) -> Option<String> {
    //     return phone_number_helper(&self,"mobile",1);
    // }
    //
    // fn get_street_work(&self) -> Option<String> {
    //     if let Some(addresses) = &self.addresses{
    //         for address in addresses {
    //             if let Some(address_type)=&address.address_type{
    //                 if address_type=="work"{
    //                     if let Some(street_address) = &address.street_address {
    //                         return Some(street_address.to_string());
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     None
    // }
    //
    // fn get_street_home(&self) -> Option<String> {
    //     if let Some(addresses) = &self.addresses{
    //         for address in addresses {
    //             if let Some(address_type)=&address.address_type{
    //                 if address_type=="home"{
    //                     if let Some(street_address) = &address.street_address {
    //                         return Some(street_address.to_string());
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     None
    // }
    //
    // fn get_city_work(&self) -> Option<String> {
    //     if let Some(addresses) = &self.addresses{
    //         for address in addresses {
    //             if let Some(address_type)=&address.address_type{
    //                 if address_type=="work"{
    //                     if let Some(city) = &address.city {
    //                         return Some(city.to_string());
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     None
    // }
    //
    // fn get_city_home(&self) -> Option<String> {
    //     if let Some(addresses) = &self.addresses{
    //         for address in addresses {
    //             if let Some(address_type)=&address.address_type{
    //                 if address_type=="home"{
    //                     if let Some(city) = &address.city {
    //                         return Some(city.to_string());
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     None
    // }
    //
    // fn get_state_work(&self) -> Option<String> {
    //     if let Some(addresses) = &self.addresses{
    //         for address in addresses {
    //             if let Some(address_type)=&address.address_type{
    //                 if address_type=="work"{
    //                     if let Some(region) = &address.region {
    //                         return Some(region.to_string());
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     None
    // }
    //
    // fn get_state_home(&self) -> Option<String> {
    //     if let Some(addresses) = &self.addresses{
    //         for address in addresses {
    //             if let Some(address_type)=&address.address_type{
    //                 if address_type=="home"{
    //                     if let Some(region) = &address.region {
    //                         return Some(region.to_string());
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     None
    // }
    //
    // fn get_zip_work(&self) -> Option<String> {
    //     if let Some(addresses) = &self.addresses{
    //         for address in addresses {
    //             if let Some(address_type)=&address.address_type{
    //                 if address_type=="work"{
    //                     if let Some(postal_code) = &address.postal_code {
    //                         return Some(postal_code.to_string());
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     None
    // }
    //
    // fn get_zip_home(&self) -> Option<String> {
    //     if let Some(addresses) = &self.addresses{
    //         for address in addresses {
    //             if let Some(address_type)=&address.address_type{
    //                 if address_type=="home"{
    //                     if let Some(postal_code) = &address.postal_code {
    //                         return Some(postal_code.to_string());
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     None
    // }
    //
    // fn get_notes(&self) -> Option<String> {
    //     if let Some(biographies) = &self.biographies{
    //         for biography in biographies {
    //             if let Some(notes)=&biography.value{
    //                 return Some(notes.to_string());
    //             }
    //         }
    //     }
    //     None
    // }
    //
    // fn get_contact_group(&self) -> Option<String> {
    //     None
    // }
    //
    //
    //
    // fn set_full_name(&mut self, newval: &str) {
    //     if self.names.is_none(){
    //         let mut name_vec = Vec::new();
    //         name_vec.push(Name{
    //             display_name:None,
    //             family_name:None,
    //             display_name_last_first:None,
    //             given_name:None,
    //             metadata:None,
    //             source_primary:None,
    //             unstructured_name:None
    //         });
    //         self.names = Some(name_vec);
    //     }
    //     let mut orig_vec = self.names.as_ref().unwrap().clone();
    //     orig_vec[0].display_name = Some(String::from(newval));
    //     self.names = Some(orig_vec);
    //
    //     // self.names.unwrap()[0].display_name = Some(String::from(newval));
    // }
    //
    // fn set_first_name(mut self, newval: &str) {
    //     if self.names.is_none(){
    //         let mut name_vec = Vec::new();
    //         name_vec.push(Name{
    //             display_name:None,
    //             family_name:None,
    //             display_name_last_first:None,
    //             given_name:None,
    //             metadata:None,
    //             source_primary:None,
    //             unstructured_name:None
    //         });
    //         self.names = Some(name_vec);
    //     }
    //     self.names.unwrap()[0].given_name = Some(String::from(newval));
    // }
    //
    // fn set_last_name(mut self, newval: &str) {
    //     if self.names.is_none(){
    //         let mut name_vec = Vec::new();
    //         name_vec.push(Name{
    //             display_name:None,
    //             family_name:None,
    //             display_name_last_first:None,
    //             given_name:None,
    //             metadata:None,
    //             source_primary:None,
    //             unstructured_name:None
    //         });
    //         self.names = Some(name_vec);
    //     }
    //     self.names.unwrap()[0].family_name = Some(String::from(newval));
    // }
    //
    // fn set_email_work(mut self, newval: &str) {
    //     let mut new_email:contact::EmailAddress = EmailAddress{
    //         formatted_type:None,
    //         metadata:None,
    //         email_address_type:Some(String::from("work")),
    //         value:Some(String::from(newval))
    //     };
    //     if self.email_addresses.is_none(){
    //         let mut vec:Vec<contact::EmailAddress> = Vec::new();
    //         vec.push(new_email);
    //         self.email_addresses = Some(vec);
    //     }else{
    //         self.email_addresses.unwrap().push(new_email);
    //     }
    // }
    //
    // fn set_email_home(mut self, newval: &str) {
    //     let mut new_email:contact::EmailAddress = EmailAddress{
    //         formatted_type:None,
    //         metadata:None,
    //         email_address_type:Some(String::from("home")),
    //         value:Some(String::from(newval))
    //     };
    //     if self.email_addresses.is_none(){
    //         let mut vec:Vec<contact::EmailAddress> = Vec::new();
    //         vec.push(new_email);
    //         self.email_addresses = Some(vec);
    //     }else{
    //         self.email_addresses.unwrap().push(new_email);
    //     }
    // }
    //
    // fn set_company(mut self, newval: &str) {
    //     if self.organizations.is_none(){
    //         let mut org_vec = Vec::new();
    //         org_vec.push(Organization{
    //             formatted_type:None,
    //             metadata:None,
    //             name:None,
    //             title:None,
    //             organization_type:Some(String::from("work"))
    //         });
    //         self.organizations = Some(org_vec);
    //     }
    //     self.organizations.unwrap()[0].name = Some(String::from(newval));
    // }
    //
    // fn set_job_title(mut self, newval: &str) {
    //     if self.organizations.is_none(){
    //         let mut org_vec = Vec::new();
    //         org_vec.push(Organization{
    //             formatted_type:None,
    //             metadata:None,
    //             name:None,
    //             title:None,
    //             organization_type:Some(String::from("work"))
    //         });
    //         self.organizations = Some(org_vec);
    //     }
    //     self.organizations.unwrap()[0].title = Some(String::from(newval));
    // }
    //
    // fn set_phone_number_work1(mut self, newval: &str) {
    //     let mut new_phone:contact::PhoneNumber = PhoneNumber{
    //         formatted_type:None,
    //         metadata:None,
    //         phone_number_type:Some("work".into()),
    //         value:Some(String::from(newval)),
    //     };
    //     if self.phone_numbers.is_none(){
    //         let mut vec:Vec<contact::PhoneNumber> = Vec::new();
    //         vec.push(new_phone);
    //         self.phone_numbers = Some(vec);
    //     }else{
    //         self.phone_numbers.unwrap().push(new_phone);
    //     }
    // }
    //
    // fn set_phone_number_work2(mut self, newval: &str) {
    //     todo!()
    // }
    //
    // fn set_phone_number_home1(mut self, newval: &str) {
    //     todo!()
    // }
    //
    // fn set_phone_number_home2(mut self, newval: &str) {
    //     todo!()
    // }
    //
    // fn set_phone_number_mobile1(mut self, newval: &str) {
    //     todo!()
    // }
    //
    // fn set_phone_number_mobile2(mut self, newval: &str) {
    //     todo!()
    // }
    //
    // fn set_street_work(mut self, newval: &str) {
    //     todo!()
    // }
    //
    // fn set_street_home(mut self, newval: &str) {
    //     todo!()
    // }
    //
    // fn set_city_work(mut self, newval: &str) {
    //     todo!()
    // }
    //
    // fn set_city_home(mut self, newval: &str) {
    //     todo!()
    // }
    //
    // fn set_state_work(mut self, newval: &str) {
    //     todo!()
    // }
    //
    // fn set_state_home(mut self, newval: &str) {
    //     todo!()
    // }
    //
    // fn set_zip_work(mut self, newval: &str) {
    //     todo!()
    // }
    //
    // fn set_zip_home(mut self, newval: &str) {
    //     todo!()
    // }
    //
    // fn set_notes(mut self, newval: &str) {
    //     todo!()
    // }
    //
    // fn set_contact_group(mut self, newval: &str) {
    //     todo!()
    // }


}

