export function Dto() {
    return (target) => {
        Reflect.defineMetadata('dto', true, target);
        return target;
    };
}
//# sourceMappingURL=dto.js.map