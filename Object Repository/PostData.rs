<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PostData</name>
   <tag></tag>
   <elementGuidId>8e6899ce-9a6a-430f-8841-04bfce28fa11</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>${Token Local}</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;lob\&quot; : \&quot;MMU\&quot;,\n    \&quot;fullName\&quot; : \&quot;Kurosaki\&quot;,\n    \&quot;mobilePhone\&quot; : \&quot;081585598942\&quot;,\n    \&quot;noKtp\&quot; : \&quot;3173082011800008\&quot;,\n    \&quot;birthDate\&quot; : \&quot;1980-11-20\&quot;,\n    \&quot;birthPlace\&quot; : \&quot;JAKARTA\&quot;,\n    \&quot;gender\&quot; : \&quot;M\&quot;,\n    \&quot;mothersName\&quot; : \&quot;RODIAH\&quot;,\n    \&quot;financingType\&quot; : \&quot;U\&quot;,\n    \&quot;packageCode\&quot; : \&quot;PR0002\&quot;,\n    \&quot;tenor\&quot; : 12,\n    \&quot;downPayment\&quot; : 500000,\n    \&quot;channel\&quot; : \&quot;FNA\&quot;\n}\n&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/plain</value>
      <webElementGuid>4ff99d20-068c-4257-be2d-10fb67d3879c</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${Token Local}</value>
      <webElementGuid>c6a8f545-1a02-45be-af4c-f4b8c0426fc5</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.2.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://fifada-qa-lb01.fifgroup.co.id/backend/leads/draft</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Get_Token</defaultValue>
      <description></description>
      <id>1cd5c04d-46c6-41f4-a51c-ac129abf40ba</id>
      <masked>false</masked>
      <name>Token Local</name>
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

GlobalVariable.Draft_Id = WS.getElementPropertyValue(response, 'result.draftId')
System.out.println(GlobalVariable.Draft_Id)
	
	
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
