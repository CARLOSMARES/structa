export class TestingModule {
  private providers: Map<any, any> = new Map();
  private controllers: any[] = [];
  private imports: any[] = [];

  static create(options: {
    providers?: any[];
    controllers?: any[];
    imports?: any[];
  }): TestingModule {
    const module = new TestingModule();
    
    if (options.providers) {
      options.providers.forEach((provider: any) => {
        module.providers.set(provider, provider);
      });
    }
    
    if (options.controllers) {
      module.controllers = options.controllers;
    }
    
    if (options.imports) {
      module.imports = options.imports;
    }
    
    return module;
  }

  get<T>(token: any): T {
    const instance = this.providers.get(token);
    if (!instance) {
      throw new Error(`Provider for ${token.name || token} not found in TestingModule`);
    }
    return instance;
  }

  overrideProvider<T>(token: any, value: T): void {
    this.providers.set(token, value);
  }

  compile(): Promise<TestingModule> {
    this.providers.forEach((provider, token) => {
      if (typeof provider === 'function' && !this.isInstantiated(provider)) {
        try {
          const instance = new provider();
          this.providers.set(token, instance);
        } catch (e) {
          console.warn(`Could not instantiate ${token.name || token}`);
        }
      }
    });
    return Promise.resolve(this);
  }

  private isInstantiated(provider: any): boolean {
    const instances = Array.from(this.providers.values());
    return instances.some(instance => instance instanceof provider);
  }
}

export async function createTestingModule(options: {
  providers?: any[];
  controllers?: any[];
  imports?: any[];
}): Promise<TestingModule> {
  const module = TestingModule.create(options);
  return module.compile();
}
