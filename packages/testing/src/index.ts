import 'reflect-metadata';

export { TestingModule } from './testing-module';
export { Test } from './decorators/test';
export { BeforeEach, AfterEach, BeforeAll, AfterAll } from './decorators/lifecycle';
export { Describe, It, Expect, TestRunner } from './test-runner';
export { Mock, Spy, when } from './mocks';
export { createMock, mockProvider } from './mock-factory';

export interface TestingModuleOptions {
  providers?: any[];
  imports?: any[];
  controllers?: any[];
}

export interface MockOptions {
  provide: any;
  useValue?: any;
  useFactory?: Function;
  useClass?: any;
}
