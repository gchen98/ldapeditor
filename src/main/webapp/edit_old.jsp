<%@page import="java.util.Enumeration" %>
<%@page import="java.util.Properties" %>

<jsp:useBean id="contact" scope="request" class="web.ContactBean" />

<!DOCTYPE html>
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<html>
<head><title>Edit Contact Page</title>
<link rel="stylesheet" href="/ldapeditor/ldap.css" style="text/css"/>

<script langage="javascript">
function doSave() {
  document.editform.action.value="edit";
  document.editform.submit();
}
function doDelete() {
 if (confirm("Are you sure you want to delete this contact?")) {
  document.editform.action.value="delete";
  document.editform.submit();
 }
}
function myPopup() 
  { 
   //window.open('http://www.caseyandgary.com/departmentkey.html','myExample3','location=no,menubar=no,width=450,height=450, resizable=yes,toolbar=no') 
   window.open('departmentkey.html','myExample3','location=no,menubar=no,resizable=yes') 
  } 

</script>
</head>
<body>
<form method="post" name="editform" accept-charset="UTF-8" action="/ldapeditor/controller">
<input type="hidden" name="action"/>
<input name="old_cn" type="hidden" value="<jsp:getProperty name="contact" property="cn"/>"/>

<b>
<jsp:getProperty name="contact" property="statusMesg"/>
</b>
<hr>
<table>
<tr><td>Unique Name: </td>
<td>
<input name="cn" type="text" size="20" value="<jsp:getProperty name="contact" property="cn"/>"/>
</td></tr>
<tr><td>First Name: </td>
<td><input name="firstName" type="text" size="20" value="<jsp:getProperty name="contact" property="firstName"/>"/></td></tr>
<tr><td>Last Name: </td>
<td><input name="lastName" type="text" size="20" value="<jsp:getProperty name="contact" property="lastName"/>"/></td></tr>
<tr><td>Email Address: </td>
<td><input name="email" type="text" size="20" value="<jsp:getProperty name="contact" property="email"/>"/></td></tr>
<tr><td>Mobile Phone: </td>
<td><input name="mobile" type="text" size="20" value="<jsp:getProperty name="contact" property="mobile"/>"/></td></tr>
<tr><td>Pager: </td>
<td><input name="pager" type="text" size="20" value="<jsp:getProperty name="contact" property="pager"/>"/></td></tr>
<tr><td>Company: </td>
<td><input name="company" type="text" size="20" value="<jsp:getProperty name="contact" property="company"/>"/></td></tr>
<tr><td colspan="2"><b>Contact Group</b>&nbsp; <!--input type="button" name="deptbutton" value="Help" onclick="javascript:myPopup()"/-->: </td>
<!--input name="department" type="text" size="20" value="<jsp:getProperty name="contact" property="department"/>"/--></tr>
<%java.util.Properties deptProp = (java.util.Properties)request.getSession().getAttribute(web.Controller.DEPT_PROP);
  Enumeration enumer = deptProp.propertyNames();
  while(enumer.hasMoreElements()){
    String key = (String)enumer.nextElement();
    String val = (String)deptProp.get(key);
    out.print("<tr><td>");
    out.print("<input type=\"checkbox\" name=\"department\" id=\"department\" value=\""+key+"\"");
    if (contact.isInDepartment(key)){
      out.print(" checked");
    }
    out.println("></input>");
    out.print("</td><td>"+val+"</td></tr>");
  }
%>

<tr><td>Job Title: </td>
<td><input name="jobTitle" type="text" size="20" value="<jsp:getProperty name="contact" property="jobTitle"/>"/></td></tr>
<!--
<tr><td colspan="2" align="center"><b>Business Section</b></td></tr>
-->
<tr><td>Address: </td>
<td><input name="businessAddress" type="text" size="20" value="<jsp:getProperty name="contact" property="businessAddress"/>"/></td></tr>
<tr><td>City: </td>
<td><input name="businessCity" type="text" size="20" value="<jsp:getProperty name="contact" property="businessCity"/>"/></td></tr>
<tr><td>State: </td>
<td><input name="businessState" type="text" size="20" value="<jsp:getProperty name="contact" property="businessState"/>"/></td></tr>
<tr><td>Zip: </td>
<td><input name="businessZip" type="text" size="20" value="<jsp:getProperty name="contact" property="businessZip"/>"/></td></tr>
<tr><td>Business Phone: </td>
<td><input name="businessPhone" type="text" size="20" value="<jsp:getProperty name="contact" property="businessPhone"/>"/></td></tr>
<tr><td>Fax: </td>
<td><input name="businessFax" type="text" size="20" value="<jsp:getProperty name="contact" property="businessFax"/>"/></td></tr>
<!--
<tr><td colspan="2" align="center"><b>Personal Section</b></td></tr>
-->
<tr><td>Home Phone: </td>
<td><input name="homePhone" type="text" size="20" value="<jsp:getProperty name="contact" property="homePhone"/>"/></td></tr>
<tr><td>Notes: </td>
<td><input name="notes" type="text" size="20" value="<jsp:getProperty name="contact" property="notes"/>"/></td></tr>
</table>
Save as new record?<input type="checkbox" name="copy" value="1"></input><br>
<input type="button" name="savebutton" value="Save" onclick="javascript:doSave()"/>
<input type="button" name="deletebutton" value="Delete this record" onclick="javascript:doDelete()"/>
<a href="/ldapeditor/search.jsp">Return to search</a>
</form>
</body>
</html>