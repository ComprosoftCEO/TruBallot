import { useLayoutEffect } from 'react';
import { clearNestedState } from 'redux/helpers';

export const PLACEHOLDER_TEXT = `Question 1
- Candidate 1
- Candidate 2

Question 2
- Candidate 1
- Candidate 2`;

export const useClearState = () => {
  useLayoutEffect(() => clearNestedState('dashboard'), []);
};

export const parseListString = (inputText: string): [string, string[]][] => {
  const input = inputText.replace(/\s/g, '') ? inputText : PLACEHOLDER_TEXT;
  const lines = input.split(/\r?\n/).map((line) => line.trimLeft().slice(0, 255));

  const items: [string, string[]][] = [];

  // Read items one-by-one and push them into the list
  let currentLine: string | null = null;
  let currentLineQuestions: string[] = [];
  for (const line of lines) {
    // Blank line
    if (line.length === 0) {
      if (currentLine !== null) {
        // Flush the current question
        items.push([currentLine, currentLineQuestions]);
        currentLine = null;
        currentLineQuestions = [];
      }
      continue;
    }

    // Non-blank line
    if (currentLine === null) {
      // First line initializes a new question
      currentLine = line.trim();
    } else {
      // Next lines are candidates, so trim the "- " or "* " from the front of the string
      currentLineQuestions.push(line.replace(/^[-*] /g, '').trim());
    }
  }

  // Flush at the end if we haven't flushed yet
  if (currentLine !== null) {
    items.push([currentLine, currentLineQuestions]);
  }

  return items;
};
