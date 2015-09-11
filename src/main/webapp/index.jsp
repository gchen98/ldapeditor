<jsp:useBean id="contact" scope="request" class="web.ContactBean" />
<!DOCTYPE html>
<meta name="viewport" content="width=device-width, initial-scale=1.0">

<html>
<head><title>Contact Editor</title>
<link rel="stylesheet" href="/ldapeditor/ldap.css" style="text/css"/>
</head>
<body>
<b>
<jsp:getProperty name="contact" property="statusMesg"/>
<!--
<p>NOTE: The LDAP server is currently in read only mode until further notice.</p>
-->
</b>
<form name="form1" method="post" accept-charset="UTF-8" action="/ldapeditor/controller">
<input type="hidden" name="action" value="login"/>
<table align="center" valign="center">
<tr><td>Username: </td><td><input type="text" name="username"/></td></tr>
<tr><td>Password: </td><td><input type="password" name="password"/></td></tr>
<!--input type="hidden" name="ou" value="Public"/-->
<tr><td>Organizational Unit: </td><td>
<select name="ou">
<option value="Public" selected="true">Chen Family</option>
<option value="Employees">Pioneer Employees</option>
<option value="MetalsApp">Metals App</option>
<option value="JobQuotesApp">JobQuotes App</option>
</select>
<script language="JavaScript">;
document.form1.ou.selectedIndex=<%out.print(request.getParameter("OUIndex"));%>;
</script>
</td></tr>
<tr><td colspan="2" align="center"><input type="Submit" value="Login"/></td></tr>
</table>
</form>
</body>
</html>
