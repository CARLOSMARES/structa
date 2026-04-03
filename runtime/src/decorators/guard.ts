export function Guard(): ClassDecorator {
  return (target: any) => {
    Reflect.defineMetadata('guard', true, target);
    return target;
  };
}
