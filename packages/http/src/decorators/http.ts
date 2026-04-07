import 'reflect-metadata';

export interface ControllerMetadata {
  path: string;
  target: any;
}

export interface RouteMetadata {
  method: string;
  path: string;
  target: any;
  propertyKey: string | symbol;
}

const CONTROLLER_METADATA_KEY = Symbol('http:controller');
const ROUTES_METADATA_KEY = Symbol('http:routes');

export function Controller(path: string): any {
  return function(target: any) {
    Reflect.defineMetadata(CONTROLLER_METADATA_KEY, { path }, target);
    return target;
  };
}

export function Injectable(): ClassDecorator {
  return (target: any) => {
    return target;
  };
}

export function Get(path: string): MethodDecorator {
  return (target: any, propertyKey: string | symbol) => {
    const routes: RouteMetadata[] = Reflect.getOwnMetadata(ROUTES_METADATA_KEY, target) || [];
    routes.push({ method: 'GET', path, target, propertyKey });
    Reflect.defineMetadata(ROUTES_METADATA_KEY, routes, target);
  };
}

export function Post(path: string): MethodDecorator {
  return (target: any, propertyKey: string | symbol) => {
    const routes: RouteMetadata[] = Reflect.getOwnMetadata(ROUTES_METADATA_KEY, target) || [];
    routes.push({ method: 'POST', path, target, propertyKey });
    Reflect.defineMetadata(ROUTES_METADATA_KEY, routes, target);
  };
}

export function Put(path: string): MethodDecorator {
  return (target: any, propertyKey: string | symbol) => {
    const routes: RouteMetadata[] = Reflect.getOwnMetadata(ROUTES_METADATA_KEY, target) || [];
    routes.push({ method: 'PUT', path, target, propertyKey });
    Reflect.defineMetadata(ROUTES_METADATA_KEY, routes, target);
  };
}

export function Delete(path: string): MethodDecorator {
  return (target: any, propertyKey: string | symbol) => {
    const routes: RouteMetadata[] = Reflect.getOwnMetadata(ROUTES_METADATA_KEY, target) || [];
    routes.push({ method: 'DELETE', path, target, propertyKey });
    Reflect.defineMetadata(ROUTES_METADATA_KEY, routes, target);
  };
}

export function Patch(path: string): MethodDecorator {
  return (target: any, propertyKey: string | symbol) => {
    const routes: RouteMetadata[] = Reflect.getOwnMetadata(ROUTES_METADATA_KEY, target) || [];
    routes.push({ method: 'PATCH', path, target, propertyKey });
    Reflect.defineMetadata(ROUTES_METADATA_KEY, routes, target);
  };
}

export function addRoute(target: any, method: string, path: string, propertyKey: string | symbol): void {
  const routes: RouteMetadata[] = Reflect.getOwnMetadata(ROUTES_METADATA_KEY, target) || [];
  routes.push({ method, path, target, propertyKey });
  Reflect.defineMetadata(ROUTES_METADATA_KEY, routes, target);
}

export function getControllerMetadata(target: any): ControllerMetadata | undefined {
  return Reflect.getMetadata(CONTROLLER_METADATA_KEY, target);
}

export function getRoutesMetadata(target: any): RouteMetadata[] {
  return Reflect.getOwnMetadata(ROUTES_METADATA_KEY, target) || [];
}
