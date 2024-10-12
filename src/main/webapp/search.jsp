<%@page import="java.util.Enumeration" %>
<%@page import="com.caseyandgary.ldapeditor.ldap.Contact" %>
<%@page import="java.util.Properties" %>
<%@page contentType="text/html;charset=UTF-8"%>
<jsp:useBean id="contact" scope="request" class="com.caseyandgary.ldapeditor.web.ContactBean" />
<% request.setCharacterEncoding("UTF-8"); %>
<!DOCTYPE html>
<meta name="viewport" content="width=device-width, initial-scale=1.0">

<html>
<head><title>Search Page</title>
<link rel="stylesheet" href="/ldapeditor/ldap.css" style="text/css"/>

<script language="Javascript" charset="utf-8">
function addslashes( str ) {
    return (str + '').replace(/[\\"']/g, '\\$&').replace(/\u0000/g, '\\0');
}

function doEdit(cn) {
  //document.search.edit_id.value=addslashes(cn);
  document.search.edit_id.value=cn;
  //alert(document.search.edit_id.value);
  document.search.action.value='select';
  document.search.submit();
}

function checkSubmit() {
  //if (document.search.cn.value=='') {
  //  alert("Please fill in the name field");
  //}
  //else {
    document.search.action.value='search';
    document.search.submit();  
  //}
}
function myPopup() 
  { 
   //window.open('departmentkey.html','myExample3','location=no,menubar=no,width=450,height=450, resizable=yes,toolbar=no') 
   window.open('departmentkey.html','myExample3','location=no,menubar=no,resizable=yes')
  } 
</script>

</head>
<body>



<%
java.util.List resultsList = contact.getResults();
if (resultsList!=null){
	%>
<h3>Search Results:</h3>
	<%
	  java.util.Iterator it = resultsList.iterator();
        boolean exists=it.hasNext();
        if (exists) {%>
         <table>
       	<tr>
	<th>Unique Name</th>
	<th>Email Address</th>
	<th>Mobile Phone</th>
	</tr>
          <%
          while (it.hasNext()) {
		Contact nextContact = (Contact)it.next();
		String cn = nextContact.getCn();
                //String newcn = org.apache.commons.lang.StringEscapeUtils.escape(cn);
		%>
		<tr>		
		<td>
		<a href="javascript:doEdit(&quot;<%=cn%>&quot;)">
		<%=cn%>
		</a>
		</td>
		<td><a href="mailto:<%=nextContact.getEmail()%>"><%=nextContact.getEmail()%></a></td>
		<td><a href="tel:<%=nextContact.getMobileByIndex(0)%>"><%=nextContact.getMobileByIndex(0)%></a></td>
		</tr>
           <%
         }
       }
       else {
          out.println("No matching contacts");
       }
	
%>
</table>
<%
}
%>
<form name="search" method="post" accept-charset="UTF-8" action="/ldapeditor/controller">
<b>
<%=contact.getStatusMesg()%>
</b>
<hr>
<b>
Search by name and/or contact group:
</b>
<input type="hidden" name="action" value="search"/>
<input type="hidden" name="edit_id"/>
<table>
<tr><td>Name: </td><td><input name="cn" type="text" size="20"></input></td></tr>
<tr><td>Contact Group:<!--input type="button" name="deptbutton" value="Help" onclick="javascript:myPopup()"/--></td>
<td><!--input name="dept" type="text" size="20"-->
<select name="dept">
<%java.util.Properties deptProp = (java.util.Properties)request.getSession().getAttribute(web.Controller.DEPT_PROP);
Enumeration enumer = deptProp.propertyNames();
%>
<option value="" selected="true">Search all contacts</option>
<%
while(enumer.hasMoreElements()){
  String key = (String)enumer.nextElement();
  String val = deptProp.getProperty(key);
  out.print("<option value=\""+key+"\"");
  out.println(">"+val+"</option>");
}
%>
</select>
</input></td></tr>
</table>
<input type="checkbox" name="sort" value="1" checked></input>Alphabetize results?<br>
<input type="button" onclick="javascript:checkSubmit()" value="Search"></input>
<!--input type="submit" value="Search"></input-->
<br>
<a href="/ldapeditor/controller?action=add_form">Add a new contact</a>
<br>
<a href="/ldapeditor/controller?action=logout">Logout</a>
<br>
<!--a href="/ldapeditor/menu.jsp">Return to main menu</a-->
</form>
</body>
</html>
