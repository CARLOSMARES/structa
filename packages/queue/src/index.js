export interface JobData {
  [key: string]: any;
}

export interface JobOptions {
  attempts?: number;
  backoff?: {
    type: 'fixed' | 'exponential';
    delay?: number;
  };
  delay?: number;
  removeOnComplete?: boolean;
  removeOnFail?: boolean;
}

export class Job {
  id?: string;
  name: string;
  data: JobData;
  options: JobOptions;
  attemptsMade: number = 0;
  progress: number = 0;
  status: 'waiting' | 'active' | 'completed' | 'failed' | 'delayed' = 'waiting';
  result?: any;
  error?: Error;
  createdAt: Date = new Date();
  processedAt?: Date;
  finishedAt?: Date;

  constructor(name: string, data: JobData = {}, options: JobOptions = {}) {
    this.name = name;
    this.data = data;
    this.options = {
      attempts: options.attempts ?? 3,
      backoff: options.backoff ?? { type: 'exponential', delay: 1000 },
      delay: options.delay ?? 0,
      removeOnComplete: options.removeOnComplete ?? true,
      removeOnFail: options.removeOnFail ?? false,
    };
  }

  async progress(value: number) {
    this.progress = Math.min(100, Math.max(0, value));
  }
}

export interface QueueDriver {
  add(name: string, data: JobData, options?: JobOptions): Promise<Job>;
  process(name: string, handler: (job: Job) => Promise<any>): Promise<void>;
  getJob(id: string): Promise<Job | null>;
  getJobs(status?: string): Promise<Job[]>;
  pause(): Promise<void>;
  resume(): Promise<void>;
  clear(): Promise<void>;
}

export class QueueService {
  constructor(private driver: QueueDriver) {}

  async add<T extends JobData>(name: string, data: T, options?: JobOptions): Promise<Job> {
    return this.driver.add(name, data, options);
  }

  async process(name: string, handler: (job: Job) => Promise<any>): Promise<void> {
    return this.driver.process(name, handler);
  }

  async getJob(id: string): Promise<Job | null> {
    return this.driver.getJob(id);
  }

  async getJobs(status?: string): Promise<Job[]> {
    return this.driver.getJobs(status);
  }

  async pause(): Promise<void> {
    return this.driver.pause();
  }

  async resume(): Promise<void> {
    return this.driver.resume();
  }

  async clear(): Promise<void> {
    return this.driver.clear();
  }
}

export class MemoryQueueDriver implements QueueDriver {
  private jobs = new Map<string, Job>();
  private handlers = new Map<string, (job: Job) => Promise<any>>();
  private jobIdCounter = 0;
  private isPaused = false;

  async add(name: string, data: JobData, options: JobOptions = {}): Promise<Job> {
    const job = new Job(name, data, options);
    job.id = `job_${++this.jobIdCounter}`;
    this.jobs.set(job.id, job);

    if (options.delay && options.delay > 0) {
      job.status = 'delayed';
      setTimeout(() => this.executeJob(job), options.delay);
    } else {
      this.executeJob(job);
    }

    return job;
  }

  private async executeJob(job: Job) {
    if (this.isPaused) {
      setTimeout(() => this.executeJob(job), 1000);
      return;
    }

    const handler = this.handlers.get(job.name);
    if (!handler) {
      console.warn(`No handler registered for job: ${job.name}`);
      return;
    }

    job.status = 'active';
    job.processedAt = new Date();

    try {
      job.result = await handler(job);
      job.status = 'completed';
      job.finishedAt = new Date();
    } catch (error) {
      job.attemptsMade++;
      job.error = error as Error;

      if (job.attemptsMade < (job.options.attempts ?? 3)) {
        const delay = this.calculateBackoff(job);
        setTimeout(() => this.executeJob(job), delay);
      } else {
        job.status = 'failed';
        job.finishedAt = new Date();
      }
    }

    this.cleanupJob(job);
  }

  private calculateBackoff(job: Job): number {
    const backoff = job.options.backoff;
    if (!backoff) return 1000;

    if (backoff.type === 'fixed') {
      return backoff.delay ?? 1000;
    }

    return (backoff.delay ?? 1000) * Math.pow(2, job.attemptsMade - 1);
  }

  private cleanupJob(job: Job) {
    if (job.status === 'completed' && job.options.removeOnComplete) {
      setTimeout(() => this.jobs.delete(job.id!), 5000);
    } else if (job.status === 'failed' && job.options.removeOnFail) {
      setTimeout(() => this.jobs.delete(job.id!), 5000);
    }
  }

  async process(name: string, handler: (job: Job) => Promise<any>): Promise<void> {
    this.handlers.set(name, handler);
  }

  async getJob(id: string): Promise<Job | null> {
    return this.jobs.get(id) || null;
  }

  async getJobs(status?: string): Promise<Job[]> {
    const jobs = Array.from(this.jobs.values());
    if (status) {
      return jobs.filter(j => j.status === status);
    }
    return jobs;
  }

  async pause(): Promise<void> {
    this.isPaused = true;
  }

  async resume(): Promise<void> {
    this.isPaused = false;
  }

  async clear(): Promise<void> {
    this.jobs.clear();
  }
}

export class RedisQueueDriver implements QueueDriver {
  private client: any = null;
  private processing = new Set<string>();

  constructor(private connectionString: string) {}

  private async getClient() {
    if (!this.client) {
      const { default: Redis } = await import('ioredis');
      this.client = new Redis(this.connectionString);
    }
    return this.client;
  }

  async add(name: string, data: JobData, options: JobOptions = {}): Promise<Job> {
    const client = await this.getClient();
    const job = new Job(name, data, options);
    job.id = `job_${Date.now()}_${Math.random().toString(36).slice(2)}`;

    const jobJson = JSON.stringify(job);
    await client.hset('structa:jobs', job.id, jobJson);
    await client.zadd('structa:queue', Date.now(), job.id);

    return job;
  }

  async process(name: string, handler: (job: Job) => Promise<any>): Promise<void> {
    const client = await this.getClient();

    const processNext = async () => {
      if (this.processing.size >= 5) {
        setTimeout(processNext, 100);
        return;
      }

      const jobId = await client.zpopmin('structa:queue');
      if (!jobId) {
        setTimeout(processNext, 1000);
        return;
      }

      const [, id] = jobId as [any, string];
      const jobJson = await client.hget('structa:jobs', id);

      if (!jobJson) {
        processNext();
        return;
      }

      const job: Job = JSON.parse(jobJson);
      this.processing.add(id);

      try {
        job.result = await handler(job);
        job.status = 'completed';
      } catch (error) {
        job.error = error as Error;
        job.status = 'failed';
      }

      await client.hset('structa:jobs', id, JSON.stringify(job));
      this.processing.delete(id);
      processNext();
    };

    processNext();
  }

  async getJob(id: string): Promise<Job | null> {
    const client = await this.getClient();
    const jobJson = await client.hget('structa:jobs', id);
    return jobJson ? JSON.parse(jobJson) : null;
  }

  async getJobs(status?: string): Promise<Job[]> {
    const client = await this.getClient();
    const jobs = await client.hgetall('structa:jobs');
    const allJobs = Object.values(jobs).map((j: any) => JSON.parse(j as string));
    if (status) {
      return allJobs.filter(j => j.status === status);
    }
    return allJobs;
  }

  async pause(): Promise<void> {
    // In a real implementation, this would pause the worker
  }

  async resume(): Promise<void> {
    // In a real implementation, this would resume the worker
  }

  async clear(): Promise<void> {
    const client = await this.getClient();
    await client.del('structa:jobs');
    await client.del('structa:queue');
  }
}

export interface QueueConfig {
  driver: 'memory' | 'redis';
  connectionString?: string;
}

export function createQueueService(config: QueueConfig): QueueService {
  let driver: QueueDriver;

  switch (config.driver) {
    case 'redis':
      if (!config.connectionString) {
        throw new Error('Redis connection string is required');
      }
      driver = new RedisQueueDriver(config.connectionString);
      break;
    case 'memory':
    default:
      driver = new MemoryQueueDriver();
      break;
  }

  return new QueueService(driver);
}

export function InjectQueue() {
  return function (target: any, propertyKey: string) {
    // Placeholder for DI integration
  };
}

export function Queueable(name?: string) {
  return function (target: any, propertyKey: string, descriptor: PropertyDescriptor) {
    const originalMethod = descriptor.value;
    const queueName = name || target.constructor.name;

    descriptor.value = async function (...args: any[]) {
      const queue: QueueService = (target as any).queue;
      if (!queue) {
        return originalMethod.apply(this, args);
      }

      await queue.add(queueName, { args });
    };

    return descriptor;
  };
}
