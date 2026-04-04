import 'reflect-metadata';
export function Injectable() {
    return (target) => {
        Reflect.defineMetadata('injectable', true, target);
        return target;
    };
}
export function Inject(token) {
    return (target, propertyKey) => {
        const existingTokens = Reflect.getMetadata('injectTokens', target) || [];
        existingTokens.push({ propertyKey, token });
        Reflect.defineMetadata('injectTokens', existingTokens, target);
    };
}
export function Optional() {
    return (target, propertyKey) => {
        const existingOptional = Reflect.getMetadata('optionalTokens', target) || [];
        existingOptional.push(propertyKey);
        Reflect.defineMetadata('optionalTokens', existingOptional, target);
    };
}
//# sourceMappingURL=injectable.js.map