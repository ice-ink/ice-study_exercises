import { useState } from 'react';

export function Accordion({ sections }: { sections: { title: string; content: string }[] }) {
  const [openIndex, setOpenIndex] = useState<number | null>(null);

  return (
    <div>
      {sections.map((section, index) => (
        <section key={section.title}>
          <button type="button" onClick={() => setOpenIndex(index)}>
            {section.title}
          </button>
          {openIndex === index && <p>{section.content}</p>}
        </section>
      ))}
    </div>
  );
}
