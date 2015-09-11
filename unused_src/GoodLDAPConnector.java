package ldap;

import java.util.List;
import java.util.ArrayList;
import javax.naming.Context;
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
	
	
	private String ldapServerName;
	private String rootContext;
	private String peopleContext;
	private String rootdn;
	private String rootpass;
	private String numTries;
	private List searchResult;
	
	private DirContext ctx = null;
	private Properties env = null;
	
	public LDAPConnector(Properties initProps)
		throws LDAPException{		
		setProperties(initProps);
	}	
	
	public void setProperties(Properties initProps)
	throws LDAPException{
		ldapServerName = initProps.getProperty(LDAP_SERVER_NAME);
		rootContext= initProps.getProperty(ROOT_CONTEXT);
		peopleContext= initProps.getProperty(PEOPLE_CONTEXT);
		rootdn= initProps.getProperty(ROOT_DN) + "," + rootContext;
		rootpass = initProps.getProperty(ROOT_PASS);
		numTries = initProps.getProperty(NUM_OF_TRIES);
		
		env = new Properties();		
		env.put( Context.INITIAL_CONTEXT_FACTORY,"com.sun.jndi.ldap.LdapCtxFactory" );
		env.put( Context.PROVIDER_URL, "ldaps://" + ldapServerName + "/"  );
		env.put( Context.SECURITY_PRINCIPAL, rootdn );
		env.put( Context.SECURITY_CREDENTIALS, rootpass );
		//initLDAP();
	}
	
	private void initLDAP()
	throws LDAPException{
		try {
			ctx = new InitialDirContext(env);
		}
		catch (CommunicationException ex) {
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
	
	private boolean accessContact(Contact contact, int action, int numTries)
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
				findContact(contact,false);
				break;
			case GET_CN_LIST:
				findContact(contact,true);
				break;
			case DELETE:
				deleteContact(contact);				
			}
		}
		catch (CommunicationException ex) {
			return false;
		}
		catch (NameAlreadyBoundException ex) {
			throw new LDAPException("Sorry but the unique name you entered is already in the directory.");
		}
		catch (Exception ex) {
			ex.printStackTrace();
			throw new LDAPException("An exception of type: "+ex.getClass().getName()+
				" was found while accessing contact with message: "+ex.getMessage());
		}		
		return true;
	}
	
	public List getSearchResults() {
		return this.searchResult;
	}
	
	/**  This accessor method is the only entry point as 
	 *    we want to support retries.
	 */
	
	public void accessContact(Contact contact,int action)
		throws LDAPException{
		boolean success=false;
		int retryCount = new Integer(numTries).intValue();
		do {
			try {					
					initLDAP();
					//System.out.println("Retrieving data from LDAP");
					success=accessContact(contact,action,--retryCount);
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
	
	private void addContact(Contact contact)
		throws Exception {
		ctx.bind("cn="+contact.getCn()+","+peopleContext+","+rootContext,contact);
	}
	
	private void deleteContact(Contact contact)
		throws Exception {
		ctx.unbind("cn="+contact.getCn()+","+peopleContext+","+rootContext);
	}
	
	
	
	private void findContact(Contact contact, boolean cnOnly)
		throws Exception {
		
		 SearchControls ctls = new SearchControls();
			if (cnOnly) {
			     String[] returnAttr = {"cn"};
				 ctls.setReturningAttributes(returnAttr);
			}
			String filter = null;
			if ((contact.getDepartmentAttr()!=null) && (contact.getCNAttr()!=null)) {
				System.out.println("filtering on cn and org unit");
				filter = "(&(cn=*"+contact.getCn()+"*)(ou=*"+contact.getDepartment()+"*))";
			}
			else if (contact.getDepartmentAttr()==null) {
				System.out.println("Not filtering on org unit");
				filter = "(cn=*"+contact.getCn()+"*)";
			}
			else if (contact.getCNAttr()==null) {
				System.out.println("Not filtering on cn");
				filter = "(ou=*"+contact.getDepartment()+"*)";
			}         
     
        // Search for objects using filter
		
        NamingEnumeration enum = ctx.search(peopleContext+","+rootContext,
       filter, ctls);
	    
     	List contacts=new ArrayList();
		
		if (enum!=null) {
			while (enum.hasMore()) {
				Object obj = enum.next();
				SearchResult result = (SearchResult)obj;
				Contact newContact = new Contact(result.getAttributes());
				contacts.add(newContact);
			}
		}
		
		this.searchResult = contacts;
	}
	
	private void editContact(Contact contact)
		throws Exception {
		
		// Specify the changes to make		
		ModificationItem[] mods = new ModificationItem[15];		
		mods[0] = new ModificationItem(DirContext.REPLACE_ATTRIBUTE,		
			contact.getEmailAttr());		
		mods[1] = new ModificationItem(DirContext.REPLACE_ATTRIBUTE,			
			contact.getJobTitleAttr());		
		mods[2] = new ModificationItem(DirContext.REPLACE_ATTRIBUTE,
			contact.getDepartmentAttr());		
		mods[3] = new ModificationItem(DirContext.REPLACE_ATTRIBUTE,
			contact.getCompanyAttr());		
		mods[4] = new ModificationItem(DirContext.REPLACE_ATTRIBUTE,
			contact.getMobileAttr());		
		mods[5] = new ModificationItem(DirContext.REPLACE_ATTRIBUTE,
			contact.getFirstNameAttr());
		mods[6] = new ModificationItem(DirContext.REPLACE_ATTRIBUTE,
			contact.getLastNameAttr());		
		mods[7] = new ModificationItem(DirContext.REPLACE_ATTRIBUTE,
			contact.getBusinessAddressAttr());		
		mods[8] = new ModificationItem(DirContext.REPLACE_ATTRIBUTE,
			contact.getBusinessCityAttr());		
		mods[9] = new ModificationItem(DirContext.REPLACE_ATTRIBUTE,
			contact.getBusinessStateAttr());		
		mods[10] = new ModificationItem(DirContext.REPLACE_ATTRIBUTE,
			contact.getHomePhoneAttr());		
		mods[11] = new ModificationItem(DirContext.REPLACE_ATTRIBUTE,
			contact.getHomeAddrAttr());		
		mods[12] = new ModificationItem(DirContext.REPLACE_ATTRIBUTE,
			contact.getBusinessPhoneAttr());		
		mods[13] = new ModificationItem(DirContext.REPLACE_ATTRIBUTE,
			contact.getBusinessFaxAttr());		
		mods[14] = new ModificationItem(DirContext.REPLACE_ATTRIBUTE,
			contact.getBusinessZipAttr());		
		//mods[15] = new ModificationItem(DirContext.REPLACE_ATTRIBUTE,
		//	new BasicAttribute(Mapping.HOMEPAGE, contact.getHomePageAttr());
		
		ctx.modifyAttributes("cn="+contact.getCn()+","+peopleContext+","+rootContext, mods);
	}
}