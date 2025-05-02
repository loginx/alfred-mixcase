import alfy from 'alfy';

function altCaseWord(subject) {
  let out = "";
  Array.from(subject).forEach((letter, idx) => {
    out += idx % 2 ? letter.toUpperCase() : letter.toLowerCase();
  });
  return out;
}

function altCaseStr(subject) {
  if (subject === undefined) {
    throw new Error('Input is required');
  }
  return subject.split(" ").map(altCaseWord).join(" ");
}

// Only run Alfred-specific code if this file is being run directly
if (import.meta.url === `file://${process.argv[1]}`) {
  try {
    const result = altCaseStr(alfy.input);
    alfy.output([{
      title: result,
      subtitle: "copy to clipboard",
      arg: result
    }]);
  } catch (error) {
    alfy.output([{
      title: "No input provided",
      subtitle: "Please enter some text to convert to mixed case",
      valid: false,
      icon: {
        path: alfy.icon.info
      }
    }]);
  }
}

export { altCaseWord, altCaseStr };
