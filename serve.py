#!/usr/bin/env python3
"""
Simple HTTP server with proper WASM MIME type support
"""
import http.server
import socketserver
import os

PORT = 8000

class WasmHandler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        # Add CORS headers for local development
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type')
        super().end_headers()
    
    def guess_type(self, path):
        mime_type, encoding = super().guess_type(path)
        # Fix MIME type for WASM files
        if path.endswith('.wasm'):
            return 'application/wasm', encoding
        # Fix MIME type for JavaScript modules
        if path.endswith('.js') or path.endswith('.mjs'):
            return 'text/javascript', encoding
        return mime_type, encoding

    def do_OPTIONS(self):
        self.send_response(200)
        self.end_headers()

if __name__ == '__main__':
    # Change to the script directory
    os.chdir(os.path.dirname(os.path.abspath(__file__)))
    
    with socketserver.TCPServer(("", PORT), WasmHandler) as httpd:
        print(f"🚀 Server running at http://localhost:{PORT}/")
        print(f"📂 Serving directory: {os.getcwd()}")
        print(f"🌐 Open http://localhost:{PORT}/examples/index.html")
        print(f"⏹️  Press Ctrl+C to stop")
        try:
            httpd.serve_forever()
        except KeyboardInterrupt:
            print("\n👋 Server stopped")

