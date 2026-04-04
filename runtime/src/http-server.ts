import * as http from 'http';
import { getContainer } from './container';

export interface HttpServerOptions {
  port?: number;
  host?: string;
}

export interface RouteHandler {
  method: string;
  path: string;
  handler: string;
  target: any;
}

export interface ControllerMetadata {
  path: string;
  routes: RouteHandler[];
  instance: any;
}

export class StructaHttpServer {
  private server: http.Server | null = null;
  private port: number = 3000;
  private host: string = '0.0.0.0';
  private controllers: Map<string, ControllerMetadata> = new Map();

  constructor() {
    this.discoverControllers();
  }

  private discoverControllers() {
    const container = getContainer();
    const providers = (container as any).providers;
    
    if (providers) {
      for (const [token, provider] of providers) {
        if (typeof token === 'function') {
          const isController = Reflect.getMetadata('controller', token);
          
          if (isController) {
            const path = Reflect.getMetadata('controller:path', token) || '/';
            const routes: RouteHandler[] = Reflect.getMetadata('routes', token) || [];
            const instance = typeof provider === 'function' ? new provider() : provider;
            
            this.controllers.set(token.name || 'Anonymous', { path, routes, instance });
            console.log(`📦 Controller registered: ${token.name} (${path})`);
            routes.forEach((route: RouteHandler) => {
              console.log(`   ${route.method.toUpperCase()} ${path}${route.path}`);
            });
          }
        }
      }
    }
  }

  private handleRequest(req: http.IncomingMessage, res: http.ServerResponse) {
    const url = req.url || '/';
    const method = req.method || 'GET';
    
    res.setHeader('Content-Type', 'application/json');
    res.setHeader('Access-Control-Allow-Origin', '*');
    res.setHeader('Access-Control-Allow-Methods', 'GET, POST, PUT, DELETE, PATCH, OPTIONS');
    res.setHeader('Access-Control-Allow-Headers', 'Content-Type');
    
    if (method === 'OPTIONS') {
      res.writeHead(204);
      res.end();
      return;
    }

    for (const [, controller] of this.controllers) {
      for (const route of controller.routes) {
        if (route.method.toUpperCase() !== method) continue;
        
        const fullPath = this.normalizePath(controller.path, route.path);
        const match = this.matchPath(url, fullPath);
        
        if (match) {
          const params = match.params || {};
          const query: Record<string, string> = {};
          const urlObj = new URL(url, `http://${req.headers.host}`);
          urlObj.searchParams.forEach((value, key) => {
            query[key] = value;
          });

          let body: any = {};
          if (['POST', 'PUT', 'PATCH'].includes(method)) {
            const chunks: Buffer[] = [];
            req.on('data', (chunk: Buffer) => chunks.push(chunk));
            req.on('end', async () => {
              try {
                body = JSON.parse(Buffer.concat(chunks).toString() || '{}');
              } catch {
                body = {};
              }
              await this.executeHandler(res, controller.instance, route.handler, { params, query, body });
            });
          } else {
            this.executeHandler(res, controller.instance, route.handler, { params, query, body });
          }
          return;
        }
      }
    }

    res.writeHead(404, { 'Content-Type': 'application/json' });
    res.end(JSON.stringify({ statusCode: 404, message: 'Not Found' }));
  }

  private normalizePath(...parts: string[]): string {
    return '/' + parts.map(p => p.replace(/^\/|\/$/g, '')).filter(Boolean).join('/');
  }

  private matchPath(url: string, pattern: string): { params: Record<string, string> } | null {
    const urlParts = url.split('?')[0].split('/').filter(Boolean);
    const patternParts = pattern.split('/').filter(Boolean);

    if (urlParts.length !== patternParts.length) return null;

    const params: Record<string, string> = {};
    
    for (let i = 0; i < patternParts.length; i++) {
      const patternPart = patternParts[i];
      const urlPart = urlParts[i];

      if (patternPart.startsWith(':')) {
        params[patternPart.slice(1)] = urlPart;
      } else if (patternPart !== urlPart) {
        return null;
      }
    }

    return { params };
  }

  private async executeHandler(
    res: http.ServerResponse,
    instance: any,
    handlerName: string,
    context: { params: Record<string, string>; query: Record<string, string>; body: any }
  ) {
    try {
      const handler = instance[handlerName];
      if (typeof handler !== 'function') {
        throw new Error(`Handler ${handlerName} is not a function`);
      }

      const result = await handler.call(instance, context);

      if (result === undefined) {
        res.writeHead(204);
        res.end();
        return;
      }

      if (typeof result === 'object') {
        res.writeHead(200);
        res.end(JSON.stringify(result));
      } else {
        res.writeHead(200);
        res.end(String(result));
      }
    } catch (error) {
      console.error(`Error in handler:`, error);
      res.writeHead(500, { 'Content-Type': 'application/json' });
      res.end(JSON.stringify({ statusCode: 500, message: 'Internal Server Error' }));
    }
  }

  listen(port?: number, host?: string): Promise<void> {
    this.port = port || this.port;
    this.host = host || this.host;

    return new Promise((resolve, reject) => {
      this.server = http.createServer((req, res) => this.handleRequest(req, res));
      
      this.server.on('error', (err) => {
        console.error('Server error:', err);
        reject(err);
      });

      this.server.listen(this.port, this.host, () => {
        console.log(`🚀 Structa Server running at http://${this.host}:${this.port}`);
        resolve();
      });
    });
  }

  async close(): Promise<void> {
    return new Promise((resolve) => {
      if (this.server) {
        this.server.close(() => {
          console.log('Server closed');
          resolve();
        });
      } else {
        resolve();
      }
    });
  }
}

let httpServerInstance: StructaHttpServer | null = null;

export function createHttpServer(): StructaHttpServer {
  httpServerInstance = new StructaHttpServer();
  return httpServerInstance;
}

export function getHttpServer(): StructaHttpServer | null {
  return httpServerInstance;
}
