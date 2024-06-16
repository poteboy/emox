import { languages, window, workspace } from "vscode";
import type { DocumentSelector, ExtensionContext } from "vscode";
// import { getCssInfo } from "@css-lens/wasm";
import { updateDecorations } from "./decorator.mjs";
import { provideCodeLenses } from "./provideCodeLenses.mjs";
import { log } from "./helper.mjs";

const CSS_LENS_DOCUMENT_SELECTOR: DocumentSelector = [
  "javascript",
  "typescript",
  "javascriptreact",
  "typescriptreact",
];

export function activate(context: ExtensionContext) {
  log("CSSLens is now active!");
  context.subscriptions.push(
    languages.registerCodeLensProvider(CSS_LENS_DOCUMENT_SELECTOR, {
      provideCodeLenses,
    })
  );

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
