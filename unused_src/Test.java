package com.caseyandgary.ldapeditor;

import javax.naming.Context;
import javax.naming.InitialContext;
import javax.naming.NamingException;
import javax.naming.NameAlreadyBoundException;
import javax.naming.directory.*;
import java.util.*;

public class Test {
	//final static String ldapServerName = "mail.pioneerind.com";
	final static String ldapServerName = "ucla.pioneerind.com";
	//final static String rootContext = "o=jndiTest";
	final static String rootContext = "o=ucla";
	final static String rootdn = "cn=Manager,"+rootContext;
	//final static String rootpass = "secret";
	final static String rootpass = "cas98ey";
	
	
	public static void main( String[] args ) {
		// set up environment to access the server
		
		Properties env = new Properties();
		
		env.put( Context.INITIAL_CONTEXT_FACTORY,
			"com.sun.jndi.ldap.LdapCtxFactory" );
		env.put( Context.PROVIDER_URL, "ldap://" + ldapServerName + "/" );
		env.put( Context.SECURITY_PRINCIPAL, rootdn );
		env.put( Context.SECURITY_CREDENTIALS, rootpass );
		
		try {
			// obtain initial directory context using the environment
			DirContext ctx = new InitialDirContext( env );
			
			// now, create the root context, which is just a subcontext
			// of this initial directory context.
			
			Attributes attrs = new BasicAttributes();
			attrs.put("objectclass","top");			
			attrs.put("objectclass","organization");
			
			ctx.createSubcontext( rootContext,attrs);
		} catch ( NameAlreadyBoundException nabe ) {
			System.err.println( rootContext + " has already been bound!" );
		} catch ( Exception e ) {
			System.err.println( e );
		}
	}
}
