export interface MailOptions {
  from?: string;
  to: string | string[];
  cc?: string | string[];
  bcc?: string | string[];
  subject: string;
  text?: string;
  html?: string;
  attachments?: Attachment[];
  headers?: Record<string, string>;
}

export interface Attachment {
  filename: string;
  content?: Buffer | string;
  path?: string;
  contentType?: string;
  encoding?: string;
  href?: string;
  contentId?: string;
}

export interface TemplateOptions {
  name: string;
  variables?: Record<string, any>;
  templateDir?: string;
}

export interface MailConfig {
  driver: 'smtp' | 'sendgrid' | 'mailgun' | 'ses' | 'ethereal';
  host?: string;
  port?: number;
  secure?: boolean;
  auth?: {
    user: string;
    pass: string;
  };
  apiKey?: string;
  from?: string;
  fromName?: string;
}

export interface MailDriver {
  send(options: MailOptions): Promise<SentMessage>;
  sendTemplate(template: TemplateOptions, options: Omit<MailOptions, 'subject' | 'text' | 'html'>): Promise<SentMessage>;
}

export class SentMessage {
  messageId: string;
  accepted: string[] = [];
  rejected: string[] = [];
  response?: string;

  constructor(messageId: string) {
    this.messageId = messageId;
  }
}

export class MailService {
  constructor(private driver: MailDriver, private defaultFrom?: string) {}

  async send(options: MailOptions): Promise<SentMessage> {
    const mergedOptions = {
      ...options,
      from: options.from || this.defaultFrom,
    };
    return this.driver.send(mergedOptions);
  }

  async sendTemplate(template: TemplateOptions, options: Omit<MailOptions, 'subject' | 'text' | 'html'>): Promise<SentMessage> {
    return this.driver.sendTemplate(template, {
      ...options,
      from: options.from || this.defaultFrom,
    });
  }

  async sendWelcome(to: string, name: string): Promise<SentMessage> {
    return this.sendTemplate(
      { name: 'welcome', variables: { name } },
      { to, subject: 'Welcome!' }
    );
  }

  async sendPasswordReset(to: string, token: string): Promise<SentMessage> {
    return this.sendTemplate(
      { name: 'password-reset', variables: { token, resetUrl: `https://example.com/reset?token=${token}` } },
      { to, subject: 'Password Reset Request' }
    );
  }

  async sendEmailVerification(to: string, token: string): Promise<SentMessage> {
    return this.sendTemplate(
      { name: 'email-verification', variables: { token, verifyUrl: `https://example.com/verify?token=${token}` } },
      { to, subject: 'Verify Your Email' }
    );
  }
}

export class SmtpDriver implements MailDriver {
  private transporter: any = null;

  constructor(private config: MailConfig) {}

  private async getTransporter() {
    if (!this.transporter) {
      const nodemailer = await import('nodemailer');
      this.transporter = nodemailer.createTransport({
        host: this.config.host,
        port: this.config.port || 587,
        secure: this.config.secure ?? false,
        auth: this.config.auth,
      });
    }
    return this.transporter;
  }

  async send(options: MailOptions): Promise<SentMessage> {
    const transporter = await this.getTransporter();

    const mailOptions = {
      from: options.from || this.config.from,
      to: options.to,
      cc: options.cc,
      bcc: options.bcc,
      subject: options.subject,
      text: options.text,
      html: options.html,
      attachments: options.attachments,
      headers: options.headers,
    };

    const info = await transporter.sendMail(mailOptions);
    const sent = new SentMessage(info.messageId);
    sent.accepted = info.accepted;
    sent.rejected = info.rejected;
    sent.response = info.response;
    return sent;
  }

  async sendTemplate(template: TemplateOptions, options: Omit<MailOptions, 'subject' | 'text' | 'html'>): Promise<SentMessage> {
    const { subject, text, html } = await renderTemplate(template);
    return this.send({ ...options, subject, text, html });
  }
}

export class SendgridDriver implements MailDriver {
  constructor(private apiKey: string, private defaultFrom?: string) {}

  async send(options: MailOptions): Promise<SentMessage> {
    const sgMail = await import('@sendgrid/mail');
    sgMail.setApiKey(this.apiKey);

    const msg = {
      to: options.to,
      from: options.from || this.defaultFrom,
      cc: options.cc,
      bcc: options.bcc,
      subject: options.subject,
      text: options.text,
      html: options.html,
    };

    const [response] = await sgMail.send(msg);
    const sent = new SentMessage(response.headers['x-message-id']);
    sent.accepted = [options.to as string].flat();
    return sent;
  }

  async sendTemplate(template: TemplateOptions, options: Omit<MailOptions, 'subject' | 'text' | 'html'>): Promise<SentMessage> {
    const { subject, html } = await renderTemplate(template);
    return this.send({ ...options, subject, html });
  }
}

export class EtherealDriver implements MailDriver {
  private account?: any;
  private transporter?: any;

  async send(options: MailOptions): Promise<SentMessage> {
    if (!this.transporter) {
      const nodemailer = await import('nodemailer');
      const testAccount = await nodemailer.createTestAccount();
      this.account = testAccount;
      this.transporter = nodemailer.createTransport({
        host: 'smtp.ethereal.email',
        port: 587,
        secure: false,
        auth: {
          user: testAccount.user,
          pass: testAccount.pass,
        },
      });
    }

    const info = await this.transporter.sendMail({
      from: options.from || this.account.user,
      to: options.to,
      cc: options.cc,
      bcc: options.bcc,
      subject: options.subject,
      text: options.text,
      html: options.html,
      attachments: options.attachments,
    });

    console.log('Preview URL: %s', nodemailer.getTestMessageUrl(info));

    const sent = new SentMessage(info.messageId);
    sent.accepted = info.accepted;
    sent.rejected = info.rejected;
    return sent;
  }

  async sendTemplate(template: TemplateOptions, options: Omit<MailOptions, 'subject' | 'text' | 'html'>): Promise<SentMessage> {
    const { subject, text, html } = await renderTemplate(template);
    return this.send({ ...options, subject, text, html });
  }
}

const templates = new Map<string, string>();

templates.set('welcome', `
<!DOCTYPE html>
<html>
<head><meta charset="utf-8"></head>
<body>
  <h1>Welcome, {{name}}!</h1>
  <p>Thank you for joining us.</p>
</body>
</html>
`);

templates.set('password-reset', `
<!DOCTYPE html>
<html>
<head><meta charset="utf-8"></head>
<body>
  <h1>Password Reset</h1>
  <p>Click the link below to reset your password:</p>
  <a href="{{resetUrl}}">Reset Password</a>
  <p>This link expires in 1 hour.</p>
</body>
</html>
`);

templates.set('email-verification', `
<!DOCTYPE html>
<html>
<head><meta charset="utf-8"></head>
<body>
  <h1>Verify Your Email</h1>
  <p>Click the link below to verify your email address:</p>
  <a href="{{verifyUrl}}">Verify Email</a>
  <p>This link expires in 24 hours.</p>
</body>
</html>
`);

export async function renderTemplate(template: TemplateOptions): Promise<{ subject: string; text: string; html: string }> {
  let templateStr = templates.get(template.name);

  if (!templateStr) {
    throw new Error(`Template "${template.name}" not found`);
  }

  const variables = template.variables || {};

  for (const [key, value] of Object.entries(variables)) {
    const regex = new RegExp(`\\{\\{${key}\\}\\}`, 'g');
    templateStr = templateStr.replace(regex, String(value));
  }

  const subjectMatch = templateStr.match(/<title>(.*?)<\/title>/i);
  const subject = subjectMatch ? subjectMatch[1] : template.name;

  return {
    subject,
    text: templateStr.replace(/<[^>]*>/g, ''),
    html: templateStr,
  };
}

export function registerTemplate(name: string, html: string) {
  templates.set(name, html);
}

export function createMailService(config: MailConfig): MailService {
  let driver: MailDriver;

  switch (config.driver) {
    case 'smtp':
      driver = new SmtpDriver(config);
      break;
    case 'sendgrid':
      if (!config.apiKey) {
        throw new Error('SendGrid API key is required');
      }
      driver = new SendgridDriver(config.apiKey, config.from);
      break;
    case 'ethereal':
      driver = new EtherealDriver();
      break;
    default:
      driver = new EtherealDriver();
  }

  return new MailService(driver, config.from);
}

export const mailer = new MailService(new EtherealDriver());
