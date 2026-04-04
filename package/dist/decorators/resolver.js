export function Query() {
    return (target, propertyKey, descriptor) => {
        Reflect.defineMetadata('resolver:query', propertyKey, target.constructor);
        return descriptor;
    };
}
export function Mutation() {
    return (target, propertyKey, descriptor) => {
        Reflect.defineMetadata('resolver:mutation', propertyKey, target.constructor);
        return descriptor;
    };
}
export function Subscription() {
    return (target, propertyKey, descriptor) => {
        Reflect.defineMetadata('resolver:subscription', propertyKey, target.constructor);
        return descriptor;
    };
}
//# sourceMappingURL=resolver.js.map