export function UseMiddleware(...middleware: any[]): MethodDecorator & ClassDecorator {
  return (target: any, propertyKey?: string | symbol, descriptor?: TypedPropertyDescriptor<any>) => {
    if (descriptor) {
      const middlewares = Reflect.getMetadata('http:middlewares', descriptor.value) || [];
      Reflect.defineMetadata('http:middlewares', [...middlewares, ...middleware], descriptor.value);
    } else {
      const middlewares = Reflect.getMetadata('class:middlewares', target) || [];
      Reflect.defineMetadata('class:middlewares', [...middlewares, ...middleware], target);
    }
    return descriptor || target;
  };
}

export function UseGuards(...guards: any[]): MethodDecorator & ClassDecorator {
  return (target: any, propertyKey?: string | symbol, descriptor?: TypedPropertyDescriptor<any>) => {
    if (descriptor) {
      const routeGuards = Reflect.getMetadata('http:guards', descriptor.value) || [];
      Reflect.defineMetadata('http:guards', [...routeGuards, ...guards], descriptor.value);
    } else {
      const classGuards = Reflect.getMetadata('class:guards', target) || [];
      Reflect.defineMetadata('class:guards', [...classGuards, ...guards], target);
    }
    return descriptor || target;
  };
}
