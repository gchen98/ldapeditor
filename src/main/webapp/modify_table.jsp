<%@page import="java.util.Enumeration" %>
<%@page import="java.util.Properties" %>

<table border="1">
<!-- name section -->
<tr>
<td>
<table>
<tr>
<th colspan="4" align="center">Names and company information</th>
</tr>
<tr>
<td>Full:</td> <td > <input name="cn" type="text" size="40" value="<jsp:getProperty name="contact" property="cn"/>"/></td><td colspan="2"/>
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
<tr><td>
<table>
<tr>
<th colspan='2' align="center">E-mail addresses</th>
</tr>
<tr>
 <td><table>
  <tr><td>primary:</td><td><input name="email" type="text" size="20" value="<jsp:getProperty name="contact" property="email"/>"/></td></tr>
 </table></td>
 <td><table>
  <tr><td>secondary:</td><td><input name="emailSecondary" type="text" size="20" value="<jsp:getProperty name="contact" property="secondaryEmail"/>"/></td></tr>
 </table></td>
</tr>
</table>
</td></tr>

<!-- phone section -->
<tr> <td>
<table>
<tr>
<th colspan='2' align="center">Phone Numbers</th>
</tr>
<tr>
 <td><table>
  <tr><th colspan="2" align="center">Primary</th></tr>
  <tr><td>work:</td><td><input name="businessPhone" type="text" size="20" value="<%=contact.getBusinessPhone(0)%>"/></td></tr>
  <tr><td>home:</td><td><input name="homePhone" type="text" size="20" value="<%=contact.getHomePhone(0)%>"/></td></tr>
  <tr><td>mobile:</td><td><input name="mobile" type="text" size="20" value="<%=contact.getMobile(0)%>"/></td></tr>
  <tr><td>pager:</td><td><input name="pager" type="text" size="20" value="<%=contact.getPager(0)%>"/></td></tr>
  <tr><td>fax:</td><td><input name="businessFax" type="text" size="20" value="<%=contact.getBusinessFax(0)%>"/></td></tr>
 </table></td>
 <td><table>
  <tr><th colspan="2" align="center">Secondary</th></tr>
  <tr><td>work:</td><td><input name="businessPhone" type="text" size="20" value="<%=contact.getBusinessPhone(1)%>"/></td></tr>
  <tr><td>home:</td><td><input name="homePhone" type="text" size="20" value="<%=contact.getHomePhone(1)%>"/></td></tr>
  <tr><td>mobile:</td><td><input name="mobile" type="text" size="20" value="<%=contact.getMobile(1)%>"/></td></tr>
  <tr><td>pager:</td><td><input name="pager" type="text" size="20" value="<%=contact.getPager(1)%>"/></td></tr>
  <tr><td>fax:</td><td><input name="businessFax" type="text" size="20" value="<%=contact.getBusinessFax(1)%>"/></td></tr>
 </table></td>
</tr>
</table>
</td></tr>

<!-- address section -->
<tr> <td>
<table>
<tr>
<th colspan='2' align="center">Mailing addresses</th>
</tr>
<tr>
 <td><table>
  <tr><th colspan="2" align="center">work</th></tr>
  <tr><td>street:</td><td><input name="businessAddress" type="text" size="20" value="<jsp:getProperty name="contact" property="businessAddress"/>"/></td></tr>
  <tr><td>city:</td><td><input name="businessCity" type="text" size="20" value="<jsp:getProperty name="contact" property="businessCity"/>"/></td></tr>
  <tr><td>state:</td><td><input name="businessState" type="text" size="20" value="<jsp:getProperty name="contact" property="businessState"/>"/></td></tr>
  <tr><td>zip:</td><td><input name="businessZip" type="text" size="20" value="<jsp:getProperty name="contact" property="businessZip"/>"/></td></tr>
 </table></td>
 <td><table>
  <tr><th colspan="2" align="center">personal</th></tr>
  <tr><td>street:</td><td><input name="homeAddress" type="text" size="20" value="<jsp:getProperty name="contact" property="homeAddress"/>"/></td></tr>
  <tr><td>city:</td><td><input name="homeCity" type="text" size="20" value="<jsp:getProperty name="contact" property="homeCity"/>"/></td></tr>
  <tr><td>state:</td><td><input name="homeState" type="text" size="20" value="<jsp:getProperty name="contact" property="homeState"/>"/></td></tr>
  <tr><td>zip:</td><td><input name="homeZip" type="text" size="20" value="<jsp:getProperty name="contact" property="homeZip"/>"/></td></tr>
 </table></td>
</tr>
</table>
</td></tr>

<!-- admin section -->
<tr>
<td><table>
<tr>
<th colspan='2' align="center">Miscellaneous</th>
</tr>
 <tr>
  <td>Notes: </td><td><input name="notes" type="text" size="40" value="<jsp:getProperty name="contact" property="notes"/>"/></td>
 </tr>
 <tr>
  <td>Last modified on: </td><td><jsp:getProperty name="contact" property="timestamp"/></td>
 </tr>
</table></td>
</tr>
<tr>
<td>
<!-- contact group table -->
<table>
<tr>
<th  colspan="2"><b>Contact Group</b>&nbsp; <!--input type="button" name="deptbutton" value="Help" onclick="javascript:myPopup()"/--></td>
</th>
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
<!-- contact group table end -->
</td>
</tr>
</table>
