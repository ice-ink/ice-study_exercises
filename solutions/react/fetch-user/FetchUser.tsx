import { useEffect, useState } from 'react';

export function FetchUser() {
  const [status, setStatus] = useState<'loading' | 'success' | 'error'>('loading');
  const [name, setName] = useState('');

  useEffect(() => {
    let cancelled = false;

    fetch('/api/user')
      .then((response) => {
        if (!response.ok) {
          throw new Error('Request failed');
        }
        return response.json();
      })
      .then((data: { name: string }) => {
        if (!cancelled) {
          setName(data.name);
          setStatus('success');
        }
      })
      .catch(() => {
        if (!cancelled) {
          setStatus('error');
        }
      });

    return () => {
      cancelled = true;
    };
  }, []);

  if (status === 'loading') {
    return <p>Loading...</p>;
  }

  if (status === 'error') {
    return <p>Error</p>;
  }

  return <p>User: {name}</p>;
}
