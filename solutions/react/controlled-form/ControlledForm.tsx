import { useState } from 'react';

export function ControlledForm() {
  const [value, setValue] = useState('');
  const [submitted, setSubmitted] = useState<string | null>(null);

  return (
    <form
      onSubmit={(event) => {
        event.preventDefault();
        setSubmitted(value);
      }}
    >
      <label>
        Name
        <input value={value} onChange={(event) => setValue(event.target.value)} />
      </label>
      <button type="submit">Submit</button>
      {submitted !== null && <p>Submitted: {submitted}</p>}
    </form>
  );
}
