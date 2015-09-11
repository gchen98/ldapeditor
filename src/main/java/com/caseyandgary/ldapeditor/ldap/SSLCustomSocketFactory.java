package com.caseyandgary.ldapeditor.ldap;

/*
 * Author: gchen
 * Created: Friday, June 04, 2004 11:00:26 PM
 * Modified: Friday, June 04, 2004 11:00:26 PM
 */


/* 
 * @(#)CustomSocketFactory.java	1.1 02/10/30
 * 
 * Copyright 2002 Sun Microsystems, Inc. All Rights Reserved.
 * 
 * Sun grants you ("Licensee") a non-exclusive, royalty free,
 * license to use, modify and redistribute this software in source and
 * binary code form, provided that i) this copyright notice and license
 * appear on all copies of the software; and ii) Licensee does not 
 * utilize the software in a manner which is disparaging to Sun.
 *
 * This software is provided "AS IS," without a warranty of any
 * kind. ALL EXPRESS OR IMPLIED CONDITIONS, REPRESENTATIONS AND
 * WARRANTIES, INCLUDING ANY IMPLIED WARRANTY OF MERCHANTABILITY, 
 * FITNESS FOR A PARTICULAR PURPOSE OR NON-INFRINGEMENT, ARE 
 * HEREBY EXCLUDED.  SUN AND ITS LICENSORS SHALL NOT BE LIABLE 
 * FOR ANY DAMAGES SUFFERED BY LICENSEE AS A RESULT OF USING, 
 * MODIFYING OR DISTRIBUTING THE SOFTWARE OR ITS DERIVATIVES. IN 
 * NO EVENT WILL SUN OR ITS LICENSORS BE LIABLE FOR ANY LOST 
 * REVENUE, PROFIT OR DATA, OR FOR DIRECT, INDIRECT, SPECIAL,
 * CONSEQUENTIAL, INCIDENTAL OR PUNITIVE DAMAGES, HOWEVER 
 * CAUSED AND REGARDLESS OF THE THEORY OF LIABILITY, ARISING OUT 
 * OF THE USE OF OR INABILITY TO USE SOFTWARE, EVEN IF SUN HAS 
 * BEEN ADVISED OF THE POSSIBILITY OF SUCH DAMAGES.
 * 
 * This software is not designed or intended for use in on-line
 * control of aircraft, air traffic, aircraft navigation or aircraft
 * communications; or in the design, construction, operation or
 * maintenance of any nuclear facility. Licensee represents and warrants
 * that it will not use or redistribute the Software for such purposes.  
 */

import java.io.IOException;
import java.net.InetAddress;
import java.net.Socket;
import java.net.UnknownHostException;
import javax.net.ssl.SSLSocketFactory;
import javax.net.ssl.SSLSocket;
import javax.net.SocketFactory;

/*
 * A custom socket factory used to override the default socket factory.
 * It prints out debugging information before using default Socket creation
 * methods. This class is required for the UseFactory example.
 */
public class SSLCustomSocketFactory extends SSLSocketFactory {
	
	private static SSLSocketFactory delegate = null;
	public static Socket currSocket=null;
	
	public SSLCustomSocketFactory(){
		//System.out.println("Creating the delegate");
		delegate = (SSLSocketFactory)super.getDefault();
		//System.out.println("Created a secure socket factory");
	}
	
	public static SocketFactory getDefault() {
		//System.out.println("[acquiring the secure socket factory]");
	    //delegate =(SSLSocketFactory)SSLSocketFactory.getDefault();
		return new SSLCustomSocketFactory();
		//return delegate;
    }


	// Returns a socket layered over an existing socket connected to the named host, at the given port. 
    public Socket createSocket(Socket s, String host, int port, boolean autoClose)
	throws IOException{
		//System.out.println("Returning the socket with autoclose "+autoClose);
		return delegate.createSocket(s,host,port,autoClose);
	}
          
	//Returns the list of cipher suites which are enabled by default. 
	public String[] getDefaultCipherSuites() {
		return delegate.getDefaultCipherSuites();
	}
          
	//Returns the names of the cipher suites which could be enabled for use on an SSL connection. 
	public String[] getSupportedCipherSuites() {
		return delegate.getSupportedCipherSuites();
	}      
	
	
	public Socket createSocket(String host, int port)
	throws IOException, UnknownHostException {	
	//return new Socket(host, port);
	currSocket = delegate.createSocket(host,port);
	//System.out.println("Created a secure socket (method 1) on port "+port);
	return currSocket;
    }

    public Socket createSocket(String host, int port, InetAddress localHost,
	int localPort) throws IOException, UnknownHostException {

	//System.out.println("[creating a custom socket (method 2)]");
	return new Socket(host, port, localHost, localPort);
    }

    public Socket createSocket(InetAddress host, int port) throws IOException {

	//System.out.println("[creating a custom socket (method 3)]");
	return new Socket(host, port);
    }

    public Socket createSocket(InetAddress address, int port,
	InetAddress localAddress, int localPort) throws IOException {

	//System.out.println("[creating a custom socket (method 4)]");
	return new Socket(address, port, localAddress, localPort);
    }


}



