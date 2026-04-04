export interface Request {
    method: string;
    url: string;
    headers: Record<string, string>;
    body?: any;
    params?: Record<string, string>;
    query?: Record<string, string>;
}
export interface Response {
    statusCode: number;
    body?: any;
    headers: Record<string, string>;
    json(data: any): this;
    send(data: any): this;
    status(code: number): this;
    setHeader(name: string, value: string): this;
}
export type NextFunction = () => void | Promise<void>;
export interface Context {
    request: Request;
    response: Response;
    next: NextFunction;
    params: Record<string, string>;
    query: Record<string, string>;
    body: any;
    status(code: number): this;
    json(data: any): this;
    send(data: any): this;
}
export declare function createContext(request: Request, next: NextFunction): Context;
export { Response as HttpResponse };
//# sourceMappingURL=context.d.ts.map