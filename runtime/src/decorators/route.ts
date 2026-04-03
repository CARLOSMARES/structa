import 'reflect-metadata';

export type HttpMethod = 'GET' | 'POST' | 'PUT' | 'PATCH' | 'DELETE' | 'HEAD' | 'OPTIONS';

function createRouteDecorator(method: HttpMethod) {
  return (path: string = '/'): MethodDecorator => {
    return (target: any, propertyKey: string | symbol, descriptor: PropertyDescriptor) => {
      const routes: any[] = Reflect.getMetadata('routes', target.constructor) || [];
      routes.push({ method, path, handler: propertyKey });
      Reflect.defineMetadata('routes', routes, target.constructor);
      return descriptor;
    };
  };
}

export const Get = createRouteDecorator('GET');
export const Post = createRouteDecorator('POST');
export const Put = createRouteDecorator('PUT');
export const Patch = createRouteDecorator('PATCH');
export const Delete = createRouteDecorator('DELETE');
export const Head = createRouteDecorator('HEAD');
export const Options = createRouteDecorator('OPTIONS');
