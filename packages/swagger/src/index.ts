import 'reflect-metadata';

export function ApiTags(...tags: string[]): ClassDecorator {
  return (target: any) => {
    Reflect.defineMetadata('swagger:tags', tags, target);
    return target;
  };
}

export function ApiOperation(options: { summary?: string; description?: string }): MethodDecorator {
  return (target, propertyKey, descriptor: PropertyDescriptor) => {
    Reflect.defineMetadata('swagger:operation', options, descriptor.value);
    return descriptor;
  };
}

export function ApiResponse(
  status: number,
  options: { description?: string; type?: any; schema?: any } = {}
): MethodDecorator {
  return (target, propertyKey, descriptor: PropertyDescriptor) => {
    const responses = Reflect.getMetadata('swagger:responses', descriptor.value) || [];
    responses.push({ status, ...options });
    Reflect.defineMetadata('swagger:responses', responses, descriptor.value);
    return descriptor;
  };
}

export function ApiBearerAuth(name: string = 'Bearer'): ClassDecorator & MethodDecorator {
  return (target: any, propertyKey?: string | symbol, descriptor?: TypedPropertyDescriptor<any>) => {
    if (descriptor) {
      const security = Reflect.getMetadata('swagger:security', descriptor.value) || [];
      security.push({ bearerAuth: [] });
      Reflect.defineMetadata('swagger:security', security, descriptor.value);
    } else {
      Reflect.defineMetadata('swagger:bearerAuth', name, target);
    }
    return descriptor || target;
  };
}

export function ApiProperty(options?: {
  description?: string;
  example?: any;
  required?: boolean;
  type?: any;
  format?: string;
}): PropertyDecorator & ParameterDecorator {
  return (target: any, propertyKey?: string | symbol) => {
    if (propertyKey) {
      const properties = Reflect.getMetadata('swagger:properties', target) || {};
      properties[propertyKey as string] = options || {};
      Reflect.defineMetadata('swagger:properties', properties, target);
    }
    return target;
  };
}

export function ApiQuery(options: {
  name: string;
  description?: string;
  required?: boolean;
  type?: string;
}): MethodDecorator {
  return (target, propertyKey, descriptor: PropertyDescriptor) => {
    const queries = Reflect.getMetadata('swagger:queries', descriptor.value) || [];
    queries.push(options);
    Reflect.defineMetadata('swagger:queries', queries, descriptor.value);
    return descriptor;
  };
}

export function ApiHeader(options: {
  name: string;
  description?: string;
  required?: boolean;
}): ClassDecorator {
  return (target: any) => {
    const headers = Reflect.getMetadata('swagger:headers', target) || [];
    headers.push(options);
    Reflect.defineMetadata('swagger:headers', headers, target);
    return target;
  };
}

export interface SwaggerModuleOptions {
  title?: string;
  description?: string;
  version?: string;
  path?: string;
  tags?: string[];
}

export function createSwaggerModule(options?: SwaggerModuleOptions): any {
  return {
    module: 'SwaggerModule',
    options: {
      title: options?.title || 'Structa API',
      description: options?.description || 'API Documentation',
      version: options?.version || '1.0.0',
      path: options?.path || '/api-docs',
      tags: options?.tags || []
    }
  };
}

export const SwaggerModule = {
  create: createSwaggerModule,
  forRoot: createSwaggerModule,
  forRootAsync: (options: any) => createSwaggerModule(options)
};
