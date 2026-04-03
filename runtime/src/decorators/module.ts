export function Module(options: { imports?: any[]; controllers?: any[]; services?: any[] } = {}): ClassDecorator {
  return (target: any) => {
    Reflect.defineMetadata('module', true, target);
    Reflect.defineMetadata('module:imports', options.imports || [], target);
    Reflect.defineMetadata('module:controllers', options.controllers || [], target);
    Reflect.defineMetadata('module:services', options.services || [], target);
    return target;
  };
}
