/** @type {import('ts-jest').JestConfigWithTsJest} */
module.exports = {
  preset: 'ts-jest',
  testEnvironment: 'node',
  testMatch: ['<rootDir>/test/**/*.(spec|test).ts'],
  setupFilesAfterEnv: ['<rootDir>/test/jest-setup.ts'],
};