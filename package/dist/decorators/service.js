export function Service() {
    return (target) => {
        Reflect.defineMetadata('service', true, target);
        return target;
    };
}
//# sourceMappingURL=service.js.map