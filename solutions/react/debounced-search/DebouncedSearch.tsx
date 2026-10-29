import { useEffect, useState } from 'react';

export function DebouncedSearch({ onSearch }: { onSearch: (query: string) => void }) {
  const [query, setQuery] = useState('');

  useEffect(() => {
    const timer = setTimeout(() => {
      onSearch(query);
    }, 300);

    return () => clearTimeout(timer);
  }, [query, onSearch]);

  return (
    <input
      type="text"
      value={query}
      aria-label="Search"
      onChange={(event) => setQuery(event.target.value)}
    />
  );
}
