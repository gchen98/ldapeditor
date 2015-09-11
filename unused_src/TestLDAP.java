package com.caseyandgary.ldapeditor;
/*
TestLDAP.java -- basic JNDI application used for
        testing adding/retrieving objects from the server.
*/


import javax.naming.Context;
import javax.naming.InitialContext;
import javax.naming.NamingException;
import javax.naming.NameAlreadyBoundException;
import javax.naming.directory.*;
import java.util.*;
import java.io.*;
import com.caseyandgary.ldapeditor.ldap.*;

public class TestLDAP {
	
	private LDAPConnector ldap = null;
	
	public TestLDAP(String propFileLocation)
		throws LDAPException,IOException{
		Properties prop = new Properties();
		prop.load(new FileInputStream(propFileLocation));
		ldap = new LDAPConnector(prop);
	}
	
	public void addContact(String cn)
		throws LDAPException{
		Contact contact = new Contact();
		contact.setCn(cn);
		contact.setEmail("garyc@pioneedsfsrind.com");
		contact.setBusinessPhone("310-209-1343");
		contact.setBusinessFax("123-234-1232");
		contact.setBusinessAddress("1234 Elm Street");
		contact.setMobile("713-412-5323");
		//contact.setHomeAddr("2421 moreno$LA$CA$90039");
		contact.setBusinessCity("West LA");
		contact.setBusinessState("CA");
		contact.setCompany("Comerce One");
		contact.setDepartment("Engineer");
		
		contact.setFirstName("Gary");
		contact.setHomePhone("323-668-1545");
		contact.setJobTitle("Developer");
		contact.setLastName("Chen");
		//contact.setURL("http://www.ucla.edu");
		contact.setBusinessZip("90024");
		ldap.accessContact(contact,LDAPConnector.ADD);		
		
	}
	
	public void modifyContact(String cn)
		throws LDAPException{
		
		Contact contact = new Contact();
		contact.setCn(cn);
		contact.setEmail("garyc@pioneer.com");
		contact.setBusinessPhone("310-209-1343");
		contact.setBusinessFax("123-234-1232");
		contact.setBusinessAddress("1234 Elm Street");
		contact.setMobile("713-412-5323");
		//contact.setHomeAddr("2421 moreno$LA$CA$90039");
		contact.setBusinessCity("Westwood");
		contact.setBusinessState("CA");
		//contact.setBusinessCountry("US");
		contact.setCompany("UCLA");
		contact.setDepartment("Cluster");
		
		contact.setFirstName("Kenny Cho");
		contact.setHomePhone("323-668-1545");
		contact.setJobTitle("QA Engineer");
		contact.setLastName("Chen");
		//contact.setHomePage("http://www.ucla.edu");
		contact.setBusinessZip("90024");
		ldap.accessContact(contact,LDAPConnector.EDIT);		
	}
	
	public void removeContact(String cn) 
		throws LDAPException{
		Contact contact = new Contact();
		contact.setCn(cn);
		ldap.accessContact(contact,LDAPConnector.DELETE);		
	}
	
	public void findContact(String cn)
		throws LDAPException{
		Contact contact = new Contact();
		contact.setCn("Frank");
		//contact.setDepartment("contractor");
		//contact.setEmail("garyc@pioneedsfsrind.com");
		//contact.setBusinessPhone("310-209-1343");
		//contact.setBusinessFax("123-234-1232");		
		//contact.setFirstName("Gary");
		//contact.setHomePhone("323-668-1545");
		//contact.setJobTitle("Student");
		//contact.setLastName("Chen");
		//contact.setHomePage("http://www.ucla.edu");
		//contact.setBusinessZip("90024");
		long begin = System.currentTimeMillis();
		ldap.accessContact(contact,LDAPConnector.GET_CN_LIST);
		long end = System.currentTimeMillis();
        System.out.println("Time to search: "+(double)(end-begin)/1000);
		List contactList = ldap.getSearchResults();
		Iterator it = contactList.iterator();
		while (it.hasNext()) {
			Contact nextContact = (Contact)it.next();
			System.out.println("The cn is: "+nextContact.getCn());
			System.out.println("The email is: "+nextContact.getEmail());
		}
		
	}
	
	
	
	public static void main( String[] args ) {
		
		try {			

  String dept = "alice";
  java.util.Map<String,String> deptMap= new java.util.HashMap();
  deptMap.put("family","Chen family contacts");
  deptMap.put("alice","Alice's personal contacts");
  deptMap.put("caseych","Casey's personal contacts");
  deptMap.put("garyc","Gary's personal contacts");
  deptMap.put("bohu","Casey's and Gary's personal contacts");
  java.util.Iterator<String> iterator = deptMap.keySet().iterator();
  while(iterator.hasNext()){
    String key = (String)iterator.next();
    String val = (String)deptMap.get(key);
    System.out.print("<option value=\"key\"");
    if (key.equals(dept)){
      System.out.print(" selected=\"true\"");
    }
    System.out.print(">"+val+"</option>");
  }

			String newCN = "Frank";
			//String orgUnit = "Friend";
			//TestLDAP test = new TestLDAP("z:\\docs\\development\\ldap\\webapps\\WEB-INF\\ldap_prop.txt");
			TestLDAP test = new TestLDAP("/home/garyc/docs/development/ldap/webapps/WEB-INF/ldap_prop.txt");
			
			//test.init();
			//test.removeContact(newCN);
			
			//test.addContact(newCN);
			
			//System.out.println("Modifying "+newCN);
			//test.modifyContact(newCN);
			for (int i=0;i<30;i++) {
				System.out.println("Iteration "+i+" Searching "+newCN);
				test.findContact(newCN);					
			}
			System.out.println("Done");
			System.exit(0);
			
		} catch ( Exception e ) {
			e.printStackTrace();
			System.err.println( e );
		}
	}
}
