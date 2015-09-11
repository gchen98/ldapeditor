<%@ page contentType="text/html; charset=UTF-8" %>
<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01 Transitional//EN">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<html>
   <head>
     <title>Character encoding test page</title>
<script language="Javascript" charset="utf-8">
  function doEdit(cn) {
    //alert(cn);
    //document.search.edit_id.value=cn;
    document.search.submit();
  }
</script>
   </head>
   <body>
     <p>Data posted to this form was:
     <%
       request.setCharacterEncoding("UTF-8");
       out.print(request.getParameter("edit_id"));
System.err.println("JSPDEBUG: "+request.getParameter("edit_id"));
     %>

     </p>
     <form name="search" method="POST" action="test.jsp">
       <input type="hidden" name="action" value="select"/>
       <input type="text" name="edit_id">
        <a href="javascript:doEdit('<%request.setCharacterEncoding("UTF-8"); out.print(request.getParameter("edit_id"));%>')">Submit </a>
       <!--input type="submit" value="Submit" /-->
       <input type="reset" value="Reset" />
     </form>
   </body>
</html>
