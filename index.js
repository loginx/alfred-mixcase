const alfy = require("alfy");

function altCaseWord(subject) {
  let out = "";
  Array.from(subject).forEach((letter, idx) => {
    out += idx % 2 ? letter.toUpperCase() : letter.toLowerCase();
  });
  return out;
}

function altCaseStr(subject) {
  return subject.split(" ").map(altCaseWord).join(" ");
}

// Only run Alfred-specific code if this file is being run directly
if (require.main === module) {
  const result = altCaseStr(alfy.input);
  const items = [
    {
      title: result,
      subtitle: "copy to clipboard",
      arg: result
    }
  ];
  alfy.output(items);
}

module.exports = { altCaseWord, altCaseStr };
