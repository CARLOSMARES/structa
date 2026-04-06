# @structa/mail

Email sending module with support for SMTP, SendGrid, and test driver (Ethereal).

## Installation

```bash
structa add @structa/mail
```

## Usage

### Quick Start

```javascript
import { mailer } from '@structa/mail';

// Send simple email
await mailer.send({
  to: 'user@example.com',
  subject: 'Hello',
  text: 'Hello World!',
  html: '<h1>Hello World!</h1>'
});
```

### Send with Template

```javascript
await mailer.sendTemplate(
  { name: 'welcome', variables: { name: 'John' } },
  { to: 'user@example.com', subject: 'Welcome!' }
);
```

## Configuration

### SMTP

```javascript
import { createMailService } from '@structa/mail';

const mailer = createMailService({
  driver: 'smtp',
  host: 'smtp.example.com',
  port: 587,
  secure: false,
  auth: {
    user: 'your-email@example.com',
    pass: 'your-password'
  },
  from: 'Your App <noreply@example.com>'
});
```

### SendGrid

```javascript
const mailer = createMailService({
  driver: 'sendgrid',
  apiKey: process.env.SENDGRID_API_KEY,
  from: 'Your App <noreply@example.com>'
});
```

### Ethereal (Testing)

```javascript
// Creates test accounts automatically
const mailer = createMailService({
  driver: 'ethereal'
});

// Preview URL is logged to console
// Preview URL: https://ethereal.email/message/...
```

## Built-in Templates

### Welcome Email

```javascript
await mailer.sendWelcome('user@example.com', 'John');
// Variables: name
```

### Password Reset

```javascript
await mailer.sendPasswordReset('user@example.com', 'reset-token-123');
// Variables: token, resetUrl
// resetUrl = https://example.com/reset?token=reset-token-123
```

### Email Verification

```javascript
await mailer.sendEmailVerification('user@example.com', 'verify-token-456');
// Variables: token, verifyUrl
// verifyUrl = https://example.com/verify?token=verify-token-456
```

## Email Options

```javascript
await mailer.send({
  from: 'sender@example.com',        // Override default from
  to: 'user@example.com',             // Required: recipient(s)
  cc: 'cc@example.com',              // Carbon copy
  bcc: 'bcc@example.com',            // Blind carbon copy
  subject: 'Email Subject',           // Required: subject
  text: 'Plain text version',        // Plain text body
  html: '<p>HTML version</p>',       // HTML body
  attachments: [                      // File attachments
    {
      filename: 'document.pdf',
      path: '/path/to/file.pdf'
    },
    {
      filename: 'image.png',
      content: Buffer.from('...'),
      contentType: 'image/png'
    }
  ],
  headers: {                          // Custom headers
    'X-Custom-Header': 'value'
  }
});
```

## Custom Templates

### Register Template

```javascript
import { registerTemplate } from '@structa/mail';

registerTemplate('order-confirmation', `
<!DOCTYPE html>
<html>
<head>
  <title>Order Confirmed</title>
</head>
<body>
  <h1>Thank you for your order!</h1>
  <p>Order ID: {{orderId}}</p>
  <p>Total: ${{total}}</p>
</body>
</html>
`);
```

### Use Template

```javascript
await mailer.sendTemplate(
  { 
    name: 'order-confirmation',
    variables: { 
      orderId: 'ORD-12345',
      total: '99.99'
    }
  },
  { 
    to: 'user@example.com',
    subject: 'Order Confirmation'
  }
);
```

## Full Example

```javascript
import { createMailService, registerTemplate } from '@structa/mail';

// Configure
const mailer = createMailService({
  driver: 'ethereal',  // Use 'smtp' or 'sendgrid' in production
  from: 'My App <noreply@myapp.com>'
});

// Register custom template
registerTemplate('newsletter', `
<!DOCTYPE html>
<html>
<body>
  <h1>{{title}}</h1>
  <p>{{content}}</p>
  <footer>Unsubscribe: {{unsubscribeUrl}}</footer>
</body>
</html>
`);

// Send newsletter
await mailer.sendTemplate(
  { 
    name: 'newsletter',
    variables: {
      title: 'Monthly Update',
      content: 'Here are the latest news...',
      unsubscribeUrl: 'https://myapp.com/unsubscribe?email={{email}}'
    }
  },
  { 
    to: ['user1@example.com', 'user2@example.com'],
    subject: 'Monthly Update - April 2026'
  }
);

// Send bulk with BCC
await mailer.send({
  to: 'newsletter@myapp.com',
  bcc: ['user1@example.com', 'user2@example.com', 'user3@example.com'],
  subject: 'Special Offer',
  html: '<h1>50% Off!</h1><p>Use code SAVE50</p>'
});
```

## Integration with Queue

```javascript
import { createMailService } from '@structa/mail';
import { createQueueService } from '@structa/queue';

const mailer = createMailService({ driver: 'smtp', ... });
const queue = createQueueService({ driver: 'memory' });

// Process email queue
await queue.process('send-email', async (job) => {
  await mailer.send(job.data);
});

// Queue emails for sending
await queue.add('send-email', {
  to: 'user@example.com',
  subject: 'Welcome!',
  html: '<h1>Welcome to our app!</h1>'
});
```

## Mail API

```javascript
const mailer = createMailService(config);

// Send email
const sent = await mailer.send(options);

// Send from template
const sent = await mailer.sendTemplate(template, options);

// Built-in emails
await mailer.sendWelcome(to, name);
await mailer.sendPasswordReset(to, token);
await mailer.sendEmailVerification(to, token);
```

### SentMessage Response

```javascript
const sent = await mailer.send({ to: 'user@example.com', ... });

console.log(sent.messageId);   // Email message ID
console.log(sent.accepted);    // ['user@example.com']
console.log(sent.rejected);    // []
console.log(sent.response);    // Server response
```

## Driver Comparison

| Driver | Use Case | Configuration |
|--------|----------|---------------|
| `smtp` | Production | SMTP server credentials |
| `sendgrid` | Production | SendGrid API key |
| `ethereal` | Development/Testing | No config needed |

## Environment Variables

```bash
# SMTP
SMTP_HOST=smtp.example.com
SMTP_PORT=587
SMTP_USER=your-email
SMTP_PASS=your-password

# SendGrid
SENDGRID_API_KEY=SG.xxxxxxxx

# Common
MAIL_FROM=noreply@example.com
```
