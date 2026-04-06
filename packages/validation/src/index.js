export class ValidationError extends Error {
  constructor(
    public property: string,
    public constraints: Record<string, string>,
    public value?: any
  ) {
    super(`Validation failed for property "${property}"`);
    this.name = 'ValidationError';
  }
}

export class ValidationException {
  constructor(public errors: ValidationError[]) {}

  getMessages(): string[] {
    return this.errors.flatMap(e =>
      Object.entries(e.constraints).map(([, message]) => `${e.property}: ${message}`)
    );
  }

  toJSON() {
    return {
      statusCode: 400,
      message: 'Validation failed',
      errors: this.errors.map(e => ({
        property: e.property,
        value: e.value,
        constraints: e.constraints,
      })),
    };
  }
}

export function IsString() {
  return function (target: any, propertyKey: string) {
    addValidationRule(target, propertyKey, {
      name: 'isString',
      validate: (value) => typeof value === 'string',
      message: 'must be a string',
    });
  };
}

export function IsNumber() {
  return function (target: any, propertyKey: string) {
    addValidationRule(target, propertyKey, {
      name: 'isNumber',
      validate: (value) => typeof value === 'number' && !isNaN(value),
      message: 'must be a number',
    });
  };
}

export function IsInt() {
  return function (target: any, propertyKey: string) {
    addValidationRule(target, propertyKey, {
      name: 'isInt',
      validate: (value) => Number.isInteger(value),
      message: 'must be an integer',
    });
  };
}

export function IsBoolean() {
  return function (target: any, propertyKey: string) {
    addValidationRule(target, propertyKey, {
      name: 'isBoolean',
      validate: (value) => typeof value === 'boolean',
      message: 'must be a boolean',
    });
  };
}

export function IsEmail() {
  return function (target: any, propertyKey: string) {
    addValidationRule(target, propertyKey, {
      name: 'isEmail',
      validate: (value) => {
        if (typeof value !== 'string') return false;
        const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
        return emailRegex.test(value);
      },
      message: 'must be a valid email address',
    });
  };
}

export function IsUrl() {
  return function (target: any, propertyKey: string) {
    addValidationRule(target, propertyKey, {
      name: 'isUrl',
      validate: (value) => {
        if (typeof value !== 'string') return false;
        try {
          new URL(value);
          return true;
        } catch {
          return false;
        }
      },
      message: 'must be a valid URL',
    });
  };
}

export function IsOptional() {
  return function (target: any, propertyKey: string) {
    addValidationRule(target, propertyKey, {
      name: 'isOptional',
      validate: (value) => value === undefined || value === null,
      message: 'is optional',
      skipOnNull: true,
    });
  };
}

export function MinLength(min: number) {
  return function (target: any, propertyKey: string) {
    addValidationRule(target, propertyKey, {
      name: 'minLength',
      validate: (value) => {
        if (typeof value !== 'string') return false;
        return value.length >= min;
      },
      message: `must be at least ${min} characters`,
    });
  };
}

export function MaxLength(max: number) {
  return function (target: any, propertyKey: string) {
    addValidationRule(target, propertyKey, {
      name: 'maxLength',
      validate: (value) => {
        if (typeof value !== 'string') return false;
        return value.length <= max;
      },
      message: `must be at most ${max} characters`,
    });
  };
}

export function Min(min: number) {
  return function (target: any, propertyKey: string) {
    addValidationRule(target, propertyKey, {
      name: 'min',
      validate: (value) => {
        if (typeof value !== 'number') return false;
        return value >= min;
      },
      message: `must be at least ${min}`,
    });
  };
}

export function Max(max: number) {
  return function (target: any, propertyKey: string) {
    addValidationRule(target, propertyKey, {
      name: 'max',
      validate: (value) => {
        if (typeof value !== 'number') return false;
        return value <= max;
      },
      message: `must be at most ${max}`,
    });
  };
}

export function IsEnum(enumClass: any) {
  return function (target: any, propertyKey: string) {
    const enumValues = Object.values(enumClass);
    addValidationRule(target, propertyKey, {
      name: 'isEnum',
      validate: (value) => enumValues.includes(value),
      message: `must be one of: ${enumValues.join(', ')}`,
    });
  };
}

export function IsArray() {
  return function (target: any, propertyKey: string) {
    addValidationRule(target, propertyKey, {
      name: 'isArray',
      validate: (value) => Array.isArray(value),
      message: 'must be an array',
    });
  };
}

export function IsDate() {
  return function (target: any, propertyKey: string) {
    addValidationRule(target, propertyKey, {
      name: 'isDate',
      validate: (value) => value instanceof Date || !isNaN(Date.parse(value)),
      message: 'must be a valid date',
    });
  };
}

export function Matches(regex: RegExp, message?: string) {
  return function (target: any, propertyKey: string) {
    addValidationRule(target, propertyKey, {
      name: 'matches',
      validate: (value) => {
        if (typeof value !== 'string') return false;
        return regex.test(value);
      },
      message: message || `must match pattern: ${regex}`,
    });
  };
}

export function IsPhone() {
  return function (target: any, propertyKey: string) {
    addValidationRule(target, propertyKey, {
      name: 'isPhone',
      validate: (value) => {
        if (typeof value !== 'string') return false;
        const phoneRegex = /^[\+]?[(]?[0-9]{1,3}[)]?[-\s\.]?[(]?[0-9]{1,4}[)]?[-\s\.]?[0-9]{1,4}[-\s\.]?[0-9]{1,9}$/;
        return phoneRegex.test(value.replace(/\s/g, ''));
      },
      message: 'must be a valid phone number',
    });
  };
}

export function IsUUID() {
  return function (target: any, propertyKey: string) {
    addValidationRule(target, propertyKey, {
      name: 'isUUID',
      validate: (value) => {
        if (typeof value !== 'string') return false;
        const uuidRegex = /^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i;
        return uuidRegex.test(value);
      },
      message: 'must be a valid UUID',
    });
  };
}

const VALIDATION_RULES_KEY = Symbol('validation_rules');

interface ValidationRule {
  name: string;
  validate: (value: any) => boolean;
  message: string;
  skipOnNull?: boolean;
}

function addValidationRule(target: any, propertyKey: string, rule: ValidationRule) {
  if (!target[VALIDATION_RULES_KEY]) {
    target[VALIDATION_RULES_KEY] = {};
  }
  if (!target[VALIDATION_RULES_KEY][propertyKey]) {
    target[VALIDATION_RULES_KEY][propertyKey] = [];
  }
  target[VALIDATION_RULES_KEY][propertyKey].push(rule);
}

function getValidationRules(target: any, propertyKey: string): ValidationRule[] {
  return target[VALIDATION_RULES_KEY]?.[propertyKey] || [];
}

export class Validator {
  validate<T extends object>(dtoClass: new () => T, data: any): ValidationException | null {
    const errors: ValidationError[] = [];
    const prototype = dtoClass.prototype;
    const rules = prototype[VALIDATION_RULES_KEY] || {};

    for (const [propertyKey, propertyRules] of Object.entries(rules) as [string, ValidationRule[]][]) {
      const value = data?.[propertyKey];

      for (const rule of propertyRules) {
        if (rule.skipOnNull && (value === undefined || value === null)) {
          continue;
        }

        if (!rule.validate(value)) {
          errors.push(
            new ValidationError(propertyKey, { [rule.name]: rule.message }, value)
          );
        }
      }
    }

    return errors.length > 0 ? new ValidationException(errors) : null;
  }

  async validateOrThrow<T extends object>(dtoClass: new () => T, data: any): Promise<T> {
    const result = this.validate(dtoClass, data);
    if (result) {
      throw result;
    }
    return data as T;
  }
}

export const validator = new Validator();

export function validate<T extends object>(dtoClass: new () => T, data: any): ValidationException | null {
  return validator.validate(dtoClass, data);
}

export function validateSync<T extends object>(dtoClass: new () => T, data: any): ValidationException | null {
  return validator.validate(dtoClass, data);
}
