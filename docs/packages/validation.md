# @structa/validation

Input validation module with decorators for Structa framework.

## Installation

```bash
structa add @structa/validation
```

## Usage

### Define Validated DTO

```javascript
import { 
  IsString, IsEmail, IsInt, IsBoolean, 
  IsOptional, MinLength, MaxLength, Min, Max,
  validate 
} from '@structa/validation';

class CreateUserDto {
  @IsString()
  @MinLength(3)
  name;

  @IsEmail()
  email;

  @IsInt()
  @Min(18)
  @Max(100)
  age;

  @IsString()
  @IsOptional()
  phone;

  @IsBoolean()
  acceptTerms;
}

// Validate data
const result = validate(CreateUserDto, {
  name: 'John',
  email: 'john@example.com',
  age: 25,
  acceptTerms: true
});

if (result) {
  console.log(result.getMessages());
  // ['name: must be at least 3 characters', ...]
}
```

## Available Validators

### String Validators

```javascript
import {
  IsString,
  IsEmail,
  IsUrl,
  IsPhone,
  IsUUID,
  MinLength,
  MaxLength,
  Matches
} from '@structa/validation';

class Dto {
  @IsString()
  name;

  @IsEmail()
  email;

  @IsUrl()
  website;

  @IsPhone()
  phone;

  @IsUUID()
  uuid;

  @IsString()
  @MinLength(8)
  @MaxLength(50)
  password;

  @IsString()
  @Matches(/^[A-Z]{2}$/, 'Must be 2 uppercase letters')
  countryCode;
}
```

### Number Validators

```javascript
import { IsNumber, IsInt, Min, Max } from '@structa/validation';

class Dto {
  @IsNumber()
  price;

  @IsInt()
  quantity;

  @IsNumber()
  @Min(0)
  @Max(100)
  percentage;
}
```

### Boolean Validators

```javascript
import { IsBoolean } from '@structa/validation';

class Dto {
  @IsBoolean()
  active;
}
```

### Array/Object Validators

```javascript
import { IsArray, IsDate } from '@structa/validation';

class Dto {
  @IsArray()
  tags;

  @IsDate()
  startDate;
}
```

### Enum Validators

```javascript
import { IsEnum } from '@structa/validation';

const UserRole = {
  ADMIN: 'admin',
  USER: 'user',
  GUEST: 'guest'
};

class Dto {
  @IsEnum(UserRole)
  role;
}
```

### Optional Fields

```javascript
import { IsOptional, IsString } from '@structa/validation';

class Dto {
  @IsString()
  @IsOptional()
  nickname;  // Can be undefined or string
}
```

## Custom Validation

### Custom Message

```javascript
import { IsString, MinLength } from '@structa/validation';

class Dto {
  @IsString()
  @MinLength(10, { message: 'Username must be at least 10 characters long' })
  username;
}
```

### Validation Error

```javascript
import { ValidationError, ValidationException } from '@structa/validation';

try {
  const result = validate(CreateUserDto, data);
  if (result) {
    throw new ValidationException(result.errors);
  }
} catch (e) {
  if (e instanceof ValidationException) {
    res.status(400).json(e.toJSON());
  }
}
```

## Full Example with HTTP

```javascript
import { createApp, Controller, Post, Body } from '@structa/http';
import { 
  IsString, IsEmail, IsInt, Min, Max,
  validate, ValidationException 
} from '@structa/validation';

class CreateUserDto {
  @IsString()
  @MinLength(3)
  name;

  @IsEmail()
  email;

  @IsInt()
  @Min(18)
  age;
}

@Controller()
class UserController {
  @Post('/')
  async create(@Body() body) {
    const errors = validate(CreateUserDto, body);
    if (errors) {
      throw new ValidationException(errors.errors);
    }
    // Create user...
    return { id: 1, ...body };
  }
}

const app = createApp({
  controllers: [UserController],
  port: 3000
});

app.listen();
```

## API Reference

### Functions

| Function | Description |
|----------|-------------|
| `validate(dto, data)` | Validate data against DTO class |
| `validateSync(dto, data)` | Synchronous validation |

### Classes

| Class | Description |
|-------|-------------|
| `ValidationError` | Single validation error |
| `ValidationException` | Collection of validation errors |

### Decorators

| Decorator | Description |
|-----------|-------------|
| `@IsString()` | Value must be a string |
| `@IsNumber()` | Value must be a number |
| `@IsInt()` | Value must be an integer |
| `@IsBoolean()` | Value must be a boolean |
| `@IsEmail()` | Value must be a valid email |
| `@IsUrl()` | Value must be a valid URL |
| `@IsPhone()` | Value must be a valid phone number |
| `@IsUUID()` | Value must be a valid UUID |
| `@IsDate()` | Value must be a valid date |
| `@IsArray()` | Value must be an array |
| `@IsEnum(Enum)` | Value must be one of enum values |
| `@IsOptional()` | Value can be undefined/null |
| `@Min(n)` | Number must be >= n |
| `@Max(n)` | Number must be <= n |
| `@MinLength(n)` | String length must be >= n |
| `@MaxLength(n)` | String length must be <= n |
| `@Matches(regex, msg?)` | String must match regex |
