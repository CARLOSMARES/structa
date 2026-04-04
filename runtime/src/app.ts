import 'reflect-metadata';
import { createContext, Context, Request, Response, NextFunction } from './context.js';
import { getContainer } from './container.js';

export interface StructaOptions {
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
}

class StructaAppImpl {
  private controllers: Map<any, ControllerMetadata> = new Map();
  private services: Set<any> = new Set();
  private middleware: any[] = [];
  private port: number = 3000;
  private host: string = '0.0.0.0';

  constructor() {
    this.discoverComponents();
  }

  private discoverComponents() {
    const container = getContainer();
    const providers = (container as any).providers;
    
    if (providers) {
      for (const [token, provider] of providers) {
        if (typeof token === 'function') {
          const isController = Reflect.getMetadata('controller', token);
          const isService = Reflect.getMetadata('service', token);
          const isMiddleware = Reflect.getMetadata('middleware', token);

          if (isController) {
            const path = Reflect.getMetadata('controller:path', token) || '/';
            const routes: RouteHandler[] = Reflect.getMetadata('routes', token) || [];
            this.controllers.set(token, { path, routes });
          } else if (isService) {
            this.services.add(token);
          } else if (isMiddleware) {
            this.middleware.push(token);
          }
        }
      }
    }
  }

  listen(port?: number, host?: string): Promise<void> {
    this.port = port || this.port;
    this.host = host || this.host;
    
    return new Promise((resolve) => {
      console.log(`Structa app listening on http://${this.host}:${this.port}`);
      resolve();
    });
  }

  getControllers(): Map<any, ControllerMetadata> {
    return this.controllers;
  }

  getServices(): Set<any> {
    return this.services;
  }
}

let appInstance: StructaAppImpl | null = null;

export function createApp(): StructaAppImpl {
  appInstance = new StructaAppImpl();
  return appInstance;
}

export class StructaApp {
  private impl: StructaAppImpl;

  constructor() {
    this.impl = createApp();
  }

  listen(port?: number, host?: string): Promise<void> {
    return this.impl.listen(port, host);
  }

  getControllers(): Map<any, ControllerMetadata> {
    return this.impl.getControllers();
  }

  getServices(): Set<any> {
    return this.impl.getServices();
  }
}
