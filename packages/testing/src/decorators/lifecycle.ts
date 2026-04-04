import 'reflect-metadata';

export function BeforeAll(): MethodDecorator {
  return (target, propertyKey, descriptor: PropertyDescriptor) => {
    Reflect.defineMetadata('test:beforeAll', true, descriptor.value);
    return descriptor;
  };
}

export function AfterAll(): MethodDecorator {
  return (target, propertyKey, descriptor: PropertyDescriptor) => {
    Reflect.defineMetadata('test:afterAll', true, descriptor.value);
    return descriptor;
  };
}

export function BeforeEach(): MethodDecorator {
  return (target, propertyKey, descriptor: PropertyDescriptor) => {
    Reflect.defineMetadata('test:beforeEach', true, descriptor.value);
    return descriptor;
  };
}

export function AfterEach(): MethodDecorator {
  return (target, propertyKey, descriptor: PropertyDescriptor) => {
    Reflect.defineMetadata('test:afterEach', true, descriptor.value);
    return descriptor;
  };
}
