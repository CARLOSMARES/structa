import http from 'http';

const _routes = [];
const _middleware = [];
const _services = {};

class Container {
    static services = {};
    
    static set(name, instance) {
        this.services[name] = instance;
    }
    
    static get(name) {
        if (!this.services[name]) {
            this.services[name] = this.resolve(name);
        }
        return this.services[name];
    }
    
    static resolve(name) {
        const parts = name.replace(/Controller$/, '').replace(/Service$/, '').replace(/Repository$/, '');
        const className = name.charAt(0).toUpperCase() + name.slice(1);
        try {
            return eval('new ' + className + '()');
        } catch (e) {
            return {};
        }
    }
    
    static clear() {
        this.services = {};
    }
}

const _container = Container;

export function createServer(options = {}) {
    const port = options.port || process.env.PORT || 3000;
    const host = options.host || '0.0.0.0';
    
    return {
        port, host,
        route(config) { _routes.push(config); },
        use(fn) { _middleware.push(fn); },
        listen() {
            http.createServer(async (req, res) => {
                res.setHeader('Access-Control-Allow-Origin', '*');
                res.setHeader('Access-Control-Allow-Methods', 'GET, POST, PUT, DELETE, PATCH, OPTIONS');
                res.setHeader('Access-Control-Allow-Headers', 'Content-Type, Authorization');
                
                if (req.method === 'OPTIONS') {
                    res.writeHead(204);
                    res.end();
                    return;
                }
                
                let body = {};
                if (['POST', 'PUT', 'PATCH'].includes(req.method)) {
                    try {
                        const bodyText = await new Promise((resolve, reject) => {
                            let data = '';
                            req.on('data', chunk => data += chunk);
                            req.on('end', () => resolve(data));
                            req.on('error', reject);
                        });
                        if (bodyText) {
                            body = JSON.parse(bodyText);
                        }
                    } catch (e) {
                        body = {};
                    }
                }
                
                let path = (req.url || '').split('?')[0];
                path = path.replace(/\/+$/, '') || '/';
                
                let middlewareIndex = 0;
                const runNext = () => {
                    if (middlewareIndex < _middleware.length) {
                        const mw = _middleware[middlewareIndex++];
                        try {
                            mw.handle({ req, res, body, params: {} }, res, runNext);
                        } catch (e) {
                            res.writeHead(500, { 'Content-Type': 'application/json' });
                            res.end(JSON.stringify({ error: e.message }));
                        }
                    } else {
                        handleRoute(req, res, path, body);
                    }
                };
                runNext();
            }).listen(port, host, () => {
                console.log('\x1b[32mStructa running at http://' + host + ':' + port + '\x1b[0m');
            });
        }
    };
}

function handleRoute(req, res, path, body) {
    const method = req.method.toUpperCase();
    
    for (const route of _routes) {
        if (route.method !== method && route.method !== 'ALL') continue;
        
        let routePath = route.path.replace(/\/+$/, '') || '/';
        const pattern = routePath.replace(/:(\w+)/g, '([^/]+)');
        const match = path.match(new RegExp('^' + pattern + '$'));
        
        if (match) {
            const params = {};
            const paramNames = routePath.match(/:(\w+)/g) || [];
            paramNames.forEach((p, i) => {
                params[p.replace(':', '')] = match[i + 1];
            });
            
            try {
                const result = route.handler({ req, res, body, params });
                if (!res.writableEnded) {
                    if (result instanceof Promise) {
                        result.then(data => {
                            if (!res.writableEnded) {
                                res.writeHead(200, { 'Content-Type': 'application/json' });
                                res.end(JSON.stringify(data));
                            }
                        }).catch(err => {
                            if (!res.writableEnded) {
                                res.writeHead(500, { 'Content-Type': 'application/json' });
                                res.end(JSON.stringify({ error: err.message }));
                            }
                        });
                    } else {
                        res.writeHead(200, { 'Content-Type': 'application/json' });
                        res.end(JSON.stringify(result));
                    }
                }
            } catch (err) {
                if (!res.writableEnded) {
                    res.writeHead(500, { 'Content-Type': 'application/json' });
                    res.end(JSON.stringify({ error: err.message }));
                }
            }
            return;
        }
    }
    
    if (!res.writableEnded) {
        res.writeHead(404, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify({ error: 'Not Found', path }));
    }
}

export const server = createServer({ port: process.env.PORT || 3000 });
export { _routes as routes, _middleware as middleware, Container };
