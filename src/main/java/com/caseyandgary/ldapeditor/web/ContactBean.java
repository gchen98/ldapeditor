package com.caseyandgary.ldapeditor.web;

import java.util.Iterator;
import java.util.Set;
import java.util.HashSet;
import java.util.List;
import java.util.ArrayList;
import javax.servlet.jsp.JspWriter;

public class ContactBean
{
        public static final String DEFAULT_VAL="";
	private List resultsList = null;

	private String statusMesg = DEFAULT_VAL;
	private String action;
	private String currentOU = DEFAULT_VAL;
	
	private String cn=DEFAULT_VAL;
	private String firstName=DEFAULT_VAL;
	private String lastName=DEFAULT_VAL;
	private String company=DEFAULT_VAL;
	private String jobTitle=DEFAULT_VAL;
	private String notes=DEFAULT_VAL;
	private String email=DEFAULT_VAL;
	private String emailSecondary=DEFAULT_VAL;

	private String businessAddress=DEFAULT_VAL;
	private String businessCity=DEFAULT_VAL;
	private String businessState=DEFAULT_VAL;
	private String businessZip=DEFAULT_VAL;
	private String homeAddress=DEFAULT_VAL;
	private String homeCity=DEFAULT_VAL;
	private String homeState=DEFAULT_VAL;
	private String homeZip=DEFAULT_VAL;

	private List<String> businessPhoneList=new ArrayList<String>();
	private List<String> homePhoneList=new ArrayList<String>();
	private List<String> mobileList = new ArrayList<String>();
	private List<String> pagerList=new ArrayList<String>();
	private List<String> businessFaxList=new ArrayList<String>();

	private Set<String> departmentSet = new HashSet<String>();

	private String timestamp=DEFAULT_VAL;
	
	public String getStatusMesg() {
		return statusMesg;
	}
	
	public List getResults() {
		return resultsList;
	}
	
	
	public String getAction() {
		return action;
	}

	public String getCurrentOU() {
		return currentOU;
	}
	
	//getters
	//
	
	public String getCn() {
		return cn;
	}

	public String getFirstName() {
		return firstName;
	}
	
	public String getLastName() {
		return lastName;
	}

	public String getCompany() {
		return company;
	}

	public String getJobTitle() {
		return jobTitle;
	}

	public String getNotes() {
		return notes;
	}
	
	public String getEmail() {
		return email;
	}

	public String getSecondaryEmail() {
		return emailSecondary;
	}
	
	public String getBusinessAddress() {
		return businessAddress;
	}
	
	public String getBusinessCity() {
		return businessCity;
	}
	
	public String getBusinessState() {
		return businessState;
	}
	
	public String getBusinessZip() {
		return businessZip;
	}
	
	public String getHomeAddress() {
		return homeAddress;
	}
	
	public String getHomeCity() {
		return homeCity;
	}
	
	public String getHomeState() {
		return homeState;
	}
	
	public String getHomeZip() {
		return homeZip;
	}

	
	public String getBusinessPhone(int index) {
        	try{
			return businessPhoneList.get(index);
		}catch (IndexOutOfBoundsException ex){
			return DEFAULT_VAL;
		}
	}

	public String getHomePhone(int index) {
        	try{
			return homePhoneList.get(index);
		}catch (IndexOutOfBoundsException ex){
			return DEFAULT_VAL;
		}
	}

	public String getMobile(int index) {
        	try{
			return mobileList.get(index);
		}catch (IndexOutOfBoundsException ex){
			return DEFAULT_VAL;
		}
	}

	public String getPager(int index) {
        	try{
			return pagerList.get(index);
		}catch (IndexOutOfBoundsException ex){
			return DEFAULT_VAL;
		}
	}
	
	public String getBusinessFax(int index) {
        	try{
			return businessFaxList.get(index);
		}catch (IndexOutOfBoundsException ex){
			return DEFAULT_VAL;
		}
	}
	public String getDepartment(){
		return DEFAULT_VAL;
	}
	
	public boolean isInDepartment(String dept) {
		return departmentSet.contains(dept);
	}
	
	public String getTimestamp() {
		return timestamp;
	}

	// setters
	//
	public void setStatusMesg(String statusMesg) {
		this.statusMesg = statusMesg;
	}
	
	public void setAction(String action) {
		this.action = action;
	}
	
	public void setResultsList(List list) {
		this.resultsList = list;
	}	

	public void setCurrentOU(String currentOU){
		this.currentOU=currentOU;
	}
		
	public void setCn(String cn) {
		this.cn = cn;
	}

	public void setFirstName(String firstName) {
		this.firstName = firstName;
	}

	public void setLastName(String lastName) {
		this.lastName = lastName;
	}	

	public void setCompany(String company) {
		this.company = company;
	}

	public void setJobTitle(String jobTitle) {
		this.jobTitle = jobTitle;
	}
	
	public void setNotes(String notes) {
		this.notes = notes;
	}

	public void setEmail(String email) {
		this.email = email;
	}
	
	public void setEmailSecondary(String emailSecondary) {
		this.emailSecondary = emailSecondary;
	}

	public void setBusinessAddress(String businessAddress) {
		this.businessAddress = businessAddress;
	}
	
	public void setBusinessCity(String businessCity) {
		this.businessCity = businessCity;
	}
	
	public void setBusinessState(String businessState) {
		this.businessState = businessState;
	}
	
	public void setBusinessZip(String businessZip) {
		this.businessZip = businessZip;
	}

	public void setHomeAddress(String homeAddress) {
		this.homeAddress = homeAddress;
	}
	
	public void setHomeCity(String homeCity) {
		this.homeCity = homeCity;
	}
	
	public void setHomeState(String homeState) {
		this.homeState = homeState;
	}
	
	public void setHomeZip(String homeZip) {
		this.homeZip = homeZip;
	}
	
	public void setBusinessPhone(List<String> businessPhone) {
		this.businessPhoneList = businessPhone;
	}

	public void setHomePhone(List<String> homePhone) {
		this.homePhoneList = homePhone;
	}

	public void setMobile(List<String> mobile) {
		this.mobileList = mobile;
	}

	public void setPager(List<String> pager) {
		this.pagerList = pager;
	}
	
	public void setBusinessFax(List<String> businessFax) {
		this.businessFaxList = businessFax;
	}

	public void setDepartment(List<String> deptList) {
                Iterator<String> it = deptList.iterator();
                while(it.hasNext()){
		  this.departmentSet.add((String)it.next());
                }
	}

	public void setTimestamp(String timestamp) {
		this.timestamp = timestamp;
	}
}

