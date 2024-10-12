<jsp:useBean id="contact" scope="session" class="com.caseyandgary.ldapeditor.web.ContactBean" />

<!DOCTYPE html>
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<html>
<head><title>New Contact Page</title>
<link rel="stylesheet" href="/ldapeditor/ldap.css" style="text/css"/>
<script language="Javascript">
function validate() {
  if ((document.adder.cn.value=='') || (document.adder.lastName.value=='')) {
    alert("You must fill in the fields for unique name and last name");
  }
  else {
    document.adder.submit();
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
<form name="adder" method="post" accept-charset="UTF-8" action="/ldapeditor/controller">

<input type="hidden" name="action" value="add"/>
<b>
<jsp:getProperty name="contact" property="statusMesg"/>
</b>
<hr>

<%@ include file="modify_table.jsp" %>

<input type="button" onclick="javascript:validate()" value="Add"></input>
<a href="/ldapeditor/controller?action=search_form">Return to search</a>
</form>
</body>
</html>
