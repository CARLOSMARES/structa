export function Guard() {
    return (target) => {
        Reflect.defineMetadata('guard', true, target);
        return target;
    };
}
//# sourceMappingURL=guard.js.map