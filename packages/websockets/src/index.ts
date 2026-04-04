import 'reflect-metadata';

export function Gateway(namespace?: string): ClassDecorator {
  return (target: any) => {
    Reflect.defineMetadata('websocket:gateway', true, target);
    Reflect.defineMetadata('websocket:namespace', namespace || '/', target);
    return target;
  };
}

export function SubscribeMessage(event: string): MethodDecorator {
  return (target, propertyKey, descriptor: PropertyDescriptor) => {
    Reflect.defineMetadata('websocket:subscribe', event, descriptor.value);
    return descriptor;
  };
}

export function WebSocketServer(): PropertyDecorator {
  return (target, propertyKey) => {
    Reflect.defineMetadata('websocket:server', true, target, propertyKey as string);
    return target;
  };
}

export function OnGatewayConnection(): MethodDecorator {
  return (target, propertyKey, descriptor: PropertyDescriptor) => {
    Reflect.defineMetadata('websocket:onConnection', true, descriptor.value);
    return descriptor;
  };
}

export function OnGatewayDisconnect(): MethodDecorator {
  return (target, propertyKey, descriptor: PropertyDescriptor) => {
    Reflect.defineMetadata('websocket:onDisconnect', true, descriptor.value);
    return descriptor;
  };
}

export function WsExceptionFilter(): ClassDecorator {
  return (target: any) => {
    Reflect.defineMetadata('websocket:exceptionFilter', true, target);
    return target;
  };
}

export interface WebSocketGatewayOptions {
  namespace?: string;
  cors?: {
    origin?: string | string[];
    credentials?: boolean;
  };
}

export function createWebSocketModule(options?: WebSocketGatewayOptions): any {
  return {
    module: 'WebSocketModule',
    options: {
      namespace: options?.namespace || '/',
      cors: options?.cors
    }
  };
}

export const WebSocketModule = {
  forRoot: createWebSocketModule,
  forRootAsync: (options: any) => createWebSocketModule(options)
};
