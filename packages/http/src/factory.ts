import { HttpModuleOptions } from './index';

export function createHttpServer(options?: HttpModuleOptions): any {
  return {
    port: options?.port || 3000,
    host: options?.host || '0.0.0.0',
    cors: options?.cors || false,
    bodyParser: options?.bodyParser !== false,
    compression: options?.compression || false,
    
    async listen(port?: number, host?: string): Promise<any> {
      const actualPort = port || this.port;
      const actualHost = host || this.host;
      console.log(`Server listening on http://${actualHost}:${actualPort}`);
      return { port: actualPort, host: actualHost };
    },
    
    async close(): Promise<void> {
      console.log('Server closed');
    },
    
    use(middleware: any): void {
      console.log('Middleware registered:', middleware.name || 'anonymous');
    },
    
    setErrorHandler(handler: (error: any, ctx: any) => void): void {
      console.log('Error handler set');
    }
  };
}
