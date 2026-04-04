export function createMock<T>(overrides?: Partial<T>): T {
  const mock = {} as T;
  
  if (overrides) {
    Object.keys(overrides).forEach(key => {
      (mock as any)[key] = (overrides as any)[key];
    });
  }
  
  return mock;
}

export function mockProvider<T>(
  token: new (...args: any[]) => T,
  overrides?: Partial<T>
): { provide: new (...args: any[]) => T; useValue: T } {
  return {
    provide: token,
    useValue: createMock(overrides),
  };
}

export function mockService<T extends object>(
  methods: (keyof T)[]
): T {
  const mock = {} as T;
  
  methods.forEach(method => {
    (mock as any)[method] = () => {};
  });
  
  return mock;
}

export class MockBuilder<T = any> {
  private mocks: Map<string, any> = new Map();

  mock(method: string, implementation: (...args: any[]) => any): this {
    this.mocks.set(method, implementation);
    return this;
  }

  mockResolvedValue(method: string, value: any): this {
    this.mocks.set(method, () => Promise.resolve(value));
    return this;
  }

  mockRejectedValue(method: string, error: Error): this {
    this.mocks.set(method, () => Promise.reject(error));
    return this;
  }

  build(): T {
    const obj = {} as T;
    this.mocks.forEach((fn, method) => {
      (obj as any)[method] = fn;
    });
    return obj;
  }
}

export function builder<T>(): MockBuilder<T> {
  return new MockBuilder<T>();
}
