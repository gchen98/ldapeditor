package com.caseyandgary.ldapeditor.ldap;

import javax.naming.*;
import javax.naming.directory.*;
import java.util.Hashtable;

public abstract class DirContextAdapter implements DirContext {	
	
	//String type;
	//Attributes myAttrs;
	
	public abstract Attributes getAttributes(String name, String[] names) throws NamingException;	
	
	public abstract Attributes getAttributes(Name name) throws NamingException;	
	
	public abstract Attributes getAttributes(Name name, String[] ids) throws NamingException;
	
	public abstract Attributes getAttributes(String name) throws NamingException;
	
	// not used for this example
	
	public Object lookup(Name name) throws NamingException {
		throw new OperationNotSupportedException();
	}
	
	public Object lookup(String name) throws NamingException {
		throw new OperationNotSupportedException();
	}
	
	public void bind(Name name, Object obj) throws NamingException {
		throw new OperationNotSupportedException();
	}
	
	public void bind(String name, Object obj) throws NamingException {
		throw new OperationNotSupportedException();
	}
	
	public void rebind(Name name, Object obj) throws NamingException {
		throw new OperationNotSupportedException();
	}
	
	public void rebind(String name, Object obj) throws NamingException {
		throw new OperationNotSupportedException();
	}
	
	public void unbind(Name name) throws NamingException {
		throw new OperationNotSupportedException();
	}
	
	public void unbind(String name) throws NamingException {
		throw new OperationNotSupportedException();
	}
	
	public void rename(Name oldName, Name newName) throws NamingException {
		throw new OperationNotSupportedException();
	}
	
	public void rename(String oldName, String newName) throws NamingException {
		throw new OperationNotSupportedException();
	}
	
	public NamingEnumeration list(Name name) throws NamingException {
		throw new OperationNotSupportedException();
	}
	
	public NamingEnumeration list(String name) throws NamingException {
		throw new OperationNotSupportedException();
	}
	
	public NamingEnumeration listBindings(Name name) throws NamingException {
		throw new OperationNotSupportedException();
	}
	public NamingEnumeration listBindings(String name) throws NamingException {
		throw new OperationNotSupportedException();
	}
	public void destroySubcontext(Name name) throws NamingException {
		throw new OperationNotSupportedException();
	}
	public void destroySubcontext(String name) throws NamingException {
		throw new OperationNotSupportedException();
	}
	public Context createSubcontext(Name name) throws NamingException {
		throw new OperationNotSupportedException();
	}
	public Context createSubcontext(String name) throws NamingException {
		throw new OperationNotSupportedException();
	}
	public Object lookupLink(Name name) throws NamingException {
		throw new OperationNotSupportedException();
	}
	public Object lookupLink(String name) throws NamingException {
		throw new OperationNotSupportedException();
	}
	public NameParser getNameParser(Name name) throws NamingException {
		throw new OperationNotSupportedException();
	}
	public NameParser getNameParser(String name) throws NamingException {
		throw new OperationNotSupportedException();
	}
	public String composeName(String name, String prefix)
		throws NamingException {
		throw new OperationNotSupportedException();
	}
	
	public Name composeName(Name name, Name prefix)
		throws NamingException {
		throw new OperationNotSupportedException();
	}
	
	public Object addToEnvironment(String propName, Object propVal) 
		throws NamingException {
		throw new OperationNotSupportedException();
	}
	public Object removeFromEnvironment(String propName) 
		throws NamingException {
		throw new OperationNotSupportedException();
	}
	public Hashtable getEnvironment() throws NamingException {
		throw new OperationNotSupportedException();
	}
	public void close() throws NamingException {
		throw new OperationNotSupportedException();
	}
	
	
	// -- DirContext
	public void modifyAttributes(Name name, int mod_op, Attributes attrs)
		throws NamingException  {
		throw new OperationNotSupportedException();
	}
	public void modifyAttributes(String name, int mod_op, Attributes attrs)
		throws NamingException  {
		throw new OperationNotSupportedException();
	}
	public void modifyAttributes(Name name, ModificationItem[] mods)
		throws NamingException  {
		throw new OperationNotSupportedException();
	}
	public void modifyAttributes(String name, ModificationItem[] mods)
		throws NamingException  {
		throw new OperationNotSupportedException();
	}
	public void bind(Name name, Object obj, Attributes attrs)
		throws NamingException  {
		throw new OperationNotSupportedException();
	}
	
	public void bind(String name, Object obj, Attributes attrs)
		throws NamingException  {
		throw new OperationNotSupportedException();
	}
	public void rebind(Name name, Object obj, Attributes attrs)
		throws NamingException  {
		throw new OperationNotSupportedException();
	}
	public void rebind(String name, Object obj, Attributes attrs)
		throws NamingException  {
		throw new OperationNotSupportedException();
	}
	public DirContext createSubcontext(Name name, Attributes attrs)
		throws NamingException  {
		throw new OperationNotSupportedException();
	}
	public DirContext createSubcontext(String name, Attributes attrs)
		throws NamingException  {
		throw new OperationNotSupportedException();
	}
	
	public DirContext getSchema(Name name) throws NamingException  {
		throw new OperationNotSupportedException();
	}
	
	public DirContext getSchema(String name) throws NamingException  {
		throw new OperationNotSupportedException();
	}
	
	public DirContext getSchemaClassDefinition(Name name)
		throws NamingException  {
		throw new OperationNotSupportedException();
	}
	
	public DirContext getSchemaClassDefinition(String name)
		throws NamingException  {
		throw new OperationNotSupportedException();
	}
	
	public NamingEnumeration search(Name name, 
		Attributes matchingAttributes,
		String[] attributesToReturn)
		throws NamingException  {
		throw new OperationNotSupportedException();
	}
	
	public NamingEnumeration search(String name, 
		Attributes matchingAttributes,
		String[] attributesToReturn)
		throws NamingException  {
		throw new OperationNotSupportedException();
	}
	
	public NamingEnumeration search(Name name,
		Attributes matchingAttributes)
		throws NamingException  {
		throw new OperationNotSupportedException();
	}
	
	public NamingEnumeration search(String name,
		Attributes matchingAttributes)
		throws NamingException  {
		throw new OperationNotSupportedException();
	}
	public NamingEnumeration search(Name name, 
		String filter,
		SearchControls cons)
		throws NamingException  {
		throw new OperationNotSupportedException();
	}
	
	public NamingEnumeration search(String name, 
		String filter,
		SearchControls cons)
		throws NamingException  {
		throw new OperationNotSupportedException();
	}
	
	public NamingEnumeration search(Name name,
		String filterExpr,
		Object[] filterArgs,
		SearchControls cons)
		throws NamingException  {
		throw new OperationNotSupportedException();
	}
	
	public NamingEnumeration search(String name,
		String filterExpr,
		Object[] filterArgs,
		SearchControls cons)
		throws NamingException  {
		throw new OperationNotSupportedException();
	}
	
	public String getNameInNamespace() throws NamingException {
		throw new OperationNotSupportedException();
	}
	
	
}

