jest.mock('alfy');

const { altCaseWord, altCaseStr } = require('../index.js');

describe('altCaseWord', () => {
  test('converts single word to alternating case', () => {
    expect(altCaseWord('hello')).toBe('hElLo');
    expect(altCaseWord('WORLD')).toBe('wOrLd');
    expect(altCaseWord('MiXeD')).toBe('mIxEd');
  });

  test('handles empty string', () => {
    expect(altCaseWord('')).toBe('');
  });

  test('handles single character', () => {
    expect(altCaseWord('a')).toBe('a');
    expect(altCaseWord('A')).toBe('a');
  });

  test('preserves non-alphabetic characters', () => {
    expect(altCaseWord('hello123')).toBe('hElLo123');
    expect(altCaseWord('!@#$%')).toBe('!@#$%');
  });
});

describe('altCaseStr', () => {
  test('converts multiple words to alternating case', () => {
    expect(altCaseStr('hello world')).toBe('hElLo wOrLd');
    expect(altCaseStr('THIS IS A TEST')).toBe('tHiS iS a tEsT');
  });

  test('handles empty string', () => {
    expect(altCaseStr('')).toBe('');
  });

  test('handles single word', () => {
    expect(altCaseStr('hello')).toBe('hElLo');
  });

  test('handles multiple spaces', () => {
    expect(altCaseStr('hello   world')).toBe('hElLo   wOrLd');
  });

  test('preserves punctuation and special characters', () => {
    expect(altCaseStr('hello, world!')).toBe('hElLo, wOrLd!');
    expect(altCaseStr('test@example.com')).toBe('tEsT@ExAmPlE.CoM');
  });
}); 