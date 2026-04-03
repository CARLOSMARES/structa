export interface Provider<T = any> {
  provide: any;
  useClass?: new (...args: any[]) => T;
  useValue?: T;
  useFactory?: () => T;
}

export interface TokenMetadata {
  id: string;
}

export interface Container {
  register<T>(token: any, provider: Provider<T> | T): this;
  resolve<T>(token: any): T;
  get<T>(token: any): T | undefined;
  has(token: any): boolean;
}

const globalContainer = new Map<any, Provider>();

class StructaContainer implements Container {
  private providers = new Map<any, Provider>();
  private instances = new Map<any, any>();

  register<T>(token: any, provider: Provider<T> | T): this {
    if (typeof provider === 'function') {
      this.providers.set(token, { provide: token, useClass: provider as new (...args: any[]) => any });
    } else if (typeof provider === 'object' && provider !== null) {
      this.providers.set(token, provider as Provider<T>);
    } else {
      this.providers.set(token, { provide: token, useValue: provider });
    }
    return this;
  }

  resolve<T>(token: any): T {
    const provider = this.providers.get(token);
    if (!provider) {
      throw new Error(`No provider found for token: ${token}`);
    }

    if (provider.useValue !== undefined) {
      return provider.useValue;
    }

    if (provider.useFactory) {
      return provider.useFactory();
    }

    if (provider.useClass) {
      const cached = this.instances.get(token);
      if (cached) return cached as T;

      const InstanceClass = provider.useClass as new (...args: any[]) => any;
      const instance = new InstanceClass();
      this.instances.set(token, instance);
      return instance as T;
    }

    throw new Error(`Invalid provider configuration for token: ${token}`);
  }

  get<T>(token: any): T | undefined {
    try {
      return this.resolve<T>(token);
    } catch {
      return undefined;
    }
  }

  has(token: any): boolean {
    return this.providers.has(token);
  }
}

const containerInstance = new StructaContainer();

export function createContainer(): Container {
  return new StructaContainer();
}

export function getContainer(): Container {
  return containerInstance;
}

export function register(token: any, provider: any): void {
  containerInstance.register(token, provider);
}

export function resolve<T>(token: any): T {
  return containerInstance.resolve<T>(token);
}

export function injectable(token?: any): ClassDecorator {
  return (target: any) => {
    if (token) {
      containerInstance.register(token, target);
    } else {
      containerInstance.register(target, target);
    }
    return target;
  };
}
