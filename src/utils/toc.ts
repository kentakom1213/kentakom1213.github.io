export type TocItem = {
  text: string;
  id: string;
};

const createSlugifier = () => {
  const slugCounts = new Map<string, number>();

  return (text: string) => {
    const base = text
      .trim()
      .toLowerCase()
      .replace(/[^\p{L}\p{N}\s-]/gu, "")
      .replace(/\s+/g, "-");

    const count = slugCounts.get(base) ?? 0;
    slugCounts.set(base, count + 1);
    return count === 0 ? base : `${base}-${count}`;
  };
};

export const createTocItems = (markdown: string): TocItem[] => {
  const slugify = createSlugifier();

  return markdown
    .split("\n")
    .map((line) => line.match(/^(#{2})\s+(.+)$/u))
    .filter((match): match is RegExpMatchArray => Boolean(match))
    .map((match) => ({
      text: match[2].trim(),
      id: slugify(match[2]),
    }));
};
