# NPM Publishing Configuration for Structa

## Requirements for Publishing @structa/* packages

1. **npm Account**: You need an npm account with access to the `@structa` organization
2. **Login**: Run `npm login` to authenticate
3. **2FA**: Enable 2FA on your npm account (required for publish)

## Publishing Commands

```bash
# Login to npm
npm login

# Build all packages
npm run build

# Publish all packages (one by one)
npm publish --workspace=@structa/http
npm publish --workspace=@structa/orm
npm publish --workspace=@structa/validation
npm publish --workspace=@structa/cache
npm publish --workspace=@structa/queue
npm publish --workspace=@structa/mail
npm publish --workspace=@structa/swagger
npm publish --workspace=@structa/websockets
npm publish --workspace=@structa/graphql
npm publish --workspace=@structa/testing
```

## Or use the publish script

```bash
# From repository root
npm run publish:all
```

## Notes

- All packages are scoped under `@structa/`
- Packages with `dist/` need to be built first (TypeScript)
- Packages with `src/` are JavaScript and can be published directly
- Each package has its own `peerDependencies` that must be satisfied
