// @ts-check
import { window } from "vscode";
import { updateDecorations } from "./decorator.mjs";
/**
 * @typedef {import('vscode').TextDocument} TextDocument
 * @typedef {import('vscode').CancellationToken} CancellationToken
 * @typedef {import('vscode').ProviderResult<any>} ProviderResult
 * @param {TextDocument} document
 * @param {CancellationToken} token
 * @returns {ProviderResult}
 */
export function provideCodeLenses(document, token) {
  const editor = window.activeTextEditor;
  updateDecorations(editor);
}
