export function Dto(): ClassDecorator {
  return (target: any) => {
    Reflect.defineMetadata('dto', true, target);
    return target;
  };
}
