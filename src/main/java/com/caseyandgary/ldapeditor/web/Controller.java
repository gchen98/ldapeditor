package com.caseyandgary.ldapeditor.web;

//import java.beans.*;
//import javax.servlet.http.*;
//import javax.servlet.*;
import jakarta.servlet.http.*;
import jakarta.servlet.*;
import java.io.*;
import java.util.*;
//import jakarta.naming.NamingException;
//import jakarta.naming.directory.Attribute;

import com.caseyandgary.ldapeditor.ldap.*;

public class Controller
	extends HttpServlet{
	
	public static final String DEPT_PROP = "dept_prop";
	private final String LDAP_PROPERTYFILE="ldap_prop.txt";
	private final String USERS_PROPERTYFILE="users_prop.txt";
	private final String OU_PROPERTYFILE="ou_prop.txt";
	private final String CURR_USER = "CURR_USER";
	private final String CURR_OU = "CurrentOU";
	private final String DEFAULT_OU = "Internal";
	private LDAPConnector ldap;	
        private ServletConfig config;
	private ContactBean contactBean;
	private Properties userProp;
	private Properties ouProp;
	private Properties ldapProp;	
        private Properties deptProp;
	boolean secure;
	
	public void init(ServletConfig config)
		throws ServletException {
		super.init(config);		
		try {
                        this.config = config;
			String secureFlag = config.getInitParameter("secure");
			secure = Boolean.valueOf(secureFlag).booleanValue();
			//secure = true;
			ldapProp = new Properties();
			ldapProp.load(config.getServletContext().getResourceAsStream("/WEB-INF/"+LDAP_PROPERTYFILE));
			userProp = new Properties();
			userProp.load(config.getServletContext().getResourceAsStream("/WEB-INF/"+USERS_PROPERTYFILE));
			ouProp = new Properties();
			ouProp.load(config.getServletContext().getResourceAsStream("/WEB-INF/"+OU_PROPERTYFILE));
		}
		catch (IOException ex) {
			throw new ServletException("Property files couldn't load because: "+ex.getMessage());
		}		
	}
	
	private Contact toContact(HttpServletRequest request, HttpServletResponse response) throws LDAPException{
		Contact contact = new Contact();
		contact.setCn(request.getParameter("cn"));
		contact.setFirstName(request.getParameter("firstName"));
		contact.setLastName(request.getParameter("lastName"));		
		contact.setCompany(request.getParameter("company"));
		contact.setJobTitle(request.getParameter("jobTitle"));
		contact.setNotes(request.getParameter("notes"));
		contact.setEmail(request.getParameter("email"));
		contact.setEmailSecondary(request.getParameter("emailSecondary"));

		contact.setBusinessPhone(request.getParameterValues("businessPhone"));
		contact.setHomePhone(request.getParameterValues("homePhone"));
		contact.setMobile(request.getParameterValues("mobile"));
		contact.setPager(request.getParameterValues("pager"));
		contact.setBusinessFax(request.getParameterValues("businessFax"));

		contact.setBusinessAddress(request.getParameter("businessAddress"));
		contact.setBusinessCity(request.getParameter("businessCity"));
		contact.setBusinessState(request.getParameter("businessState"));
		contact.setBusinessZip(request.getParameter("businessZip"));
		contact.setHomeAddress(request.getParameter("homeAddress"));
		contact.setHomeCity(request.getParameter("homeCity"));
		contact.setHomeState(request.getParameter("homeState"));
		contact.setHomeZip(request.getParameter("homeZip"));
                String[] depts = request.getParameterValues("department");		
                if (depts==null) throw new LDAPException("At least one contact group must be entered. Please hit the browser back button to retrieve previous field values.");
              
		contact.setDepartment(depts);
		return contact;
	}
	
	private void setContactBean(Contact contact)
        throws LDAPException {		
		contactBean.setCn(contact.getCn());
		contactBean.setFirstName(contact.getFirstName());
                
		contactBean.setLastName(contact.getLastName());
		contactBean.setCompany(contact.getCompany());
		contactBean.setJobTitle(contact.getJobTitle());
		contactBean.setNotes(contact.getNotes());
		contactBean.setEmail(contact.getEmail());
		contactBean.setEmailSecondary(contact.getEmailSecondary());

		contactBean.setBusinessPhone(contact.getBusinessPhone());
		contactBean.setHomePhone(contact.getHomePhone());
		contactBean.setMobile(contact.getMobile());
		contactBean.setPager(contact.getPager());
		contactBean.setBusinessFax(contact.getBusinessFax());

		contactBean.setBusinessAddress(contact.getBusinessAddress());
		contactBean.setBusinessCity(contact.getBusinessCity());
		contactBean.setBusinessState(contact.getBusinessState());
		contactBean.setBusinessZip(contact.getBusinessZip());		

		contactBean.setHomeAddress(contact.getHomeAddress());
		contactBean.setHomeCity(contact.getHomeCity());
		contactBean.setHomeState(contact.getHomeState());
		contactBean.setHomeZip(contact.getHomeZip());		

		contactBean.setDepartment(contact.getDepartmentList());
                contactBean.setTimestamp(contact.getTimestamp());
	}
	
	
	private void add(HttpServletRequest request, HttpServletResponse response)
		throws LDAPException{
		Contact contact =  toContact(request,response);
		try {			
			ldap.accessContact(contact,LDAPConnector.ADD,true);
			contactBean.setStatusMesg("Contact added successfully!  You may add another contact.");
		}		
		catch (LDAPException ex) {
			setContactBean(contact);
			throw new LDAPException(ex.getMessage());
		}		
	}
	
	private void edit(HttpServletRequest request, HttpServletResponse response)
		throws LDAPException {
		Contact contact = toContact(request,response);
		Contact queryContact = new Contact();
		queryContact.setCn(request.getParameter("cn"));
		Contact prevContact = new Contact();
		prevContact.setCn(request.getParameter("old_cn"));
                boolean copy_contact = request.getParameter("copy")!=null;
		try {			
			ldap.accessContact(queryContact,LDAPConnector.SEARCH,true);		
			List contactList = ldap.getSearchResults();
			if ((contactList==null) || (contactList.size()<1)) {
                                // this case means the new cn is new
				ldap.accessContact(contact, LDAPConnector.ADD,true);
				queryContact.setCn(request.getParameter("old_cn"));
                                if (!copy_contact){
				  ldap.accessContact(prevContact, LDAPConnector.DELETE,true);
                                }
			}
			else {
				if ((request.getParameter("old_cn").equals(request.getParameter("cn")))){
						ldap.accessContact(contact,LDAPConnector.EDIT,true);
				}
				else {
					throw new LDAPException("The unique name you entered is already in the directory.  Please create a new one.");
				}					
			}
			contactBean.setStatusMesg("Contact modified successfully!  You may search for another contact now.");
		}
		catch (LDAPException ex) {
			setContactBean(contact);
			throw new LDAPException(ex.getMessage());
		}
	}
		
		private void select(HttpServletRequest request, HttpServletResponse response)
			throws LDAPException{
			Contact contact = new Contact();
			contact.setCn(request.getParameter("edit_id"));
			ldap.accessContact(contact, LDAPConnector.SEARCH, true);
			List contactList = ldap.getSearchResults();
			Contact foundContact=null;
			if (contactList!=null) {
				// Should be unique so one element
				foundContact = (Contact)contactList.get(0);
				setContactBean(foundContact);			
			}		
		}
		
		private void search(HttpServletRequest request, HttpServletResponse response)
			throws LDAPException{
			Contact contact = new Contact();
			String cn = request.getParameter("cn");
			String dept = request.getParameter("dept");
                        String sort = request.getParameter("sort");
                        if (sort!=null){
                          ldap.setSort(true);
                        }else{
                          ldap.setSort(false);
                        }
			if (cn.length()>0)	contact.setCn(cn);
			if (dept.length()>0) contact.setDepartment(dept);
			ldap.accessContact(contact,LDAPConnector.SEARCH,false);		
			List contactList = ldap.getSearchResults();
			contactBean.setResultsList(contactList);								
			request.setAttribute("contact",contactBean);		
		}
		
		private void delete(HttpServletRequest request, HttpServletResponse response)
			throws LDAPException {
			Contact contact = new Contact();
			contact.setCn(request.getParameter("cn"));
			ldap.accessContact(contact, LDAPConnector.DELETE,true);		
			contactBean.setStatusMesg("Contact deleted successfully!  You may search for another contact now.");
		}
		
		private boolean login(HttpServletRequest request, HttpServletResponse response) {
			String user = request.getParameter("username");
			String password = request.getParameter("password");
			String ou = request.getParameter("ou");
                        try{
                          deptProp = new Properties();
                          java.io.InputStream stream = config.getServletContext().getResourceAsStream("/WEB-INF/dept."+ou+".txt");
                          if (stream!=null){
                            deptProp.load(config.getServletContext().getResourceAsStream("/WEB-INF/dept."+ou+".txt"));
                            request.getSession().setAttribute(DEPT_PROP,deptProp);
                          }else{
                            contactBean.setStatusMesg("This org unit has no contact groups initialized yet. Contact your administrator.");
                             return false;
                          }
                        }catch (IOException ex){
                          contactBean.setStatusMesg("There was a problem reading in the file for contact groups. Contact your administrator.");
                          return false;
                        }
			if (userProp.getProperty(user)==null) {
				contactBean.setStatusMesg("The username you entered does not exist. Please retry.");
			}
			else {
				if (!userProp.getProperty(user).equals(password)) {
					contactBean.setStatusMesg("Incorrect password. Please retry.");
				}
				else {
					String[] authUsers = ouProp.getProperty(ou).split(",");
					Arrays.sort(authUsers);
					int retCode = Arrays.binarySearch(authUsers,user);
					if (retCode>-1) {
						ldapProp.setProperty(LDAPConnector.PEOPLE_CONTEXT,"ou="+ou);
                                                // validate this session as true
						request.getSession().setAttribute(CURR_USER,user);
						return true;
					}
					else {
						contactBean.setStatusMesg("Not authorized to edit this Org Unit. Please retry.");
					}
				}
			}		
			return false;
		}
		
		private boolean checkAuth(HttpSession session) {
			if (session.getAttribute(CURR_USER) == null){
				if (secure) {
					contactBean.setStatusMesg("Your session has timed out. Please re-login");			
					return false;
				}
				else {
					ldapProp.setProperty(LDAPConnector.PEOPLE_CONTEXT,"ou="+DEFAULT_OU);
				}
			}
			return true;
		}


		public void doGet(HttpServletRequest request, HttpServletResponse response)
			throws ServletException, java.io.IOException {	
			handlePostAndGet(request,response);
		}

		
		public void doPost(HttpServletRequest request, HttpServletResponse response)
			throws ServletException, java.io.IOException {	
			handlePostAndGet(request,response);
		}

		private void handlePostAndGet(HttpServletRequest request, HttpServletResponse response)
			throws ServletException, java.io.IOException {	
			// This MVC design requires all POSTs to specifiy an action. Abort if none specified.
                        request.setCharacterEncoding("UTF8");
                        response.setCharacterEncoding("UTF8");
			
			
			String action = request.getParameter("action");
			if (action==null)	{
				PrintWriter writer = response.getWriter();
				writer.println("<html><head><title>Oops</title></head><body>Sorry, but no action was specified.</body></html>");
				return;
			}
			
			// first things first, make sure we have a contact bean to return user friendly error messages
			contactBean = new ContactBean();
			Contact contact = new Contact();
			// when in doubt go to main menu
			String pageToForward = "/menu.jsp";
			
			if (action.equals("login")) {
				if (login(request,response)) {
					// Successful
					pageToForward = "/search.jsp";
					//pageToForward = "/menu.jsp";
				}
				else {
					// Access denied
					pageToForward = "/index.jsp";
				}
				request.setAttribute("contact",contactBean);
				request.getRequestDispatcher(pageToForward).forward(request,response);
				return;
			}else if (action.equals("logout")){
				request.getSession().removeAttribute(CURR_USER);
				pageToForward = "/index.jsp";
				request.setAttribute("contact",contactBean);
			}else if (action.equals("add_form")){
				pageToForward = "/add.jsp";
				request.setAttribute("contact",contactBean);
			}else if (action.equals("search_form")){
				pageToForward = "/search.jsp";
				request.setAttribute("contact",contactBean);
                        
			}else {			
				// If we're in any page besides login page, let's make sure the session auth object is alive
				if (!checkAuth(request.getSession())){
					// Session must have died. Return to login page.
					request.setAttribute("contact",contactBean);
					request.getRequestDispatcher("/index.jsp").forward(request,response);
					return;
				}			
			}
			
			// Lazy instantiation.  We'll need the LDAP server on all pages besides Login page
			
			if (ldap==null) {
				try {
					ldap = new LDAPConnector(ldapProp);
				}
				catch (LDAPException ex) {				
					contactBean.setStatusMesg("Could not initialize connection to LDAP server because of message '" + ex.getMessage()+
						"'. Please try again later when the server is up again.");
				}
			}
			if (ldap!=null){
				try {
					ldap.setProperties(ldapProp);
					if (action.equals("add")) {
						pageToForward = "/add.jsp";
					        add(request,response);					
					}
					else if (action.equals("search")) {						
						pageToForward = "/search.jsp";
						search(request,response);					
					}					
					else if (action.equals("select")) {
						pageToForward = "/edit.jsp";
						try {
							select(request,response);
						}
						catch (LDAPException ex) {
							pageToForward = "/search.jsp";
							throw new LDAPException(ex.getMessage());
						}					
					}
					else if (action.equals("delete")) {
						pageToForward = "/search.jsp";					
						try {
							delete(request,response);
						}
						catch (LDAPException ex) {
							pageToForward = "/edit.jsp";
							throw new LDAPException(ex.getMessage());
						}
					}
					else if (action.equals("edit")) {
						pageToForward = "/search.jsp";
						try {
							edit(request,response);					
						}
						catch (LDAPException ex) {
							pageToForward = "/edit.jsp";
							throw new LDAPException(ex.getMessage());
						}
					}			
					else {
						PrintWriter writer = response.getWriter();
						writer.println("<html><head><title>Oops</title></head><body>Sorry, the action of "+
							action+" was not recognized.</body></html>");
					}	// end action if		
				} // end try
				
				// Make sure nothing slips thru
				
				catch (Exception ex) {
                                        ex.printStackTrace();
					contactBean.setStatusMesg(ex.getMessage());
					//contactBean.setStatusMesg("An error occured while trying to perform your action request.  The "+ "exception was of type '"+ex.getClass().getName()+"' and message '"+ex.getMessage() +"'. Please try again later after the problem has been fixed.");
				}
				
				// Worst case scenario
				
				catch (Error ex) {
					contactBean.setStatusMesg("A severe problem occured while trying to perform your action request.  The "+
						"exception was of type '"+ex.getClass().getName()+"' and message '"+ex.getMessage()
						+"'. Please try again later after the problem has been fixed.");
				}
			}				
			request.setAttribute("contact",contactBean);
			request.getRequestDispatcher(pageToForward).forward(request,response);
			
		}
		
	}
