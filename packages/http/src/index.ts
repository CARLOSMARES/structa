export { Body, Query, Param, Headers, Cookies } from './decorators/params';
export { UseMiddleware, UseGuards } from './decorators/common';
export { Cors, RateLimit, Compression } from './decorators/middleware';
export { HttpException, HttpStatus } from './exceptions';
export { HttpServer } from './server';
export { createHttpServer } from './factory';

export interface HttpModuleOptions {
  port?: number;
  host?: string;
  cors?: boolean | CorsOptions;
  bodyParser?: boolean;
  compression?: boolean;
}

export interface CorsOptions {
  origin?: boolean | string | RegExp | (string | RegExp)[];
  methods?: string[];
  credentials?: boolean;
  allowedHeaders?: string[];
  exposedHeaders?: string[];
}

export interface RouteOptions {
  path?: string;
  method?: string;
  middleware?: any[];
  guards?: any[];
  prefix?: string;
}
