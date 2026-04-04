import 'reflect-metadata';
import { getContainer } from './container';
class StructaAppImpl {
    controllers = new Map();
    services = new Set();
    middleware = [];
    port = 3000;
    host = '0.0.0.0';
    constructor() {
        this.discoverComponents();
    }
    discoverComponents() {
        const container = getContainer();
        const providers = container.providers;
        if (providers) {
            for (const [token, provider] of providers) {
                if (typeof token === 'function') {
                    const isController = Reflect.getMetadata('controller', token);
                    const isService = Reflect.getMetadata('service', token);
                    const isMiddleware = Reflect.getMetadata('middleware', token);
                    if (isController) {
                        const path = Reflect.getMetadata('controller:path', token) || '/';
                        const routes = Reflect.getMetadata('routes', token) || [];
                        this.controllers.set(token, { path, routes });
                    }
                    else if (isService) {
                        this.services.add(token);
                    }
                    else if (isMiddleware) {
                        this.middleware.push(token);
                    }
                }
            }
        }
    }
    listen(port, host) {
        this.port = port || this.port;
        this.host = host || this.host;
        return new Promise((resolve) => {
            console.log(`Structa app listening on http://${this.host}:${this.port}`);
            resolve();
        });
    }
    getControllers() {
        return this.controllers;
    }
    getServices() {
        return this.services;
    }
}
let appInstance = null;
export function createApp() {
    appInstance = new StructaAppImpl();
    return appInstance;
}
export class StructaApp {
    impl;
    constructor() {
        this.impl = createApp();
    }
    listen(port, host) {
        return this.impl.listen(port, host);
    }
    getControllers() {
        return this.impl.getControllers();
    }
    getServices() {
        return this.impl.getServices();
    }
}
//# sourceMappingURL=app.js.map