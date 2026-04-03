export function Query(): MethodDecorator {
  return (target: any, propertyKey: string | symbol, descriptor: PropertyDescriptor) => {
    Reflect.defineMetadata('resolver:query', propertyKey, target.constructor);
    return descriptor;
  };
}

export function Mutation(): MethodDecorator {
  return (target: any, propertyKey: string | symbol, descriptor: PropertyDescriptor) => {
    Reflect.defineMetadata('resolver:mutation', propertyKey, target.constructor);
    return descriptor;
  };
}

export function Subscription(): MethodDecorator {
  return (target: any, propertyKey: string | symbol, descriptor: PropertyDescriptor) => {
    Reflect.defineMetadata('resolver:subscription', propertyKey, target.constructor);
    return descriptor;
  };
}
