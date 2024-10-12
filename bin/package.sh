mvn package
sudo rm -fr /var/lib/tomcat10/webapps/ldapeditor
sudo cp target/*.war /var/lib/tomcat10/webapps/
