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

<%@ include file="modify_table.jsp" %>

Save as new record?<input type="checkbox" name="copy" value="1"></input><br>
<input type="button" name="savebutton" value="Save" onclick="javascript:doSave()"/>
<input type="button" name="deletebutton" value="Delete this record" onclick="javascript:doDelete()"/>
<a href="/ldapeditor/controller?action=search_form">Return to search</a>
</form>
</body>
</html>
