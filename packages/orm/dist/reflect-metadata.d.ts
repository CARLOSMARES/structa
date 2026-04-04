import 'reflect-metadata';
declare global {
    namespace Reflect {
        function getMetadata(metadataKey: string, target: any): any;
        function getMetadata(metadataKey: string, target: any, propertyKey: string | symbol): any;
        function defineMetadata(metadataKey: string, metadataValue: any, target: any): void;
        function defineMetadata(metadataKey: string, metadataValue: any, target: any, propertyKey: string | symbol): void;
        function metadata(metadataKey: string, metadataValue: any): MethodDecorator & ClassDecorator;
    }
}
export {};
//# sourceMappingURL=reflect-metadata.d.ts.map