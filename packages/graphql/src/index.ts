import 'reflect-metadata';

export function Resolver(): ClassDecorator {
  return (target: any) => {
    Reflect.defineMetadata('graphql:resolver', true, target);
    return target;
  };
}

export function Query(returnTypeFunc: () => any, options?: { nullable?: boolean }): MethodDecorator {
  return (target, propertyKey, descriptor: PropertyDescriptor) => {
    Reflect.defineMetadata('graphql:query', { returnType: returnTypeFunc, options }, descriptor.value);
    return descriptor;
  };
}

export function Mutation(returnTypeFunc: () => any): MethodDecorator {
  return (target, propertyKey, descriptor: PropertyDescriptor) => {
    Reflect.defineMetadata('graphql:mutation', returnTypeFunc, descriptor.value);
    return descriptor;
  };
}

export function Subscription(returnTypeFunc: () => any): MethodDecorator {
  return (target, propertyKey, descriptor: PropertyDescriptor) => {
    Reflect.defineMetadata('graphql:subscription', returnTypeFunc, descriptor.value);
    return descriptor;
  };
}

export function Field(options?: { nullable?: boolean; type?: () => any }): PropertyDecorator & MethodDecorator {
  return (target: any, propertyKey?: string | symbol, descriptor?: TypedPropertyDescriptor<any>) => {
    if (descriptor) {
      Reflect.defineMetadata('graphql:field', options, descriptor.value);
    } else if (propertyKey) {
      Reflect.defineMetadata('graphql:field', options, target, propertyKey);
    }
    return descriptor || target;
  };
}

export function Args(): ParameterDecorator {
  return (target, propertyKey, parameterIndex) => {};
}

export function ObjectType(): ClassDecorator {
  return (target: any) => {
    Reflect.defineMetadata('graphql:objectType', true, target);
    return target;
  };
}

export function InputType(): ClassDecorator {
  return (target: any) => {
    Reflect.defineMetadata('graphql:inputType', true, target);
    return target;
  };
}

export interface GraphQLModuleOptions {
  path?: string;
  autoSchema?: boolean;
  playground?: boolean;
  schemaFile?: string;
}

export function createGraphQLModule(options?: GraphQLModuleOptions): any {
  return {
    module: 'GraphQLModule',
    options: {
      path: options?.path || '/graphql',
      autoSchema: options?.autoSchema !== false,
      playground: options?.playground !== false,
      schemaFile: options?.schemaFile
    }
  };
}

export const GraphQLModule = {
  forRoot: createGraphQLModule,
  forRootAsync: (options: any) => createGraphQLModule(options)
};
