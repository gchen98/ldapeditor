package com.caseyandgary.ldapeditor.ldap;

import java.util.List;
import java.util.Hashtable;
import java.util.ArrayList;
import javax.naming.Context;
import javax.naming.Name;
import javax.naming.NameParser;
import javax.naming.NamingException;
import javax.naming.NameAlreadyBoundException;
import javax.naming.CommunicationException;
import javax.naming.NamingEnumeration;
import javax.naming.directory.Attribute;
import javax.naming.directory.Attributes;
import javax.naming.directory.BasicAttributes;
import javax.naming.directory.BasicAttribute;
import javax.naming.directory.SearchResult;
import javax.naming.directory.ModificationItem;
import javax.naming.directory.DirContext;
import javax.naming.directory.InitialDirContext;


import javax.naming.directory.SearchControls;
import javax.naming.ldap.Control;
import javax.naming.ldap.*;
import javax.naming.ldap.InitialLdapContext;

import java.util.Properties;

public class LDAPConnector {
	
	public static final int EDIT = 0;
	public static final int ADD = 1;
	public static final int DELETE = 2;
	public static final int SEARCH = 3;
	public static final int GET_CN_LIST = 4;
	
	public static final String PEOPLE_CONTEXT="peopleContext";
	public static final String LDAP_SERVER_NAME="ldapServerName";
	public static final String ROOT_CONTEXT="rootContext";
	public static final String ROOT_DN="rootdn";
	public static final String ROOT_PASS="rootpass";
	public static final String NUM_OF_TRIES="numtries";
	public static final String TRUST_PATH="trustpath";
	public static final String SECURE="secure";	
	
	private String ldapServerName;
        private boolean enableSort;
	private String rootContext;
	private String peopleContext;
	private String rootdn;
	private String rootpass;
	private String numTries;
	private String trustStorePath;
	private String isSecure;
	private List searchResult;
	
        // NEW WAY Modified 8-2-2013
	private LdapContext ctx = null;
	private Hashtable<String,Object> env = null;
        // OLD WAY 
	//private InitialDirContext ctx = null;
//	private StartTlsResponse tls = null;
	//private Properties env = null;
	
	public LDAPConnector(Properties initProps)
		throws LDAPException{
//		java.security.Security.addProvider(new com.sun.net.ssl.internal.ssl.Provider());
		setProperties(initProps);
                enableSort = false;
	}	
        public void setSort(boolean val){
                enableSort = val;
        }
	
	public void setProperties(Properties initProps)
	throws LDAPException{
		ldapServerName = initProps.getProperty(LDAP_SERVER_NAME);
		rootContext= initProps.getProperty(ROOT_CONTEXT);
		peopleContext= initProps.getProperty(PEOPLE_CONTEXT);
		rootdn= initProps.getProperty(ROOT_DN) + "," + rootContext;
		rootpass = initProps.getProperty(ROOT_PASS);
		numTries = initProps.getProperty(NUM_OF_TRIES);
		trustStorePath = initProps.getProperty(TRUST_PATH);
		isSecure = initProps.getProperty(SECURE);
		
		//System.getProperties().list(System.out);		
		// NEW WAY
		env = new Hashtable<String,Object>(11);
		// OLD WAY
		//env = new Properties();		
		env.put( Context.INITIAL_CONTEXT_FACTORY,"com.sun.jndi.ldap.LdapCtxFactory" );		
		String prefix = "ldap://";
		if (isSecure.equals("true")) {
			System.setProperty("javax.net.ssl.trustStore",trustStorePath);
			prefix = "ldaps://";
			env.put(Context.SECURITY_PROTOCOL, "ssl");
			env.put("java.naming.ldap.factory.socket", "ldap.SSLCustomSocketFactory");
		}
		else {
			env.put("java.naming.ldap.factory.socket", "ldap.CustomSocketFactory");
		}
		env.put( Context.PROVIDER_URL, prefix + ldapServerName + "/"  );
		env.put( Context.SECURITY_PRINCIPAL, rootdn );
		env.put( Context.SECURITY_CREDENTIALS, rootpass );
		//env.put("com.sun.jndi.ldap.trace.ber", System.err);		
		//initLDAP();
	}
	
	private void initLDAP()
	throws LDAPException{
		try {
			//System.out.println("Creating LDAP Context");
			ctx = new InitialLdapContext(env,null);
			//ctx = new InitialDirContext(env);
			//System.out.println("LDAP Context created");
			
		}
		catch (CommunicationException ex) {
			ex.printStackTrace();
			throw new LDAPException("The server seems to be down.  The message was: "+ex.getMessage());
		}
		catch (Exception ex) {
			throw new LDAPException("An exception of type: "+ex.getClass().getName()+
				" was found while trying to access the server with message: "+ex.getMessage());
		}
	}	

	
	/*public void initSubContext()
	throws NamingException{
		Attribute rootAttr = new BasicAttribute("objectclass");
		rootAttr.add("top");
		rootAttr.add("organization");	
		Attributes attrs = new BasicAttributes();				
		attrs.put(rootAttr);		
		DirContext root = ctx.createSubcontext(rootContext,attrs);		
		Attribute peopleAttr = new BasicAttribute("objectclass");
		peopleAttr.add("top");
		peopleAttr.add("organizationalUnit");	
		attrs = new BasicAttributes();		
		attrs.put(peopleAttr);		
		DirContext people = root.createSubcontext("ou=People",attrs);		
	}*/

        private void accessContact(Contact contact, int action)
             throws LDAPException {
             accessContact(contact,action,false);
        }
	
	private boolean accessContact(Contact contact, int action, int numTries,          boolean exactMatch)
		throws LDAPException{
		try {
			switch(action) {
			case ADD:
				addContact(contact);
				break;
			case EDIT:
				editContact(contact);
				break;
			case SEARCH:
				findContact(contact,false,exactMatch);
				break;
			case GET_CN_LIST:
				findContact(contact,true,exactMatch);
				break;
			case DELETE:
				deleteContact(contact);				
			}
			// Close the TLS connection (revert back to the underlying LDAP association)
			// tls.close();
			//System.out.println("TLS closed");

			ctx.close();
			if (isSecure.equals("true")){
				java.net.Socket sock = SSLCustomSocketFactory.currSocket;
				((javax.net.ssl.SSLSocket)sock).getSession().invalidate();
				//sock.close();
				//ctx=null;
			}
			else {
				java.net.Socket sock = CustomSocketFactory.currSocket;
				sock.close();
			}
			//System.out.println("LDAP Context and socket closed");
		}		
		catch (CommunicationException ex) {
			return false;
		}
		catch (javax.naming.InvalidNameException ex) {
			throw new LDAPException("The unique name has invalid characters in it. Avoid characters such as commas and double quotes.");
		}

		catch (NameAlreadyBoundException ex) {
			throw new LDAPException("Sorry but the unique name you entered is already in the directory.");
		}
		catch (Exception ex) {
			ex.printStackTrace();
			throw new LDAPException("An exception of type: "+ex.getClass().getName()+
				" was found while accessing contact "+contact.getCn()+" with message: "+ex.getMessage());
		}		
		return true;
	}
	
	public List getSearchResults() {
		return this.searchResult;
	}

	/**  These accessor methods are the only entry points 
	 *    we want to support retries.
	 */

	
	
	public void accessContact(Contact contact,int action, boolean exactMatch)
		throws LDAPException{
		boolean success=false;
		int retryCount = new Integer(numTries).intValue();
		do {
			try {					
					initLDAP();
					//System.out.println("Retrieving data from LDAP");
					success=accessContact(contact,action,--retryCount, exactMatch);
					//System.out.println("Completed retrieving data from LDAP");
					if (!success) {
						Thread.sleep(1000);
					}
				}
				catch (InterruptedException ex) {
					System.err.println("Interrupted Exception occured in access contact");
				}
		} while ((!success) && (retryCount>0));		
		if (!success) {
			throw new LDAPException("We retried connecting to the directory server but failed after "+numTries+" times.");
		}
	}

        private String newCn(String oldCN)
        throws Exception{
          byte[] bytes = oldCN.getBytes();
          String newStr = new String(bytes,"UTF-8");
          //System.err.println("DEBUG: OLDSTR: "+oldCN);
          //System.err.println("DEBUG: NEWSTR: "+newStr);
          return oldCN;
        }

        private String recodeCn(String cn){
          String c = cn.replaceAll(",","\\\\,");
          return c;
        }
	
	private void addContact(Contact contact)
		throws Exception {
                NameParser parser = ctx.getNameParser("");
                String dn = "cn="+recodeCn(contact.getCn().trim())+","+peopleContext+","+rootContext;
                Name cn = parser.parse(dn);
		ctx.bind(cn,contact);
	}
	
	private void deleteContact(Contact contact)
		throws Exception {
                NameParser parser = ctx.getNameParser("");
                String dn = "cn="+recodeCn(contact.getCn().trim())+","+peopleContext+","+rootContext;
                Name cn = parser.parse(dn);
		ctx.unbind(cn);
                //NameParser ldapParser = ctx.getNameParser("");
                //Name compound = ldapParser.parse(dn);
		//ctx.unbind(compound);
	}
	
	
	
	private void findContact(Contact contact, boolean cnOnly, boolean exactMatch)
		throws Exception {
		
		 SearchControls ctls = new SearchControls();
			if (cnOnly) {
			     String[] returnAttr = {"cn"};
	                     ctls.setReturningAttributes(returnAttr);
			}else{
			     String[] returnAttr = {"cn","givenName","sn","o","title","mail","mozillaSecondEmail","telephonenumber","homePhone","mobile","pager","facsimiletelephonenumber","ou","street","l","st","postalCode","mozillaHomeStreet","mozillaHomeLocalityName","mozillaHomeState","mozillaHomePostalCode","description","modifyTimestamp"};
                             if (returnAttr!=null){
                               for(int i=0;i<returnAttr.length;++i){
                                 //System.out.println("return attr is "+returnAttr[i]);
                               }
                             }
	                     ctls.setReturningAttributes(returnAttr);
                        }
                        String wildcard = "*";
                        if (exactMatch) {
                           wildcard=""; 
                        }
                        String cnFilter = "(cn="+wildcard+contact.getCn()+
                                           wildcard+")";
                        String ouFilter = "(ou="+wildcard+
                                      contact.getDepartment()+wildcard+")";
			String filter = null;
			if ((contact.getDepartmentAttr()!=null) && (contact.getCNAttr()!=null)) {
				//System.out.println("filtering on cn and org unit");
				filter = "(&"+cnFilter+ouFilter+")";
			}
			else if (contact.getDepartmentAttr()==null) {
				//System.out.println("Not filtering on org unit");
				filter = cnFilter;
			}
			else if (contact.getCNAttr()==null) {
				//System.out.println("Not filtering on cn");
				filter = ouFilter;
			}         
     
        // Search for objects using filter
       String sortKey = "cn";
       //ctx.setRequestControls(new Control[] {
       //new SortControl(sortKey,Control.NONCRITICAL) });	
       NamingEnumeration enumer = ctx.search(peopleContext+","+rootContext,
       filter, ctls);
	    
       List contacts=new ArrayList();
		
       if (enumer!=null) {
		while (enumer.hasMore()) {
			Object obj = enumer.next();
			SearchResult result = (SearchResult)obj;
			Contact newContact = new Contact(result.getAttributes());
			contacts.add(newContact);
		}
       }
       if (enableSort){
         java.util.Collections.sort(contacts);
         //System.err.println("Done sorting");
       }
       this.searchResult = contacts;
}
	
	private void editContact(Contact contact)
		throws Exception {
                deleteContact(contact);
                addContact(contact);
	}
}
