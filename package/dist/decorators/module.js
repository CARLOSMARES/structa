export function Module(options = {}) {
    return (target) => {
        Reflect.defineMetadata('module', true, target);
        Reflect.defineMetadata('module:imports', options.imports || [], target);
        Reflect.defineMetadata('module:controllers', options.controllers || [], target);
        Reflect.defineMetadata('module:services', options.services || [], target);
        return target;
    };
}
//# sourceMappingURL=module.js.map