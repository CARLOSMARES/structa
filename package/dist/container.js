const globalContainer = new Map();
class StructaContainer {
    providers = new Map();
    instances = new Map();
    register(token, provider) {
        if (typeof provider === 'function') {
            this.providers.set(token, { provide: token, useClass: provider });
        }
        else if (typeof provider === 'object' && provider !== null) {
            this.providers.set(token, provider);
        }
        else {
            this.providers.set(token, { provide: token, useValue: provider });
        }
        return this;
    }
    resolve(token) {
        const provider = this.providers.get(token);
        if (!provider) {
            throw new Error(`No provider found for token: ${token}`);
        }
        if (provider.useValue !== undefined) {
            return provider.useValue;
        }
        if (provider.useFactory) {
            return provider.useFactory();
        }
        if (provider.useClass) {
            const cached = this.instances.get(token);
            if (cached)
                return cached;
            const InstanceClass = provider.useClass;
            const instance = new InstanceClass();
            this.instances.set(token, instance);
            return instance;
        }
        throw new Error(`Invalid provider configuration for token: ${token}`);
    }
    get(token) {
        try {
            return this.resolve(token);
        }
        catch {
            return undefined;
        }
    }
    has(token) {
        return this.providers.has(token);
    }
}
export const containerInstance = new StructaContainer();
export function createContainer() {
    return new StructaContainer();
}
export function getContainer() {
    return containerInstance;
}
export function register(token, provider) {
    containerInstance.register(token, provider);
}
export function resolve(token) {
    return containerInstance.resolve(token);
}
export function injectable(token) {
    return (target) => {
        if (token) {
            containerInstance.register(token, target);
        }
        else {
            containerInstance.register(target, target);
        }
        return target;
    };
}
//# sourceMappingURL=container.js.map