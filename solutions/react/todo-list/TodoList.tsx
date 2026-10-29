export function TodoList({ items }: { items: string[] }) {
  if (items.length === 0) {
    return <p>No items</p>;
  }

  return (
    <ul>
      {items.map((item) => (
        <li key={item}>{item}</li>
      ))}
    </ul>
  );
}
