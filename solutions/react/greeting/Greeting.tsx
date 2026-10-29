export function Greeting({ name }: { name?: string }) {
  return <p>Hello, {name ?? 'stranger'}!</p>;
}
