export class Mock<T = any> {
  private mocks: Map<string, any> = new Map();
  private calls: Map<string, any[]> = new Map();

  constructor(private target?: T) {}

  getMock(): T {
    return this.target || ({} as T);
  }

  mock(method: keyof T, implementation: (...args: any[]) => any): void {
    this.mocks.set(method as string, implementation);
  }

  when(method: keyof T, ...args: any[]): MockResponse {
    const key = `${String(method)}(${args.map(a => JSON.stringify(a)).join(', ')})`;
    return new MockResponse(this, method, args);
  }

  getCall(method: keyof T): any[] | undefined {
    return this.calls.get(method as string);
  }

  getCalls(method: keyof T): any[][] {
    return this.calls.get(method as string) || [];
  }

  reset(): void {
    this.mocks.clear();
    this.calls.clear();
  }
}

export class MockResponse {
  constructor(
    private mock: Mock,
    private method: keyof any,
    private args: any[]
  ) {}

  thenImplementation(fn: (...args: any[]) => any): void {
    const key = `${String(this.method)}(${this.args.map(a => JSON.stringify(a)).join(', ')})`;
    (this.mock as any)[key] = fn;
  }

  thenReturn(value: any): void {
    this.thenImplementation(() => value);
  }

  thenResolve(value: any): void {
    this.thenImplementation(() => Promise.resolve(value));
  }

  thenReject(error: Error): void {
    this.thenImplementation(() => Promise.reject(error));
  }
}

export function Spy<T = any>(target: T, method: keyof T): SpyInstance<T> {
  return new SpyInstance(target, method);
}

export class SpyInstance<T = any> {
  private original: any;
  private calls: any[][] = [];

  constructor(private target: T, private method: keyof T) {
    this.original = (target as any)[method];
  }

  mockImplementation(fn: (...args: any[]) => any): void {
    (this.target as any)[this.method] = (...args: any[]) => {
      this.calls.push(args);
      return fn(...args);
    };
  }

  thenReturn(value: any): void {
    this.mockImplementation(() => value);
  }

  thenResolve(value: any): void {
    this.mockImplementation(() => Promise.resolve(value));
  }

  thenReject(error: Error): void {
    this.mockImplementation(() => Promise.reject(error));
  }

  restore(): void {
    (this.target as any)[this.method] = this.original;
  }

  getCalls(): any[][] {
    return this.calls;
  }

  getLastCall(): any[] | undefined {
    return this.calls[this.calls.length - 1];
  }

  getNumberOfCalls(): number {
    return this.calls.length;
  }
}

export function when<T>(mock: Mock<T>, method: keyof T): MockResponse {
  return mock.when(method);
}
