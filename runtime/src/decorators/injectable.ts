import 'reflect-metadata';

export function Injectable(): ClassDecorator {
  return (target: any) => {
    Reflect.defineMetadata('injectable', true, target);
    return target;
  };
}

export function Inject(token: any): PropertyDecorator {
  return (target: any, propertyKey: string | symbol) => {
    const existingTokens: any[] = Reflect.getMetadata('injectTokens', target) || [];
    existingTokens.push({ propertyKey, token });
    Reflect.defineMetadata('injectTokens', existingTokens, target);
  };
}

export function Optional(): PropertyDecorator {
  return (target: any, propertyKey: string | symbol) => {
    const existingOptional: any[] = Reflect.getMetadata('optionalTokens', target) || [];
    existingOptional.push(propertyKey);
    Reflect.defineMetadata('optionalTokens', existingOptional, target);
  };
}
