package com.example.proxy;

import com.github.monkeywie.proxyee.server.HttpProxyServer;

public class ProxyApplication {

	public static void main(String[] args) {		
		new HttpProxyServer().start(9999);
	}

}
