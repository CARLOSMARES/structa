export interface Provider<T = any> {
    provide: any;
    useClass?: new (...args: any[]) => T;
    useValue?: T;
    useFactory?: () => T;
}
export interface TokenMetadata {
    id: string;
}
export interface Container {
    register<T>(token: any, provider: Provider<T> | T): this;
    resolve<T>(token: any): T;
    get<T>(token: any): T | undefined;
    has(token: any): boolean;
}
declare class StructaContainer implements Container {
    private providers;
    private instances;
    register<T>(token: any, provider: Provider<T> | T): this;
    resolve<T>(token: any): T;
    get<T>(token: any): T | undefined;
    has(token: any): boolean;
}
export declare const containerInstance: StructaContainer;
export declare function createContainer(): Container;
export declare function getContainer(): Container;
export declare function register(token: any, provider: any): void;
export declare function resolve<T>(token: any): T;
export declare function injectable(token?: any): ClassDecorator;
export {};
//# sourceMappingURL=container.d.ts.map