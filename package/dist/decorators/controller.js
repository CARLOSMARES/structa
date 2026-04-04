import { containerInstance } from '../container';
export function Controller(basePath = '/') {
    return (target) => {
        Reflect.defineMetadata('controller:path', basePath, target);
        Reflect.defineMetadata('controller', true, target);
        containerInstance.register(target, target);
        return target;
    };
}
//# sourceMappingURL=controller.js.map