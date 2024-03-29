module.exports = {
  preset: 'ts-jest',
  testEnvironment: 'node',
  testPathIgnorePatterns: [
    "<rootDir>/lib/",
    "<rootDir>/node_modules/",
  ],
  testMatch: [
    '<rootDir>/src/tests.ts',
  ]
};