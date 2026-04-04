export interface CorsOptions {
  origin?: boolean | string | RegExp | (string | RegExp)[];
  methods?: string[];
  credentials?: boolean;
  allowedHeaders?: string[];
  exposedHeaders?: string[];
}

export interface RateLimitOptions {
  windowMs?: number;
  max?: number;
}

export function Cors(options?: CorsOptions): ClassDecorator {
  return (target: any) => {
    Reflect.defineMetadata('http:cors', options || true, target);
    return target;
  };
}

export function RateLimit(options?: RateLimitOptions): ClassDecorator {
  return (target: any) => {
    Reflect.defineMetadata('http:rateLimit', options || { max: 100 }, target);
    return target;
  };
}

export function Compression(): ClassDecorator {
  return (target: any) => {
    Reflect.defineMetadata('http:compression', true, target);
    return target;
  };
}
