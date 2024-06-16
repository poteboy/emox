// @ts-check
import { window } from "vscode";

const output = window.createOutputChannel("CSS Lens");

/**
 * @param {string} message
 * @returns {void}
 */
export const log = (message) => {
  console.log(message);
  output.appendLine(message);
};
