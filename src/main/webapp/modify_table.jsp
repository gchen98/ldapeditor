<%@page import="java.util.Enumeration" %>
<%@page import="java.util.Properties" %>

<table border="1">
<!-- name section -->
<tr>
<td>
<table>
<th><td colspan="4" align="center">Names and company information</td></th>
<tr>
<td>Full:</td> <td > <input name="cn" type="text" size="20" value="<jsp:getProperty name="contact" property="cn"/>"/></td><td colspan="2"/>
</tr>
<tr>
<td>First:</td><td><input name="firstName" type="text" size="20" value="<jsp:getProperty name="contact" property="firstName"/>"/></td><td>Last: </td><td><input name="lastName" type="text" size="20" value="<jsp:getProperty name="contact" property="lastName"/>"/></td>
</tr>
<tr>
<td>Company: </td><td><input name="company" type="text" size="20" value="<jsp:getProperty name="contact" property="company"/>"/></td> <td>Job Title: </td> <td><input name="jobTitle" type="text" size="20" value="<jsp:getProperty name="contact" property="jobTitle"/>"/></td>
</tr>
</table>
</td>
</tr>
<!-- email section -->
<tr> <td>
<table>
<th><td colspan="4" align="center">E-mail addresses</td></th>
<tr>
<td>Primary:</td><td><input name="email" type="text" size="20" value="<jsp:getProperty name="contact" property="email"/>"/></td> <td>Secondary:</td><td><input name="emailSecondary" type="text" size="20" value="<jsp:getProperty name="contact" property="secondaryEmail"/>"/></td>
</tr>
</table>
</td></tr>
<!-- phone section -->
<tr><td>
<table>
<th><td colspan="4" align="center">Phone numbers</td></th>
<tr>
<td colspan="2" align="center">Primary</td>
<td colspan="2" align="center">Secondary</td>
</tr>
<tr>
<td>work: </td> <td><input name="businessPhone" type="text" size="20" value="<%=contact.getBusinessPhone(0)%>"/></td> <td>work: </td> <td><input name="businessPhone" type="text" size="20" value="<%=contact.getBusinessPhone(1)%>"/></td>
</tr>
<tr>
<td>Home: </td> <td><input name="homePhone" type="text" size="20" value="<%=contact.getHomePhone(0)%>"/></td> <td>Home: </td> <td><input name="homePhone" type="text" size="20" value="<%=contact.getHomePhone(1)%>"/></td>
</tr>
<tr>
<td>mobile:</td><td><input name="mobile" type="text" size="20" value="<%=contact.getMobile(0)%>"/></td><td>mobile:</td><td><input name="mobile" type="text" size="20" value="<%=contact.getMobile(1)%>"/></td>
</tr>
<tr>
<td>pager: </td><td><input name="pager" type="text" size="20" value="<%=contact.getPager(0)%>"/></td><td>pager: </td><td><input name="pager" type="text" size="20" value="<%=contact.getPager(1)%>"/></td>
</tr>
<tr>
<td>Fax:</td><td><input name="businessFax" type="text" size="20" value="<%=contact.getBusinessFax(0)%>"/></td> <td>Fax:</td><td><input name="businessFax" type="text" size="20" value="<%=contact.getBusinessFax(1)%>"/></td>
</tr>
</table>
</td></tr>
<!-- address section -->
<tr><td>
<table>
<th><td colspan="4" align="center">Mailing addresses</td></th>
<tr>
<td colspan="2" align="center">Work</td>
<td colspan="2" align="center">Home</td>
</tr>
<tr>
<td>Street: </td> <td><input name="businessAddress" type="text" size="20" value="<jsp:getProperty name="contact" property="businessAddress"/>"/></td> <td>Street: </td> <td><input name="homeAddress" type="text" size="20" value="<jsp:getProperty name="contact" property="homeAddress"/>"/></td>
</tr>
<tr>
<td>City: </td> <td><input name="businessCity" type="text" size="20" value="<jsp:getProperty name="contact" property="businessCity"/>"/></td> <td>City: </td> <td><input name="homeCity" type="text" size="20" value="<jsp:getProperty name="contact" property="homeCity"/>"/></td>
</tr>
<tr>
<td>State: </td> <td><input name="businessState" type="text" size="20" value="<jsp:getProperty name="contact" property="businessState"/>"/></td> <td>State: </td> <td><input name="homeState" type="text" size="20" value="<jsp:getProperty name="contact" property="homeState"/>"/></td>
</tr>
<tr>
<td>Zip: </td> <td><input name="businessZip" type="text" size="20" value="<jsp:getProperty name="contact" property="businessZip"/>"/></td> <td>Zip: </td> <td><input name="homeZip" type="text" size="20" value="<jsp:getProperty name="contact" property="homeZip"/>"/></td>
</tr>
</table>
</td></tr>
<!-- admin section -->
<tr><td>
<table>
<th><td colspan="2" align="center">Administrative settings</td></th>
<tr>
<td>
<table>
<tr>
<td>Notes: </td><td><input name="notes" type="text" size="40" value="<jsp:getProperty name="contact" property="notes"/>"/></td>
</tr>
<tr>
<td>Last modified on: </td><td><jsp:getProperty name="contact" property="timestamp"/></td>
</tr>
</table>
</td>
<td colspan="2">
<table>
<tr>
<td colspan="2"><b>Contact Group</b>&nbsp; <!--input type="button" name="deptbutton" value="Help" onclick="javascript:myPopup()"/-->: </td>
</tr>
<!--input name="department" type="text" size="20" value="<jsp:getProperty name="contact" property="department"/>"/-->
</tr>
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
</table>
</td>
</tr>
</table>
</td></tr>
</table>
