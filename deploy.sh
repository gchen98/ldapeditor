#!/bin/bash

sudo systemctl stop tomcat8
TOMCAT_HOME=/var/lib/tomcat8
sudo -u tomcat8 rm -fr ${TOMCAT_HOME}/webapps/ldapeditor
sudo -u tomcat8 cp target/ldapeditor.war ${TOMCAT_HOME}/webapps/
sudo systemctl start tomcat8
