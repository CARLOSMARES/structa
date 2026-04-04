export function Middleware() {
    return (target) => {
        Reflect.defineMetadata('middleware', true, target);
        return target;
    };
}
//# sourceMappingURL=middleware.js.map