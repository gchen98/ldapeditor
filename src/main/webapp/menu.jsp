<!DOCTYPE html>
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<html>
<jsp:useBean id="contact" scope="request" class="web.ContactBean"/>
<head><title>Pioneer Directory Administration Tool</title>
<link rel="stylesheet" href="/ldapeditor/ldap.css" style="text/css"/>
</head>
<body>
<b>
<jsp:getProperty name="contact" property="statusMesg"/>
</b>
<form method="get" name="menu" accept-charset="UTF-8" action="/ldapeditor/controller">
<h2>Choose an option below</h2>
<li><a href="search.jsp">Search</a></li>
<li><a href="add.jsp">Add a new entry</a></li>
<li><a href="index.jsp">Goto Login Page</a></li>
</form>
</body>
</html>
