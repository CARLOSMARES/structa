export function Middleware(): ClassDecorator {
  return (target: any) => {
    Reflect.defineMetadata('middleware', true, target);
    return target;
  };
}
