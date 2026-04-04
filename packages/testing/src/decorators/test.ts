import 'reflect-metadata';

export function Test(name?: string): MethodDecorator {
  return (target, propertyKey, descriptor: PropertyDescriptor) => {
    Reflect.defineMetadata('test:name', name || propertyKey as string, descriptor.value);
    Reflect.defineMetadata('test:type', 'test', descriptor.value);
    return descriptor;
  };
}

export function Skip(reason?: string): MethodDecorator {
  return (target, propertyKey, descriptor: PropertyDescriptor) => {
    Reflect.defineMetadata('test:skip', true, descriptor.value);
    Reflect.defineMetadata('test:skipReason', reason || 'Skipped', descriptor.value);
    return descriptor;
  };
}

export function Only(): MethodDecorator {
  return (target, propertyKey, descriptor: PropertyDescriptor) => {
    Reflect.defineMetadata('test:only', true, descriptor.value);
    return descriptor;
  };
}

export function Timeout(ms: number): MethodDecorator {
  return (target, propertyKey, descriptor: PropertyDescriptor) => {
    Reflect.defineMetadata('test:timeout', ms, descriptor.value);
    return descriptor;
  };
}
