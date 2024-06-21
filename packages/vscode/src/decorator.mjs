// @ts-check
import { window } from "vscode";
import { log } from "./helper.mjs";

/**
 * Updates the decorations in the active text editor to display the CSS rules
 * for class names found in the code.
 *
 * This function finds all occurrences of class names denoted by the `styles.className` in the document,
 * resolving the path to the corresponding CSS file and extracting the CSS rules for the class name,
 * and then decorates the text editor to display the CSS rules for each class name.
 * @typedef {import('vscode').TextEditor} TextEditor
 * @param {TextEditor | undefined} editor
 */
export const updateDecorations = (editor) => {
  log("updateDecorations");
  const text = window.activeTextEditor?.document.getText();

  const document = window.activeTextEditor?.document;
  if (!document) return;

  const cssPath = findCSSPath(document);
  log(`CSS Path: ${cssPath}`);

  // Find all occurrences of `styles.className` in the document
  // TODO: should use AST?
  const regex = /\bstyles\.\w+\b/g;

  /** @type {import('vscode').DecorationOptions[]} */
  const decorations = [];
};

/**
 * ## Find the path to the CSS file that contains the CSS rules for the class name
 * @param {import('vscode').TextDocument} document
 */
const findCSSPath = (document) => {
  const text = document.getText();
  document.fileName;
  const regex = /import\s+['"](.+\.css)['"]/g;
  const match = regex.exec(text);
  if (match) {
    console.log(match);
    return match[1];
  }
  return null;
};
