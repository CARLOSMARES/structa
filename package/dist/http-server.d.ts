export interface HttpServerOptions {
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
    instance: any;
}
export declare class StructaHttpServer {
    private server;
    private port;
    private host;
    private controllers;
    constructor();
    private discoverControllers;
    private handleRequest;
    private normalizePath;
    private matchPath;
    private executeHandler;
    listen(port?: number, host?: string): Promise<void>;
    close(): Promise<void>;
}
export declare function createHttpServer(): StructaHttpServer;
export declare function getHttpServer(): StructaHttpServer | null;
//# sourceMappingURL=http-server.d.ts.map