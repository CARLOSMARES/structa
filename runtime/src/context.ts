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

class StructaResponse implements Response {
  statusCode: number = 200;
  body?: any;
  headers: Record<string, string> = {};

  json(data: any): this {
    this.body = data;
    this.setHeader('Content-Type', 'application/json');
    return this;
  }

  send(data: any): this {
    this.body = data;
    return this;
  }

  status(code: number): this {
    this.statusCode = code;
    return this;
  }

  setHeader(name: string, value: string): this {
    this.headers[name] = value;
    return this;
  }
}

class StructaContext implements Context {
  request: Request;
  response: Response;
  next: NextFunction;
  params: Record<string, string> = {};
  query: Record<string, string> = {};

  constructor(request: Request, response: Response, next: NextFunction) {
    this.request = request;
    this.response = response;
    this.next = next;
  }

  get body(): any {
    return this.request.body;
  }

  status(code: number): this {
    this.response.status(code);
    return this;
  }

  json(data: any): this {
    this.response.json(data);
    return this;
  }

  send(data: any): this {
    this.response.send(data);
    return this;
  }
}

export function createContext(request: Request, next: NextFunction): Context {
  return new StructaContext(request, new StructaResponse(), next);
}

export { Response as HttpResponse };
