// @structa/http - Minimal HTTP Server Runtime for Structa Framework
// No TypeScript compilation needed - runs directly with Node.js

import http from 'http';

export interface HttpModuleOptions {
    port?: number;
    host?: string;
    prefix?: string;
}

const routes: any[] = [];
const middleware: any[] = [];

export function createServer(options: HttpModuleOptions = {}) {
    const port = options.port || 3000;
    const host = options.host || '0.0.0.0';
    const prefix = options.prefix || '';
    
    return {
        port,
        host,
        prefix,
        
        route(config) {
            routes.push(config);
        },
        
        use(fn) {
            middleware.push(fn);
        },
        
        listen() {
            const server = http.createServer((req, res) => {
                res.setHeader('Access-Control-Allow-Origin', '*');
                res.setHeader('Access-Control-Allow-Methods', 'GET, POST, PUT, DELETE, PATCH, OPTIONS');
                res.setHeader('Access-Control-Allow-Headers', 'Content-Type');
                
                if (req.method === 'OPTIONS') {
                    res.writeHead(204);
                    res.end();
                    return;
                }
                
                for (const mw of middleware) {
                    mw(req, res);
                }
                
                let path = req.url.split('?')[0];
                if (prefix && path.startsWith(prefix)) {
                    path = path.substring(prefix.length) || '/';
                }
                
                const method = req.method.toUpperCase();
                let matched = false;
                
                for (const route of routes) {
                    if (route.method !== method) continue;
                    
                    const pattern = route.path.replace(/:(\w+)/g, '([^/]+)');
                    const regex = new RegExp(`^${pattern}$`);
                    const match = path.match(regex);
                    
                    if (match) {
                        const params = match.slice(1);
                        const ctx = { req, res, params };
                        
                        try {
                            const result = route.handler(ctx);
                            res.writeHead(200, { 'Content-Type': 'application/json' });
                            res.end(JSON.stringify(result));
                        } catch (err) {
                            res.writeHead(500, { 'Content-Type': 'application/json' });
                            res.end(JSON.stringify({ error: err.message }));
                        }
                        matched = true;
                        break;
                    }
                }
                
                if (!matched) {
                    res.writeHead(404, { 'Content-Type': 'application/json' });
                    res.end(JSON.stringify({ error: 'Not Found', path }));
                }
            });
            
            return new Promise((resolve, reject) => {
                server.on('error', reject);
                server.listen(port, host, () => {
                    console.log(`🚀 Structa running at http://${host}:${port}${prefix}`);
                    resolve({ port, host });
                });
            });
        }
    };
}
