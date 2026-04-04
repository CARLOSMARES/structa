export class TestRunner {
  private suites: TestSuite[] = [];
  private currentSuite: TestSuite | null = null;
  private beforeAllHooks: Function[] = [];
  private afterAllHooks: Function[] = [];
  private beforeEachHooks: Function[] = [];
  private afterEachHooks: Function[] = [];

  describe(name: string, fn: () => void): void {
    const suite: TestSuite = {
      name,
      tests: [],
      beforeAll: [],
      afterAll: [],
      beforeEach: [],
      afterEach: [],
    };
    
    const parentSuite = this.currentSuite;
    this.currentSuite = suite;
    
    fn();
    
    this.currentSuite = parentSuite;
    this.suites.push(suite);
  }

  it(name: string, fn: () => void | Promise<void>): TestCase {
    if (!this.currentSuite) {
      throw new Error('it() must be called inside describe()');
    }
    
    const test: TestCase = {
      name,
      fn,
      skip: false,
      only: false,
    };
    
    this.currentSuite.tests.push(test);
    return test;
  }

  test = this.it;

  skip(name: string, fn?: () => void): void {
    if (!this.currentSuite) {
      throw new Error('skip() must be called inside describe()');
    }
    
    const test: TestCase = {
      name,
      fn: fn || (() => {}),
      skip: true,
      only: false,
    };
    
    this.currentSuite.tests.push(test);
  }

  async run(): Promise<TestResult> {
    const startTime = Date.now();
    let passed = 0;
    let failed = 0;
    let skipped = 0;
    const failures: TestFailure[] = [];

    for (const suite of this.suites) {
      for (const hook of suite.beforeAll) {
        await hook();
      }

      for (const test of suite.tests) {
        for (const hook of suite.beforeEach) {
          await hook();
        }

        try {
          if (test.skip) {
            skipped++;
          } else {
            await test.fn();
            passed++;
          }
        } catch (error) {
          failed++;
          failures.push({
            suite: suite.name,
            test: test.name,
            error: error as Error,
          });
        }

        for (const hook of suite.afterEach) {
          await hook();
        }
      }

      for (const hook of suite.afterAll) {
        await hook();
      }
    }

    return {
      passed,
      failed,
      skipped,
      total: passed + failed + skipped,
      duration: Date.now() - startTime,
      failures,
    };
  }
}

export interface TestSuite {
  name: string;
  tests: TestCase[];
  beforeAll: Function[];
  afterAll: Function[];
  beforeEach: Function[];
  afterEach: Function[];
}

export interface TestCase {
  name: string;
  fn: () => void | Promise<void>;
  skip: boolean;
  only: boolean;
}

export interface TestResult {
  passed: number;
  failed: number;
  skipped: number;
  total: number;
  duration: number;
  failures: TestFailure[];
}

export interface TestFailure {
  suite: string;
  test: string;
  error: Error;
}

export function Describe(name: string, fn: () => void): void {
  const runner = TestRunnerContext.getRunner();
  runner.describe(name, fn);
}

export function It(name: string, fn: () => void | Promise<void>): void {
  const runner = TestRunnerContext.getRunner();
  runner.it(name, fn);
}

export function Skip(name: string, fn?: () => void): void {
  const runner = TestRunnerContext.getRunner();
  runner.skip(name, fn);
}

export function Expect(actual: any): Expectation {
  return new Expectation(actual);
}

export class Expectation {
  constructor(private actual: any) {}

  toBe(expected: any): void {
    if (this.actual !== expected) {
      throw new Error(`Expected ${expected} but got ${this.actual}`);
    }
  }

  toEqual(expected: any): void {
    if (JSON.stringify(this.actual) !== JSON.stringify(expected)) {
      throw new Error(`Expected ${JSON.stringify(expected)} but got ${JSON.stringify(this.actual)}`);
    }
  }

  toBeTruthy(): void {
    if (!this.actual) {
      throw new Error(`Expected truthy but got ${this.actual}`);
    }
  }

  toBeFalsy(): void {
    if (this.actual) {
      throw new Error(`Expected falsy but got ${this.actual}`);
    }
  }

  toBeNull(): void {
    if (this.actual !== null) {
      throw new Error(`Expected null but got ${this.actual}`);
    }
  }

  toBeUndefined(): void {
    if (this.actual !== undefined) {
      throw new Error(`Expected undefined but got ${this.actual}`);
    }
  }

  toBeDefined(): void {
    if (this.actual === undefined) {
      throw new Error(`Expected defined but got undefined`);
    }
  }

  toContain(item: any): void {
    if (Array.isArray(this.actual)) {
      if (!this.actual.includes(item)) {
        throw new Error(`Expected array to contain ${item}`);
      }
    } else if (typeof this.actual === 'string') {
      if (!this.actual.includes(item)) {
        throw new Error(`Expected string to contain ${item}`);
      }
    }
  }

  toThrow(error?: string | RegExp): void {
    let threw = false;
    let thrownError: any;
    
    try {
      this.actual();
    } catch (e) {
      threw = true;
      thrownError = e;
    }

    if (!threw) {
      throw new Error('Expected function to throw');
    }

    if (error) {
      if (typeof error === 'string') {
        if (thrownError.message !== error) {
          throw new Error(`Expected error message "${error}" but got "${thrownError.message}"`);
        }
      } else if (error instanceof RegExp) {
        if (!error.test(thrownError.message)) {
          throw new Error(`Expected error message to match ${error} but got "${thrownError.message}"`);
        }
      }
    }
  }

  toHaveLength(length: number): void {
    if (!this.actual || this.actual.length === undefined) {
      throw new Error(`Expected value to have length property`);
    }
    if (this.actual.length !== length) {
      throw new Error(`Expected length ${length} but got ${this.actual.length}`);
    }
  }

  toBeGreaterThan(number: number): void {
    if (this.actual <= number) {
      throw new Error(`Expected ${this.actual} to be greater than ${number}`);
    }
  }

  toBeLessThan(number: number): void {
    if (this.actual >= number) {
      throw new Error(`Expected ${this.actual} to be less than ${number}`);
    }
  }

  toMatchObject(obj: any): void {
    const matches = Object.keys(obj).every(key => {
      return this.actual[key] === obj[key];
    });
    if (!matches) {
      throw new Error(`Expected object to match ${JSON.stringify(obj)}`);
    }
  }
}

class TestRunnerContext {
  private static runner = new TestRunner();
  
  static getRunner(): TestRunner {
    return this.runner;
  }
}

export { TestRunnerContext as Context };
