/* eslint-disable @typescript-eslint/no-explicit-any */
import * as http from 'http';
import { getControllerMetadata, getRoutesMetadata } from './decorators/http.js';
import 'reflect-metadata';

const registeredControllers: any[] = [];

export function registerController(controllerClass: any): void {
  console.log('[DEBUG] registerController called:', controllerClass.name);
  registeredControllers.push(controllerClass);
}

export function createHttpServer(options?: any): any {
  let server: http.Server | null = null;
  const middleware: any[] = [];
  let errorHandler: (error: any, ctx: any) => void = () => {};
  const prefix = options?.prefix || '';

  const matchRoute = (req: http.IncomingMessage, res: http.ServerResponse): boolean => {
    const reqMethod = req.method || 'GET';
    let url = req.url || '/';
    console.log(`[DEBUG] Request: ${reqMethod} ${url}`);
    if (prefix && url.startsWith(prefix)) {
      url = url.substring(prefix.length) || '/';
      console.log(`[DEBUG] After prefix strip: ${url}`);
    }
    const method = reqMethod;

    console.log(`[DEBUG] Registered controllers: ${registeredControllers.length}`);
    for (const controllerClass of registeredControllers) {
      const controllerMeta = getControllerMetadata(controllerClass);
      console.log(`[DEBUG] Controller metadata:`, controllerMeta);
      
      if (!controllerMeta) continue;

      const routes = getRoutesMetadata(controllerClass);
      console.log(`[DEBUG] Routes for controller:`, routes);
      
      for (const route of routes) {
        const basePath = controllerMeta.path.endsWith('/') ? controllerMeta.path.slice(0, -1) : controllerMeta.path;
        const routePath = route.path.startsWith('/') ? route.path : '/' + route.path;
        const fullPath = basePath + routePath;
        console.log(`[DEBUG] Checking route: ${route.method} ${fullPath}`);
        
        const routePattern = fullPath
          .replace(/:(\w+)/g, '([^/]+)')
          .replace(/\//g, '\\/');
        
        console.log(`[DEBUG] Route pattern: ${routePattern}`);
        const regex = new RegExp(`^${routePattern}$`);
        const match = url.match(regex);
        console.log(`[DEBUG] URL match: ${match}`);

        if (match && route.method === method) {
          const instance = new controllerClass();
          const handler = instance[route.propertyKey as string];
          
          if (typeof handler === 'function') {
            try {
              const result = handler.call(instance, ...match.slice(1));
              res.writeHead(200, { 'Content-Type': 'application/json' });
              res.end(JSON.stringify(result));
              return true;
            } catch (err: any) {
              errorHandler(err, { req, res });
              return true;
            }
          }
        }
      }
    }
    return false;
  };

  return {
    port: options?.port || 3000,
    host: options?.host || '0.0.0.0',
    cors: options?.cors || false,
    bodyParser: options?.bodyParser !== false,
    compression: options?.compression || false,

    async listen(port?: number, host?: string): Promise<any> {
      const actualPort = port || this.port;
      const actualHost = host || this.host;

      server = http.createServer((req, res) => {
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

        if (!matchRoute(req, res)) {
          res.writeHead(404, { 'Content-Type': 'application/json' });
          res.end(JSON.stringify({ error: 'Not Found' }));
        }
      });

      return new Promise((resolve, reject) => {
        server!.on('error', reject);
        server!.listen(actualPort, actualHost, () => {
          console.log(`🚀 Structa Server running at http://${actualHost}:${actualPort}`);
          resolve({ port: actualPort, host: actualHost });
        });
      });
    },

    async close(): Promise<void> {
      return new Promise((resolve) => {
        if (server) {
          server.close(() => resolve());
        } else {
          resolve();
        }
      });
    },

    use(middlewareFn: any): void {
      middleware.push(middlewareFn);
    },

    setErrorHandler(handler: (error: any, ctx: any) => void): void {
      errorHandler = handler;
    }
  };
}
