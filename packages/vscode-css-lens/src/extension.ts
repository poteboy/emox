import type { DocumentSelector, ExtensionContext, TextEditor } from "vscode";
import { window, workspace } from "vscode";

const CSS_LENS_DOCUMENT_SELECTOR: DocumentSelector = [
  {
    language: "javascriptreact",
    scheme: "file",
  },
  {
    language: "typescriptreact",
    scheme: "file",
  },
];

export function activate(context: ExtensionContext) {
  window.onDidChangeActiveTextEditor(
    (editor) => {
      if (editor) {
        updateDecorations(editor);
      }
    },
    null,
    context.subscriptions
  );

  workspace.onDidChangeTextDocument(
    (event) => {
      const editor = window.activeTextEditor;
      if (editor && event.document === editor.document) {
        updateDecorations(editor);
      }
    },
    null,
    context.subscriptions
  );
}

/**
 * Updates the decorations in the active text editor to display the CSS rules
 * for class names found in the code.
 *
 * This function finds all occurrences of class names denoted by the `styles.className` in the document,
 * resolving the path to the corresponding CSS file and extracting the CSS rules for the class name,
 * and then decorates the text editor to display the CSS rules for each class name.
 *
 * @todo Implement basic JSX parser to find class names and `import` statements, and resolve the path to the CSS file.
 * @todo Pass the content of the CSS file to the WebAssembly module to extract the CSS rules for the class name.
 *
 * @param {TextEditor} editor
 */
const updateDecorations = (editor: TextEditor) => {
  const text = window.activeTextEditor?.document.getText();
};
