import { containerInstance } from '../container.js';

export function Controller(basePath: string = '/'): ClassDecorator {
  return (target: any) => {
    Reflect.defineMetadata('controller:path', basePath, target);
    Reflect.defineMetadata('controller', true, target);
    containerInstance.register(target, target);
    return target;
  };
}
