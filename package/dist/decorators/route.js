import 'reflect-metadata';
function createRouteDecorator(method) {
    return (path = '/') => {
        return (target, propertyKey, descriptor) => {
            const routes = Reflect.getMetadata('routes', target.constructor) || [];
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
//# sourceMappingURL=route.js.map