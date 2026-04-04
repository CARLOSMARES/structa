import 'reflect-metadata';
export interface StructaOptions {
    port?: number;
    host?: string;
}
export interface RouteHandler {
    method: string;
    path: string;
    handler: string;
    target: any;
}
export interface ControllerMetadata {
    path: string;
    routes: RouteHandler[];
}
declare class StructaAppImpl {
    private controllers;
    private services;
    private middleware;
    private port;
    private host;
    constructor();
    private discoverComponents;
    listen(port?: number, host?: string): Promise<void>;
    getControllers(): Map<any, ControllerMetadata>;
    getServices(): Set<any>;
}
export declare function createApp(): StructaAppImpl;
export declare class StructaApp {
    private impl;
    constructor();
    listen(port?: number, host?: string): Promise<void>;
    getControllers(): Map<any, ControllerMetadata>;
    getServices(): Set<any>;
}
export {};
//# sourceMappingURL=app.d.ts.map