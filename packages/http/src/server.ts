export interface HttpServer {
  listen(port: number, host?: string): Promise<void>;
  close(): Promise<void>;
  use(middleware: any): void;
}

export interface StructaHttpServer extends HttpServer {
  applyMiddleware(middlewares: any[]): void;
  setErrorHandler(handler: (error: any, ctx: any) => void): void;
}
