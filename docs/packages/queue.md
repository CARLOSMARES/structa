# @structa/queue

Job queue system with retry support and multiple drivers.

## Installation

```bash
structa add @structa/queue
```

## Usage

### Basic Usage

```javascript
import { createQueueService } from '@structa/queue';

const queue = createQueueService({ driver: 'memory' });

// Define job handler
await queue.process('send-email', async (job) => {
  console.log(`Sending email to ${job.data.to}`);
  // Email sending logic...
  return { sent: true };
});

// Add job to queue
const job = await queue.add('send-email', {
  to: 'user@example.com',
  subject: 'Hello',
  body: 'World'
});

console.log(`Job ${job.id} added`);
```

## Drivers

### Memory Driver (Default)

```javascript
import { createQueueService } from '@structa/queue';

const queue = createQueueService({
  driver: 'memory'
});
```

### Redis Driver

```javascript
import { createQueueService } from '@structa/queue';

const queue = createQueueService({
  driver: 'redis',
  connectionString: 'redis://localhost:6379'
});
```

## Job Options

```javascript
const job = await queue.add('process-image', data, {
  attempts: 3,           // Max retry attempts
  backoff: {
    type: 'exponential',   // 'fixed' | 'exponential'
    delay: 1000           // Initial delay in ms
  },
  delay: 5000,            // Delay before processing (ms)
  removeOnComplete: true, // Remove after completion
  removeOnFail: false    // Keep failed jobs
});
```

## Job Progress

```javascript
await queue.process('import-data', async (job) => {
  const items = job.data.items;
  
  for (let i = 0; i < items.length; i++) {
    await processItem(items[i]);
    await job.progress((i / items.length) * 100);
  }
  
  return { processed: items.length };
});
```

## Retry & Backoff

### Exponential Backoff

```javascript
await queue.add('api-call', data, {
  attempts: 5,
  backoff: {
    type: 'exponential',
    delay: 1000  // Will retry at 1s, 2s, 4s, 8s...
  }
});
```

### Fixed Backoff

```javascript
await queue.add('api-call', data, {
  attempts: 3,
  backoff: {
    type: 'fixed',
    delay: 5000  // Will retry every 5 seconds
  }
});
```

## Full Example

```javascript
import { createQueueService } from '@structa/queue';

const queue = createQueueService({ driver: 'memory' });

// Process welcome emails
await queue.process('welcome-email', async (job) => {
  const { userId, email, name } = job.data;
  
  await sendEmail({
    to: email,
    subject: `Welcome ${name}!`,
    template: 'welcome',
    variables: { name }
  });
  
  await updateUser(userId, { welcomeSent: true });
  
  return { success: true };
});

// Process password reset
await queue.process('password-reset', async (job) => {
  const { userId, email, token } = job.data;
  
  await sendEmail({
    to: email,
    subject: 'Password Reset',
    template: 'password-reset',
    variables: {
      resetUrl: `https://example.com/reset?token=${token}`
    }
  });
  
  return { sent: true };
});

// Add jobs
await queue.add('welcome-email', {
  userId: 1,
  email: 'john@example.com',
  name: 'John'
});

await queue.add('password-reset', {
  userId: 1,
  email: 'john@example.com',
  token: 'abc123'
}, {
  delay: 60000  // Process after 1 minute
});
```

## Queue API

```javascript
const queue = createQueueService({ driver: 'memory' });

// Add job
const job = await queue.add(name, data, options);

// Process jobs
await queue.process(name, handler);

// Get job
const job = await queue.getJob(jobId);

// Get all jobs (optionally by status)
const jobs = await queue.getJobs();
const pending = await queue.getJobs('waiting');
const failed = await queue.getJobs('failed');

// Pause/Resume
await queue.pause();
await queue.resume();

// Clear all jobs
await queue.clear();
```

## Job Status

| Status | Description |
|--------|-------------|
| `waiting` | Job is queued, waiting to be processed |
| `active` | Job is currently being processed |
| `completed` | Job finished successfully |
| `failed` | Job failed after all retry attempts |
| `delayed` | Job is waiting for delay to expire |

## Use Cases

### Email Queue

```javascript
// Queue all emails
await queue.add('send-email', {
  to: 'user@example.com',
  subject: 'Order Confirmation',
  template: 'order-confirmation',
  data: { orderId: '12345' }
});

// Process with retry
await queue.process('send-email', async (job) => {
  await emailService.send(job.data);
});
```

### Image Processing

```javascript
await queue.process('process-image', async (job) => {
  const { imageId, operations } = job.data;
  
  for (const op of operations) {
    await processOperation(imageId, op);
    await job.progress(operations.indexOf(op) / operations.length * 100);
  }
});
```

### Scheduled Tasks

```javascript
// Schedule for specific time
const delay = targetDate - Date.now();
await queue.add('generate-report', { reportId }, { delay });
```

## Integration with HTTP

```javascript
import { createApp, Controller, Post } from '@structa/http';
import { createQueueService } from '@structa/queue';

const queue = createQueueService({ driver: 'memory' });

@Controller()
class EmailController {
  @Post('/contact')
  async submitContactForm(@Body() data) {
    // Queue email sending
    await queue.add('contact-email', data, {
      delay: 5000  // Process after 5 seconds
    });
    
    return { success: true, message: 'Email queued' };
  }
}
```
