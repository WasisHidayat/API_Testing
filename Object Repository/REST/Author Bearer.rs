<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Author Bearer</name>
   <tag></tag>
   <elementGuidId>0c01e14d-9d47-448b-a25a-f189d3b79246</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;userId\&quot;:\&quot;${user}\&quot;,\n    \&quot;password\&quot;:\&quot;${pass}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>9.2.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://fifada-qa-lb01.fifgroup.co.id/backend/user/login</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'fiona'</defaultValue>
      <description></description>
      <id>1cd726b1-2ff3-4072-8d04-9ac1829fca32</id>
      <masked>false</masked>
      <name>user</name>
   </variables>
   <variables>
      <defaultValue>'zQe9Rk4qqvzs3Xyz'</defaultValue>
      <description></description>
      <id>017ec432-246b-40ff-a490-b27cc18a189d</id>
      <masked>false</masked>
      <name>pass</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

GlobalVariable.Get_Token = WS.getElementPropertyValue(response, 'result.accessToken')
System.out.println (GlobalVariable.Get_Token)
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
