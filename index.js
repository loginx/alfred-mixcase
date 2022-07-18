import alfy from "alfy";

function altCaseWord(subject) {
  let out = "";
  Array.from(subject).forEach((letter, idx) => {
    out += idx % 2 ? letter.toUpperCase() : letter.toLowerCase();
  });

  return out;
}

function altCaseStr(subject) {
  let out = [];
  subject.split(" ").forEach((word) => {
    out.push(altCaseWord(word));
  });
  return out.join(" ");
}

const result = altCaseStr(alfy.input);

const items = [
  {
    title: result,
    subtitle: "copy to clipboard",
    arg: result
  }
];

alfy.output(items);
