package com.caseyandgary.ldapeditor.ldap;

import java.util.List;
import javax.naming.*;
import java.util.Hashtable;
import javax.naming.directory.*;

import com.caseyandgary.ldapeditor.web.*;

public class Contact
	extends DirContextAdapter 
        implements Comparable{
	
	//String type;
	private Attributes myAttrs;
	private String CN = null;
	
	public Contact(Attributes attr) {
                System.out.println("Creating a new LDAP contact object");
                try{
                  NamingEnumeration<String> ne = attr.getIDs();
                  while(ne.hasMore()){
                    String id = (String)ne.next();
                    //System.out.println("Attribute "+id+" is defined.");
                  }
                }catch(NamingException ex){
                  System.err.println("Could not enumerate the attributes\n");
                }
		this.myAttrs = attr;
	}
	
	public Contact() {
		//type = name;
		myAttrs = new BasicAttributes(true);  // Case ignore
		Attribute oc = new BasicAttribute("objectclass");
		oc.add("top");
		oc.add("person");
		oc.add("organizationalPerson");
		oc.add("inetOrgPerson");			
		oc.add("mozillaAbPersonAlpha");			
		myAttrs.put(oc);		
	}
	
	private void setAttr(String mapping, String rawvalue) {
		if ((rawvalue!=null) && (rawvalue.length()>0)) {
                  String value = rawvalue.trim();
                  Attribute newAttr = myAttrs.get(mapping);
                  if (newAttr==null){
                    newAttr = new BasicAttribute(mapping);
                  }
                  newAttr.add(value);
                  myAttrs.put(newAttr);
		}
	}
	
	/** Setters */
	
	public void setCn(String cn) {
		setAttr(Mapping.NICKNAME, cn);
	}

	public void setFirstName(String givenName) {
		setAttr(Mapping.FIRSTNAME, givenName);
	}	
	
	public void setLastName(String sn) {
		setAttr(Mapping.LASTNAME, sn);
	}

	public void setCompany(String company) {
		setAttr(Mapping.COMPANY, company);
	}
	
	public void setJobTitle(String title) {
		setAttr(Mapping.POSITION,title );
	}

	public void setNotes(String notes) {
		setAttr(Mapping.NOTES, notes);
	}

	public void setEmail(String email) {
		setAttr(Mapping.EMAIL, email);
	}

	public void setEmailSecondary(String emailSecondary) {
		setAttr(Mapping.EMAIL_SECONDARY, emailSecondary);
	}

	public void setBusinessAddress(String address) {
		setAttr(Mapping.BUSINESS_ADDR, address);
	}
	
	public void setBusinessCity(String l) {
		setAttr(Mapping.BUSINESS_CITY, l);
	}
	
	public void setBusinessState(String st) {
		setAttr(Mapping.BUSINESS_STATE, st);
	}
	
	public void setBusinessZip(String zip) {
		setAttr(Mapping.BUSINESS_ZIP, zip );
	}

	public void setHomeAddress(String address) {
		setAttr(Mapping.HOME_ADDR, address);
	}
	
	public void setHomeCity(String l) {
		setAttr(Mapping.HOME_CITY, l);
	}
	
	public void setHomeState(String st) {
		setAttr(Mapping.HOME_STATE, st);
	}
	
	public void setHomeZip(String zip) {
		setAttr(Mapping.HOME_ZIP, zip );
	}

	public void setDepartment(String dept) {
          String[] deptArr = new String[1];
          deptArr[0] = dept;
          setDepartment(deptArr);
	}
	
	public void setDepartment(String[] dept) {
          for(int i=0;i<dept.length;++i){
		setAttr(Mapping.DEPT, dept[i]);
          }
	}
	
	public void setBusinessPhone(String[] phones) {
		if (phones!=null){
			for(int i=0;i<phones.length;++i){
				setAttr(Mapping.BUSINESS_PHONE, phones[i]);
			}
		}
	}

	public void setHomePhone(String[] phones) {
		if (phones!=null){
			for(int i=0;i<phones.length;++i){
				setAttr(Mapping.HOME_PHONE, phones[i]);
			}
		}
	}
	
	public void setMobile(String[] phones) {
		if (phones!=null){
			for(int i=0;i<phones.length;++i){
				setAttr(Mapping.MOBILE_PHONE, phones[i]);
			}
		}
	}

	public void setPager(String[] phones) {
		if (phones!=null){
			for(int i=0;i<phones.length;++i){
				setAttr(Mapping.PAGER, phones[i]);
			}
		}
	}

	public void setBusinessFax(String[] phones) {
		if (phones!=null){
			for(int i=0;i<phones.length;++i){
				setAttr(Mapping.BUSINESS_FAX, phones[i]);
			}
		}
	}
	
	private String fromAttrString(String mapping) {
		try {
			Attribute attr = myAttrs.get(mapping);
			if (attr!=null) {
               		  return attr.get().toString();
			}
		}
		catch (NamingException ex) {
			
		}
		return "";
		//return "(not entered)";
	}	
	
	/** Getters	*/

        public int compareTo(Object obj){
          Contact testContact = (Contact)obj;
          String testCN = testContact.getCn();
          String thisCN = this.getCn();
          return thisCN.compareTo(testCN);
        }

        public Attribute getCNAttr() {
                return myAttrs.get(Mapping.NICKNAME);
        }

	
	public String getCn() {
		return fromAttrString(Mapping.NICKNAME);
	}	

	public  String  getFirstName() {
		return fromAttrString(Mapping.FIRSTNAME);
	}	
	
	public  String  getLastName() {
		return fromAttrString(Mapping.LASTNAME);
	}

	public  String  getCompany() {
		return fromAttrString(Mapping.COMPANY);
	}
	
	public  String  getJobTitle() {
		return fromAttrString(Mapping.POSITION);
	}

	public  String  getNotes() {
		return fromAttrString(Mapping.NOTES);
	}
	
	public  String  getEmail() {
		return fromAttrString(Mapping.EMAIL);
	}

	public  String  getEmailSecondary() {
		return fromAttrString(Mapping.EMAIL_SECONDARY);
	}

	public  List<String>  getStrList(String mapping) 
        throws LDAPException{
          List<String> strList = new java.util.ArrayList();
          try{
            Attribute attr = myAttrs.get(mapping);
            if (attr!=null){
              for(int i=0;i<attr.size();++i){
                strList.add((String)attr.get(i));
              }
            }
          }catch(NamingException ex){
            throw new LDAPException("Error in getting list of attributes for "+mapping+": "+ex.getMessage());
          }
          return strList;
	}

	public  List<String>  getBusinessPhone() 
	throws LDAPException{
		return getStrList(Mapping.BUSINESS_PHONE);
	}

	public  List<String>  getHomePhone() 
	throws LDAPException{
		return getStrList(Mapping.HOME_PHONE);
	}

	public  List<String>  getMobile() 
	throws LDAPException{
		return getStrList(Mapping.MOBILE_PHONE);
	}

	public  String getMobileByIndex(int index) 
	throws LDAPException{
		List<String> strList = getStrList(Mapping.MOBILE_PHONE);
		if (strList.size()>index){
			return strList.get(index);
		}else{
			return ContactBean.DEFAULT_VAL;
		}
	}

	public  List<String>  getPager() 
	throws LDAPException{
		return getStrList(Mapping.PAGER);
	}
	
	public  List<String>  getBusinessFax() 
	throws LDAPException{
		return getStrList(Mapping.BUSINESS_FAX);
	}

	public  String  getBusinessAddress() {
		return fromAttrString(Mapping.BUSINESS_ADDR);
	}
	
	public  String  getBusinessCity() {
		return fromAttrString(Mapping.BUSINESS_CITY);
	}
	
	public  String  getBusinessState() {
		return fromAttrString(Mapping.BUSINESS_STATE);
	}
	
	public  String  getBusinessZip() {
		return fromAttrString(Mapping.BUSINESS_ZIP);
	}
	
	public  String  getHomeAddress() {
		return fromAttrString(Mapping.HOME_ADDR);
	}
	
	public  String  getHomeCity() {
		return fromAttrString(Mapping.HOME_CITY);
	}
	
	public  String  getHomeState() {
		return fromAttrString(Mapping.HOME_STATE);
	}
	
	public  String  getHomeZip() {
		return fromAttrString(Mapping.HOME_ZIP);
	}

        public  Attribute  getDepartmentAttr() {
                return myAttrs.get(Mapping.DEPT);
        }


	public  String  getDepartment() {
		return fromAttrString(Mapping.DEPT);
	}

	public  String  getTimestamp() {
                String ret = fromAttrString(Mapping.TIMESTAMP);
                System.out.println("For "+Mapping.TIMESTAMP+": "+ret);
                String ret2 = fromAttrString("ou");
                System.out.println("For ou: "+ret2);
                return ret;
	}
	

	public  List<String>  getDepartmentList() 
        throws LDAPException{
		return getStrList(Mapping.DEPT);
	}
	
	public Attributes getAttributes(String name, String[] names)
	throws NamingException {
		return (Attributes)myAttrs.clone();
	}	
	
	public Attributes getAttributes(Name name, String[] ids) 
	throws NamingException {
		return getAttributes(name.toString(), ids);
	}
	
	public Attributes getAttributes(String name)
	throws NamingException {
		if (! name.equals("")) {
			throw new NameNotFoundException();
		}
		return (Attributes)myAttrs.clone();
	}
		
	public Attributes getAttributes(Name name) throws NamingException {
		return getAttributes(name.toString());
	}
	
}

